/*
  media-filesystem library
  tridge@samba.org, January 2001
  released under the Gnu GPL v2
*/
#include <string.h>
#include <assert.h>
#include <ctype.h>
#include "mfs.h"
#include "log.h"

static struct mfs_super super;
static struct mfs_zone_map *zones[MAX_ZONES];
static int num_zones;

static int little_endian;
static int total_inodes;
static int verbose = 0;

extern int fs_inconsistent;

char *mfs_dev;

unsigned fsid_hash(unsigned fsid, unsigned size)
{
	return fsid*67289 % size;
}

int inode_count() 
{
	return total_inodes;
}

/* useful debug routine */
void dump_sectors(void *d, int n)
{
	u16 *v = d;
	int i, j;
	char *p;
	n = SECTOR_SIZE*n/2;
	for (i=0;i<n;i+=8) {
		int i0 = i;
		while (i>8 && i < (n-8) && memcmp(&v[i-8], &v[i], 16) == 0) {
			if (i == i0) fprintf(stderr, "*\n");
			i += 8;
		}
		fprintf(stderr, "%07x: %04x %04x %04x %04x %04x %04x %04x %04x   ", 
		       i*2, 
		       ntohs(v[i]), ntohs(v[i+1]), ntohs(v[i+2]), ntohs(v[i+3]),
		       ntohs(v[i+4]), ntohs(v[i+5]), ntohs(v[i+6]), ntohs(v[i+7]));
		p = (char *)&v[i];
		for (j=0;j<16;j++) {
			fprintf(stderr, "%c", isprint(p[j])?p[j]:'.');
		}
		fprintf(stderr, "\n");
	}
}



/* load the mfs super block - assumes MFS_DEVICE is set */
static void load_super(int fix)
{
	char buffer[SECTOR_SIZE];
	char *xlist[16] = {0};		/* really only need two: A and B drive substitutions */
	int nxlist=0;
	char *dev = mfs_dev, *p;
	int i;

	if (io_vserver() != -1) {
		mfs_read_partial(&super, 0, sizeof(super));
	} else {
		int fd;
		if (!mfs_dev) {
			int len;
			mfs_dev = getenv("MFS_DEVICE");
			if (!mfs_dev) {
				mfs_dev = "/dev/hda";
				fprintf(stderr, "Assuming MFS_DEVICE=%s\n", mfs_dev);
			}

			// MFS_DEVICE is a space separated list of devices that subtitute for the TiVo A and B drives.
			// For all present TiVo's, there can be only two drives, but we handle up to 15 here.
			p = mfs_dev;
			i = 0;
			while (*p != 0 && i<(sizeof(xlist)/sizeof(xlist[0]))-1) {
				char *end = strchr(p,' ');
				len = end?end-p:strlen(p);
				xlist[i++] = strndupa(p,len);
				p += len+1;
				if (!end) break;
			}
			nxlist=i;
			
			// Read the superblock from the first device
			if (i >0) {
				dev = xlist[0];
				len = strlen(dev);
				if (len>0 && !isdigit(dev[len-1])) {
					// partition 10 if not specified.
					dev = alloca(len+3);
					strcpy(dev,xlist[0]);
					strcpy(dev+len,"10");
				}
			}
		}
		load_devs( dev, 0, 0,  O_RDONLY|O_LARGEFILE);
		fd = open(dev, O_RDONLY|O_LARGEFILE);
		if (fd < 0) {
			fprintf( stderr, "couldn't open %s", dev );
			perror(" ");
			exit(1);
		}
		read_sectors(fd, buffer, 0, 1);
		memcpy(&super, buffer, sizeof(super));

		if (*(u16 *)&super == 0x1492 || *(u16 *)&super == 0x9214) {
			if (*(u16 *)&super == 0x1492) io_need_bswap(1);
			partition_parse();
			mfs_read_partial(&super, 0, sizeof(super));
		}
		close(fd);
	}

	switch (super.magic) {
	case 0xabbafeed: /* normal tivo access */
		break;
	case 0xbaabedfe:
		io_need_bswap(1);
		break;
	case 0xedfebaab:
		little_endian = 1;
		break;
	case 0xfeedabba:
		little_endian = 1;
		io_need_bswap(1);
		break;
	case 0x37353033:
	case 0x37353134:
	case 0x37353136:
		fs_inconsistent = 1;
		break;
	case 0x35373330:
	case 0x35373431:
	case 0x35373631:
		fs_inconsistent = 1;
		io_need_bswap(1);
		break;
	case 0x33303537:
	case 0x34313537:
	case 0x36313537:
		fs_inconsistent = 1;
		little_endian = 1;
		break;
	case 0x30333735:
	case 0x31343735:
	case 0x31363735:
		fs_inconsistent = 1;
		little_endian = 1;
		io_need_bswap(1);
		break;
	default:
		fprintf(stderr,"Not a TiVo super block! (magic=0x%08x)\n", 
			super.magic);
		exit(1);
	}
	if(fs_inconsistent)
	{
		fprintf(stderr, "Warning: filesystem is inconsistent. Run fsfix and mfscheck ASAP\n");
	}

	// reread superblock with byteswapping, if needed
	if (io_get_need_bswap())
		mfs_read_partial(&super, 0, sizeof(super));

	// fill out the partition table with the list from the superblock
	load_devs(super.devlist, xlist, nxlist, O_RDWR|O_LARGEFILE);

	if (!fix) {
		check_crc((void *)&super, sizeof(super), &super.crc);
	} else if (! replace_crc((void *)&super, sizeof(super), &super.crc)) {
		fprintf( stderr, "Writing back corrected CRC\n");
		mfs_write_partial(&super, 0, sizeof(super));
	}
	byte_swap(&super, "i9 b128 i17");

	if ((super.magic != 0xabbafeed) && 
	    (super.magic != 0x37353033) &&
	    (super.magic != 0x37353134) &&
	    (super.magic != 0x37353136)
	    ) {
		fprintf(stderr,"Failed to byte swap correctly\n");
		exit(1);
	}
	
	if (io_total_size() && io_total_size() != super.total_sectors) {
		fprintf(stderr, "WARNING: total sectors doesn't match (total=%d sb=%d)\n",
		       io_total_size(), super.total_sectors);
	}	
}

/* load the mfs zones - currently we only use the inode
   zone but might as well load the lot */
static void load_zones(void)
{
	u32 next = super.zonemap_ptr;
	u32 map_size = super.zonemap_size;
	total_inodes = 0;

	while (next) {
		zones[num_zones] = (struct mfs_zone_map *)malloc(SECTOR_SIZE*map_size);
		mfs_read_sectors(zones[num_zones], next, map_size);
		check_crc(zones[num_zones], map_size*SECTOR_SIZE, &zones[num_zones]->crc);
		byte_swap(zones[num_zones], "i18");
		if (next != zones[num_zones]->sector) {
			fprintf(stderr,"sector wrong in zone (%d %d)\n",
				next, zones[num_zones]->sector);
			exit(1);
		}
		if (zones[num_zones]->type == ZONE_INODE) {
			total_inodes += zones[num_zones]->zone_size/2;
		}
		next = zones[num_zones]->next_zonemap_ptr;
		map_size = zones[num_zones]->next_zonemap_size;
		num_zones++;
		if (num_zones == MAX_ZONES) {
			fprintf(stderr,"Too many zones\n");
			exit(1);
		}
	}
}

/* turn a hash into a zone sector */
static u32 zone_sector(u32 hash)
{
	int i;
	u32 start = 0;
	for (i=0;i<num_zones;i++) {
		u32 len;
		if (zones[i]->type != ZONE_INODE) continue;
		len = zones[i]->zone_size/2;
		if (hash < start + len) {
			return zones[i]->zone_start + (hash-start)*2;
		}
		start += len;
	}
	fprintf(stderr, "Didn't find hash %x in zones!\n", hash);
	exit(1);
}

/* Check an fsid for validity. */
int mfs_valid_fsid(int fsid)
{
	struct mfs_inode in;
	unsigned hash, hash1;

	hash1 = hash = fsid_hash(fsid, total_inodes);
	do {
		mfs_read_partial(&in, zone_sector(hash), sizeof(in));
		check_crc( &in, sizeof(in), &in.crc );
		byte_swap(&in, "i10 b2 s1 i4");
		hash = (hash+1) % total_inodes;
	} while ((in.flags & MFS_FLAGS_CHAIN) && in.id != fsid && hash != hash1);
	
	return (in.id == fsid);
}

/* load one inode by fsid - optimised to avoid repeats */
void mfs_load_inode(int fsid, struct mfs_inode *inode)
{
	static struct mfs_inode in;
	static u32 last_fsid;
	unsigned hash, hash1;

	if (fsid == last_fsid) {
		*inode = in;
		return;
	}

	hash1 = hash = fsid_hash(fsid, total_inodes);
	do {
		mfs_read_partial(&in, zone_sector(hash), sizeof(in));
		check_crc( &in, sizeof(in), &in.crc );
		byte_swap(&in, "i10 b2 s1 i4");
		if (in.num_runs) {
			// cwingert There is more than 24 runs, so just use the 
			// maximum space available.
			// byte_swap(&in.u.runs[0], "i48");
			byte_swap(&in.u.runs[0], "i112");
		}
		hash = (hash+1) % total_inodes;
	} while ((in.flags & MFS_FLAGS_CHAIN) && in.id != fsid && hash != hash1);

	if (in.id != fsid) {
		fprintf(stderr, "ERROR: Didn't find fsid=%d!\n", fsid);
		exit(1);
	}

	*inode = in;
	last_fsid = fsid;
}

/* store one inode by fsid */
void mfs_store_inode(int fsid, struct mfs_inode *inode)
{
	static struct mfs_inode in, out;
	unsigned hash, hash1;
	u32 sec;

	// Find sector
  	hash1 = hash = fsid_hash(fsid, total_inodes);
	do {    
		sec = zone_sector(hash);
		mfs_read_partial(&in, sec, sizeof(in));
		check_crc( &in, sizeof(in), &in.crc );
		byte_swap(&in, "i10 b2 s1 i4");
		if (in.num_runs) {
			byte_swap(&in.u.runs[0], "i48");
		}
		hash = (hash+1) % total_inodes;
	} while ((in.flags & MFS_FLAGS_CHAIN) && in.id != fsid && hash != hash1);

	if (in.id != fsid) {
		fprintf(stderr, "ERROR: Didn't find fsid=%d!\n", fsid);
		exit(1);
	}

        out = *inode;
	// Undo byteswap and update crc
	if (out.num_runs) {
	  byte_swap(&out.u.runs[0], "i48");
	}
	byte_swap(&out, "i10 b2 s1 i4");
	out.crc = htonl(MFS_CRC_BASE);
	out.crc = htonl(crc32( (unsigned char *)&out, sizeof(out) ));

	mfs_write_partial(&out, sec, sizeof(out));
}

/* must call this before anything else */
void mfs_init_dev_fix(char *dev, int fix)
{
	char *p = (dev != 0) ? dev : getenv("MFS_DEVLIST");
	if (p) add_dev_map(p);
	load_super(fix);
	load_zones();
}
void mfs_init_dev(char *dev) 
{
	mfs_init_dev_fix(dev,0);
}
void mfs_init_fix(int fix)
{
	mfs_init_dev_fix(0,fix);
}
void mfs_init() 
{
	mfs_init_dev_fix(0,0);
}

/* dump some global info on the mfs */
void mfs_info(void)
{
	int i;
	u32 total_size = io_total_size();
	fprintf(stderr, "Super:\n\tstate=%x magic=%x\n\tdevlist=%s\n\tzonemap_ptr=%d total_secs=%d next_fsid=%d\n",
	       super.state, super.magic, super.devlist, super.zonemap_ptr,
	       super.total_sectors, super.next_fsid);
	fprintf(stderr, "\tbackup_zonemap_ptr=%x zonemap_size=%d\n",
	       super.backup_zonemap_ptr, super.zonemap_size);

	io_dev_info();

	for (i=0; i<num_zones; i++) {
		fprintf(stderr, "zone(%d):\n\tsector=%d type=%d start=%d next_zonemap=%d\n\tsize=%d per_chunk=%d limit=%d zone_size=%d\n",
		       i,
		       zones[i]->sector, 
		       zones[i]->type, 
		       zones[i]->zone_start, 
		       zones[i]->next_zonemap_ptr, 
		       zones[i]->zone_size,
		       zones[i]->per_chunk,
		       zones[i]->zone_start+zones[i]->zone_size,
		       zones[i]->zone_size);
		fprintf(stderr, "\tbackup_sector=%x zonemap_size=%d\n\tbackup_next_zonemap=%x next_zonemap_size=%d\n\tbuddy_size=%d\n",
		       zones[i]->backup_sector, zones[i]->zonemap_size, 
		       zones[i]->backup_next_zonemap_ptr, 
		       zones[i]->next_zonemap_size,
		       zones[i]->buddy_size);
		if (total_size &&
		    zones[i]->zone_start+zones[i]->zone_size > total_size) {
			fprintf(stderr, "Warning: zone is out of range\n");
		}
	}
	fprintf(stderr, "total_inodes: %d\n", total_inodes );
	
}

/* dome some details about a fsid */
void mfs_fsid_info(int fsid)
{
	struct mfs_inode inode;
	int i;
	u32 hash = fsid_hash(fsid, zones[0]->zone_size/2);

	mfs_load_inode(fsid, &inode);
	fprintf(stderr, "id=%d type=%d/%s hash=%x sec=%d typexx=%d units=%d size=%d used_units=%d used_size=%d runs=%d\n", 
	       inode.id, inode.type, 
	       mfs_type_string(inode.type),
	       hash, zones[0]->zone_start+hash*2,
	       inode.typexx,
	       inode.units, inode.size, inode.used_units, inode.used_size, 
	       inode.num_runs);
	for (i=0; i<inode.num_runs; i++) {
		fprintf(stderr, "run 0x%08x:0x%x\n", 
		       inode.u.runs[i].start,
		       inode.u.runs[i].len);
	}
#if 0
	fprintf(stderr, "raw inode data:\n");
	for(i = 0 ; i < sizeof(inode) ; ++i)
	{
		fprintf(stderr, "%02x ", ((unsigned char *)&inode)[i]);
		if((i % 16) == 15) fprintf(stderr, "\n");
	}
#endif
	fprintf(stderr, "fsid %d is a total of %lld bytes\n", 
	       fsid, mfs_fsid_size(fsid));
}


/* read count bytes from a mfs file at offset ofs,
   returning the number of bytes read 
   ofs must be on a sector boundary
*/
u32 mfs_fsid_pread(int fsid, void *buf, u64 ofs, u32 count)
{
	struct mfs_inode inode;
	int i, n;
	u32 start;
	u32 ret=0;
	u32 sec = ofs >> SECTOR_SHIFT;
	u64 size;

	mfs_load_inode(fsid, &inode);

	if (inode.num_runs == 0) {
		if (ofs >= inode.size) return 0;
		ret = count;
		if (ret > inode.size-ofs) {
			ret = inode.size-ofs;
		}
		memcpy(buf, inode.u.data, ret);
		return ret;
	}

	size = inode.size;
	if (inode.units == 0x20000) {
		size <<= 17;
	}

	if (ofs > size) return 0;
	if (ofs + count > size) {
		count = size-ofs;
	}

	// mfs_fsid_info(fsid);

	while (count > 0) {
		u32 n2;
		start = 0;
		for (i=0; i<inode.num_runs; i++) {
			if (sec < start + inode.u.runs[i].len) break;
			start += inode.u.runs[i].len;
		}
		if (i == inode.num_runs) return ret;
		n = (count+(SECTOR_SIZE-1))>>SECTOR_SHIFT;
		if (n > inode.u.runs[i].len-(sec-start)) {
			n = inode.u.runs[i].len-(sec-start);
		}
		n2 = n<<SECTOR_SHIFT;
		if (n2 > count) n2 = count;
		mfs_read_partial( buf, inode.u.runs[i].start+(sec-start), n2 );

		buf += n2;
		sec += n;
		count -= n2;
		ret += n2;
	}
	return ret;
}

/* write count bytes from a mfs file at offset ofs,
   returning the number of bytes written
   ofs must be on a sector boundary.  Write size is truncated
   to fit within the existing file, i.e. it can't extend a file.

   Probably far from optimal, but it's fast enough for what I need
   now...
*/
u32 mfs_fsid_pwrite(int fsid, void *buf, u64 ofs, u32 count)
{
	struct mfs_inode inode;
	int i, n;
	u32 start;
	u32 ret=0;
	u32 sec = ofs >> SECTOR_SHIFT;
	u64 size;

	mfs_load_inode(fsid, &inode);

	if (inode.num_runs == 0) {
		if (ofs >= inode.size) return 0;
		ret = count;
		if (ret > inode.size-ofs) {
			ret = inode.size-ofs;
		}
		memcpy(inode.u.data, buf, ret);
		mfs_store_inode(fsid,&inode);
		return ret;
	}

	/* Get the size of the file/stream */
	size = inode.size;
	if (inode.units == 0x20000) {
		size <<= 17;
	}

	/* Can't write past the end of file */
	if (ofs > size) 
	{
		if (verbose)
			fprintf(stderr, "Warning, tried to write at %lld of stream sized %lld\n", ofs, size);
		return 0;
	}

	/* Clip off any extra data sent in */
	if (ofs + count > size) {
		count = size-ofs;
		if (verbose)
			fprintf(stderr, "Warning, data clipped to %d bytes\n", count);
	}


	// mfs_fsid_info(fsid);

	while (count > 0) 
	{
		u32 n2;

		start = 0;

		/* Loop through all the runs in the file/stream */
		for (i=0; i<inode.num_runs; i++) 
		{
			/* Find the correct run for the sector */
			if (sec < start + inode.u.runs[i].len) 
				break;
			start += inode.u.runs[i].len;
		}

		/* We're past the last run, bail */
		if (i == inode.num_runs) 
		{
			if (verbose)
				fprintf(stderr, "Warning: We ran past the end of stream, bailing!\n");
			return ret;
		}

		/* Calc # sectors to write */
		n = (count+(SECTOR_SIZE-1))>>SECTOR_SHIFT;

		/* Maximum of 1 run at a time */
		if (n > inode.u.runs[i].len-(sec-start)) 
			n = inode.u.runs[i].len-(sec-start);

		n2 = n<<SECTOR_SHIFT;
		if (n2 > count) n2 = count;

		if (verbose)
			fprintf(stderr, "Writing %d bytes of data to %d sectors starting at %x\n", n2, n, inode.u.runs[i].start+(sec-start));

		mfs_write_partial( buf, inode.u.runs[i].start+(sec-start), n2 );
		/* Get ready for the next go-round */
		buf += n2;
		sec += n;
		count -= n2;
		ret += n2;
	}
	return ret;
}

/* return the type of a fsid */
int mfs_fsid_type(int fsid)
{
	struct mfs_inode inode;
	mfs_load_inode(fsid, &inode);
	return inode.type;
}

/* return the number of bytes used by a fsid */
u64 mfs_fsid_size(int fsid)
{
	struct mfs_inode inode;
	mfs_load_inode(fsid, &inode);

	// mfs_fsid_info(fsid);
	switch (inode.units) {
	case 0: return inode.size;
	case 0x20000: return ((u64)inode.size) << 17;
	}
	fprintf(stderr, "ERROR: fsid=%d Unknown units %d\n", 
		fsid, inode.units);
	exit(1);
	return inode.size;
}

/* list a mfs directory - make sure you free with mfs_dir_free() */
struct mfs_dirent *mfs_dir(int fsid, u32 *count)
{
	u32 *buf, *p;
	int n=0, i;
	int size = mfs_fsid_size(fsid);
	int dsize, dflags;
	struct mfs_dirent *ret;

	*count = 0;

	if (size < 4) return NULL;

	if (mfs_fsid_type(fsid) != MFS_TYPE_DIR) {
		fprintf(stderr,"fsid %d is not a directory\n", fsid);
		// mfs_fsid_info(fsid);
		return NULL;
	}

	buf = (u32 *)malloc(size);
	mfs_fsid_pread(fsid, buf, 0, size);
	dsize = ntohl(buf[0]) >> 16;
	dflags = ntohl(buf[0]) & 0xFFFF;
	p = buf + 1;
	while ((int)(p-buf) < dsize/4) {
		u8 *s = ((unsigned char *)p)+4;
		p += s[0]/4;
		n++;
	}
	ret = malloc((n+1)*sizeof(*ret));
	p = buf + 1;
	for (i=0;i<n;i++) {
		u8 *s = ((unsigned char *)p)+4;
		ret[i].name = strdup((char *)s+2);
		ret[i].type = s[1];
		ret[i].fsid = ntohl(p[0]);
		p += s[0]/4;
	}	
	ret[n].name = NULL;
	free(buf);
	*count = n;

	/* handle meta-directories. These are just directories which are
	   lists of other directories. All we need to do is recursively read
	   the other directories and piece together the top level directory */
	if (dflags == 0x200) {
		struct mfs_dirent *meta_dir = NULL;
		int meta_size=0;

		*count = 0;

		for (i=0;i<n;i++) {
			struct mfs_dirent *d2;
			unsigned int n2;
			if (ret[i].type != MFS_TYPE_DIR) {
				fprintf(stderr, "ERROR: non dir %d/%s in meta-dir %d!\n", 
				       ret[i].type, ret[i].name, fsid);
				continue;
			}
			d2 = mfs_dir(ret[i].fsid, &n2);
			if (!d2 || n2 == 0) continue;
			meta_dir = realloc(meta_dir, sizeof(ret[0])*(meta_size + n2 + 1));
			memcpy(meta_dir+meta_size, d2, n2*sizeof(ret[0]));
			meta_size += n2;
			free(d2);
		}
		mfs_dir_free(ret);
		if (meta_dir) meta_dir[meta_size].name = NULL;
		*count = meta_size;
		return meta_dir;
	}


	return ret;
}

/* free a dir from mfs_dir */
void mfs_dir_free(struct mfs_dirent *dir)
{
	int i;
	for (i=0; dir[i].name; i++) {
		free(dir[i].name);
		dir[i].name = NULL;
	}
	free(dir);
}

/* return a string identifier for a tivo file type */
char *mfs_type_string(int type)
{
	
	switch (type) {
	case 0: return "NULL";
	case MFS_TYPE_DIR: return "tyDir";
	case MFS_TYPE_OBJ: return "tyDb";
	case MFS_TYPE_STREAM: return "tyStream";
	case MFS_TYPE_FILE: return "tyFile";
	default: 
		fprintf(stderr,"ERROR: Unknown file type %d!\n", type);
		exit(1);
		break;
	}
}

/* resolve a path to a fsid */
u32 mfs_resolve(const char *pathin)
{
	char *path, *tok, *r=NULL;
	u32 fsid;
	struct mfs_dirent *dir = NULL;

	if (pathin[0] != '/') {
		return atoi(pathin);
	}

	fsid = MFS_ROOT_FSID;
	path = strdup(pathin);
	for (tok=strtok_r(path,"/", &r); tok; tok=strtok_r(NULL,"/", &r)) {
		u32 count;
		int i;
		dir = mfs_dir(fsid, &count);
		if (!dir) {
			fprintf(stderr,"resolve failed for fsid=%d\n",
				fsid);
			return 0;
		}
		for (i=0;i<count;i++) {
			if (strcmp(tok, dir[i].name) == 0) break;
		}
		if (i == count) {
			fsid = 0;
			goto done;
		}
		fsid = dir[i].fsid;
		if (dir[i].type != MFS_TYPE_DIR) {
			if (strtok_r(NULL, "/", &r)) {
				fprintf(stderr,"not a directory %s\n",tok);
				fsid = 0;
				goto done;
			}
			goto done;
		}
		mfs_dir_free(dir);
		dir = NULL;
	}

 done:
	if (dir) mfs_dir_free(dir);
	if (path) free(path);
	return fsid;
}


/* loop over all inodes calling fn on each one */
void mfs_all_inodes(void (*fn)(struct mfs_inode *,void *), void *data)
{
	int i, z;
	u32 start_hash=0;
	struct mfs_inode inode;

	for (z=0; z<num_zones;z++) {
		if (zones[z]->type != ZONE_INODE) continue;
		for (i=0; i<zones[z]->zone_size; i+= 2) {
			u32 hash = start_hash + i/2;
			mfs_read_partial(&inode, zone_sector(hash), 
					 sizeof(inode));
			byte_swap(&inode, "i10 b2 s1 i4");
			if (inode.num_runs) {
				byte_swap(&inode.u.runs[0], "i48");
			}
			if (inode.id != 0) {
			  //				if (inode.id > super.next_fsid) {
			  //					fprintf(stderr, "invalid fsid %d (next=%d)\n",
			  //					       inode.id, super.next_fsid);
			  //					exit(1);
			  //				}
				fn(&inode,data);
			}
		}
		start_hash += zones[z]->zone_size/2;
	}
}


struct bitmap_fn_data {
	int zone;
	u64 limit;
	struct bitmap *bm;
};

void bitmap_fn(struct mfs_inode *inode, void *data) 
{
	int i;
	struct bitmap_fn_data *bm_data = (struct bitmap_fn_data *)data;
	struct mfs_zone_map *zone = zones[bm_data->zone];

	fprintf(stderr, "inode %d runs=%d\n", inode->id, inode->num_runs);
	if (bm_data->limit && mfs_fsid_size(inode->id) > bm_data->limit) {
		fprintf(stderr, "skipping inode %d of size=%lld\n", 
			inode->id, mfs_fsid_size(inode->id));
		return;
	}
	//if (bitmap_excluded(inode->id)) return;
	for (i=0;i<inode->num_runs;i++) {
		u32 start, len;
		start = inode->u.runs[i].start;
		len = inode->u.runs[i].len;
		if (start < zone->zone_start ||
		    start >= zone->zone_start+ zone->zone_size) {
			continue;
		}
		if (len % zone->per_chunk) {
			fprintf(stderr, "Not a multiple of per-chunk? fsid=%d\n", inode->id);
			exit(1);
		}
		bitmap_set(bm_data->bm, 
			   (start-zone->zone_start)/
			   zone->per_chunk,
			   len/zone->per_chunk);
	}
}

struct bitmap *mfs_zone_bitmap(int zone, u64 limit)
{
	int num_blocks;
	struct bitmap_fn_data bm_data;

	num_blocks = zones[zone]->zone_size / zones[zone]->per_chunk;
	bm_data.zone = zone;
	bm_data.limit = limit;
	bm_data.bm = bitmap_allocate(num_blocks);
	mfs_all_inodes(bitmap_fn, &bm_data);

	return bm_data.bm;
}


void mfs_purge_zone(int zone, u32 limit)
{
	struct bitmap *bm;
	int i, n;
	int units = zones[zone]->per_chunk;

	bm = mfs_zone_bitmap(zone, limit);
	
	for (i=0;i<bm->n;) {
		if (bitmap_query(bm, i)) {
			i++; continue;
		}
		n = 1;
		while (bitmap_query(bm, i+n) == 0 && 
		       i+n < bm->n) n++;
		mfs_zero_sectors(zones[zone]->zone_start+i*units,
				 n*units);
		i += n;
	}
}

void mfs_purge_all(u64 limit)
{
	int z;

	for (z=0; z<num_zones;z++) {
		if (zones[z]->type != ZONE_STREAM &&
		    zones[z]->type != ZONE_FILE) continue;
		fprintf(stderr, "Purging zone %d with size limit %lld\n", z, limit);
		mfs_purge_zone(z, limit);
	}
}

/* zone size in sectors */
u32 mfs_zone_size(int zone)
{
	return zones[zone]->zone_size;
}
