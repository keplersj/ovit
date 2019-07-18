/*
  media-filesystem library, io routines tridge@samba.org, January 2001
  released under the Gnu GPL v2
*/

/*
   modifications to handle multiple disk drives
   Jonathan Biggar 2004
   See JPB comments
   licensed Gnu GPL vs
*/
#include <stdlib.h>
#include <ctype.h>
#include "mfs.h"
#include "log.h"

#ifdef linux
#include <linux/fs.h>
#else
#ifdef __CYGWIN__
#include <cygwin/fs.h>
#else
#ifdef __MACH__
#include <sys/disk.h>
#endif
#endif
#endif

static int readahead_enabled;
static int vserver = -1;
static int need_bswap;
static int verbose = 0;
static u32 total_size;
static char *dev_list;
extern char *mfs_dev;

void mfs_readahead(int set)
{
	readahead_enabled = set;
}

static void vserver_read_req(u32 sec, u32 count)
{
	struct vserver_cmd cmd;

	cmd.command = htonl(MFS_CMD_READ);
	cmd.param1 = htonl(sec);
	cmd.param2 = htonl(count);
	write_all(vserver, &cmd, sizeof(cmd));
}

static void vserver_list_sectors_req(u32 sec, u32 count)
{
	struct vserver_cmd cmd;

	cmd.command = htonl(MFS_CMD_LIST_SECTORS);
	cmd.param1 = htonl(sec);
	cmd.param2 = htonl(count);
	write_all(vserver, &cmd, sizeof(cmd));
}

static void vserver_receive(void *buf, u32 count)
{
	count <<= SECTOR_SHIFT;
	read_all(vserver, buf, count);
}

#define RA_BLOCKS	256
#define RA_MIN		256

static void vserver_read_sectors(void *buf, u32 sec, u32 count)
{
	static struct mfs_run readahead;
	u32 discard, coming;

	if (count == 0)
		return;

	discard = coming = 0;
	if (sec >= readahead.start && sec < readahead.start + readahead.len) {
		discard = sec - readahead.start;
		coming = readahead.len - discard;
		if (coming <= count) {
			readahead.len = 0;
		} else {
			readahead.start = sec + count;
			readahead.len -= discard + count;
		}
	} else {
		discard = readahead.len;
		coming = 0;
		readahead.len = 0;
	}

	if (coming < count) {
		u32 nreq = count - coming;
		if (readahead_enabled && nreq < RA_BLOCKS) {
			readahead.start = sec + count;
			readahead.len = RA_BLOCKS - nreq;
			nreq = RA_BLOCKS;
		}
		vserver_read_req(sec + coming, nreq);
	}

	if (readahead.len <= RA_MIN && readahead_enabled) {
		if (readahead.len == 0)
			readahead.start = sec + count;
		vserver_read_req(readahead.start + readahead.len, RA_BLOCKS);
		readahead.len += RA_BLOCKS;
	}

	if (discard) {
		void *buf2 = malloc(discard << SECTOR_SHIFT);
		vserver_receive(buf2, discard);
		free(buf2);
	}

	vserver_receive(buf, count);

	if (need_bswap) {
		u16 *v = (u16 *)buf;
		int i;
		for (i=0;i<count<<(SECTOR_SHIFT-1);i++)
			v[i] = ntohs(v[i]);
	}
}

static void vserver_write_sectors(void *buf, u32 sec, u32 count)
{
	struct vserver_cmd cmd;

	if (readahead_enabled) {
		fprintf(stderr, "vserver write not supported with readahead\n");
		exit(1);
	}

	cmd.command = htonl(MFS_CMD_WRITE);
	cmd.param1 = htonl(sec);
	cmd.param2 = htonl(count);
	write_all(vserver, &cmd, sizeof(cmd));
	write_all(vserver, buf, count*SECTOR_SIZE);
}

static run_desc vserver_list_sectors( u32 sec, u32 count)
{
	run_desc retval;
	vserver_list_sectors_req(sec,count);
	read_all( vserver, &retval, sizeof(retval) );
	retval.drive = ntohl(retval.drive);
	retval.partition = ntohl(retval.partition);
	retval.start = ntohl(retval.start);
	retval.count = ntohl(retval.count );
	return retval;
}

static void vserver_zero_sectors(u32 sec, u32 count)
{
	struct vserver_cmd cmd;

	if (readahead_enabled) {
		fprintf(stderr, "vserver write not supported with readahead\n");
		exit(1);
	}

	cmd.command = htonl(MFS_CMD_ZERO);
	cmd.param1 = htonl(sec);
	cmd.param2 = htonl(count);
	write_all(vserver, &cmd, sizeof(cmd));
}

/* this holds the list of block devices in the tivo */
static struct {
	char *dev;
	unsigned long sectors;
	int fd;
} devs[MAX_DEVS] = {{0}};

static unsigned count_devs; /*JPB*/

/* read from the virtual tivo disk (ie. all partitions 
   concatenated) */
void mfs_read_sectors(void *buf, u32 sec, u32 count)
{
	int i;
	u64 start=0;
	
	if (vserver != -1) {
		vserver_read_sectors(buf, sec, count);
		return;
	}

	sec = partition_remap(sec);

	for (i=0; devs[i].dev; i++) {
		if (sec < start + devs[i].sectors) break;
		start += devs[i].sectors;
	}
	if (!devs[i].dev) {
		fprintf(stderr,"Failed to map sector %d\n", sec);
		exit(1);
	}

	if (verbose) {
		fprintf(stderr, "mapped %d to %s/%d\n", sec, devs[i].dev, (int)(sec-start));
	}

	read_sectors(devs[i].fd, buf, sec-start, count);

	if (need_bswap) {
		u16 *v = (u16 *)buf;
		for (i=0;i<count<<(SECTOR_SHIFT-1);i++) v[i] = ntohs(v[i]);
	}
}

/** Assumes the run is entirely on one device.  */
run_desc mfs_list_sectors(u32 sec, u32 count)
{
	int i,k;
	int base=1;
	char a='a';
	char zero='0';
	char nine='9';
	u64 start=0;
	run_desc retval = {0};

	if (vserver != -1) {
		return vserver_list_sectors(sec, count);
	}

	sec = partition_remap(sec);
	for ( i=0; devs[i].dev; i++ )
	{
		if ( sec < start + devs[i].sectors ) break;
		start += devs[i].sectors;
	}
	if ( !devs[i].dev )
	{
		fprintf(stderr,"Failed to map sector %d\n", sec);
		return retval;
	}
	retval.start = sec - start;
	if ( verbose )
	{
		fprintf(stderr, "Mapped %d to %s/%d count(%d)\n", 
			sec, devs[i].dev, (int)(sec-retval.start), count);
	}

	/* parse out the partition number from the dev string...*/
	retval.partition = 0;
	for(k=strlen(devs[i].dev)-1; k>=0; k--)
	{
		if(devs[i].dev[k] >= zero && devs[i].dev[k] <= nine)
		{      
			retval.partition +=
				(( devs[i].dev[k] - zero ) * base);
			base=base*10;
		}
		else
			break;
	}
	/* parse the drive letter from /dev/hdXNN */
	if(k>=0)
	{
		retval.drive=( devs[i].dev[k] - a );
	}
	retval.count = count;
	return retval;
}

/* write to the virtual tivo disk (ie. all partitions 
   concatenated) */
void mfs_write_sectors(void *buf, u32 sec, u32 count)
{
	int i;
	u64 start=0;
	
	if (vserver != -1) {
		vserver_write_sectors(buf, sec, count);
		return;
	}

	sec = partition_remap(sec);

	for (i=0; devs[i].dev; i++) {
		if (sec < start + devs[i].sectors) break;
		start += devs[i].sectors;
	}
	if (!devs[i].dev) {
		fprintf(stderr,"Failed to map sector %d\n", sec);
		exit(1);
	}

	if (verbose) {
		fprintf(stderr, "mapped %d to %s/%d\n", sec, devs[i].dev, (int)(sec-start));
	}

	write_sectors(devs[i].fd, buf, sec-start, count);
}


void mfs_zero_sectors(int sector, int count)
{
	int chunk_size=512;
	char buf[chunk_size*512];
	char buf1[chunk_size*512];
	int total=0;

	if (vserver != -1) {
		vserver_zero_sectors(sector, count);
		return;
	}

	bzero(buf, chunk_size*512);

	fprintf(stderr, "wiping %d sectors at %d\n", count, sector);

	while (total<count) {
		int n = MIN(chunk_size, count);
		mfs_read_sectors(buf1, sector, n);
		if (memcmp(buf, buf1, n*SECTOR_SIZE)) {
			mfs_write_sectors(buf, sector, n);
		}
		sector += n;
		total += n;
		fprintf(stderr, "%3.1f%%\r", 100.0*total/count);
		fflush(stdout);
	}
	fprintf(stderr, "%3.1f%%\n", 100.0*total/count);
}

/* JPB return the number of devices */
u32 num_devs()
{
	return count_devs;
}

/* JPB return the sector offset of the given device */
u32	dev_start_sector(u32 dev_no)
{
	u32 i;
	u32	start = 0;
	
	for (i = 0; i < dev_no; ++i) {
		start += devs[i].sectors;
	}
	return start;
}


static u32 get_blockcount(int fd )
{
	u32 retval;

#ifdef BLKGETSIZE
	ioctl(fd, BLKGETSIZE, &retval);
#else
#if DKIOCGETBLOCKCOUNT32
	ioctl(devs[i].fd, DKIOCGETBLOCKCOUNT32, &retval);
#else
#if DKIOCGETBLOCKCOUNT
	{
		long long nsectors;
		ioctl(devs[i].fd, DKIOCGETBLOCKCOUNT, &nsectors );
		retval = (u32) nsectors;
	}
#else
	error("no ioctl to get block size");
#endif
#endif
#endif
	return retval;
}

/* initialise the devices list from the superblock devlist */
void load_devs(char *devlist, char *xlist[], int nxlist, int mode)
{
	char *p;
	int i=0, xi, len;
	int bsddevname;

	// Close any previously opened devices
	for (i=0; devs[i].dev; i++)
		close(devs[i].fd);
	i=0;

	if (dev_list) {
		devlist = dev_list;
		xlist = 0;
	} else
	  clear_use_ptable();

	if (vserver != -1) return;

	total_size = 0;

	while (1) {
		p = strchr(devlist, ' ');
		len = p ? (p-devlist) : strlen(devlist);

		// translate devices if required
		xi = (devlist[7] - 'a');  // driver letter (a,b),  converted to 0 based index
		if (xi >=0 && xi < nxlist && xlist[xi]) {
			// extract partition number.from devlist and append it to xlist[xi] for our device.
			char *sp=devlist;
			int l = len;
			char *ep = sp+l; /* terminating 0 byte */
			char *p1 = ep-1;

			// get partition digits from devlist
			while( p1 >= sp && isdigit(*p1))
				p1--;
			p1++;

			bsddevname = 
			  strncmp(xlist[xi],"/dev/disk",9)==0 ||
			  strncmp(xlist[xi],"/dev/rdisk",10)==0;

			l=strlen(xlist[xi]);
			if (!bsddevname) {
				// strip partition digits from xlist[xi]
				while(l>0 && isdigit(xlist[xi][l-1]))
					l--;
			}
			// Assemble device string
			devs[i].dev = (char *) malloc( l + (ep-p1) +2 );
			strncpy( devs[i].dev, xlist[xi], l );
			// check for bsd/OS X style disk partition devices
			if (bsddevname)
			  devs[i].dev[l++]='s'; 
			strncpy( devs[i].dev+l, p1, (ep-p1) );
                        devs[i].dev[l+(ep-p1)]= '\0';
		} else {
			devs[i].dev = strndup(devlist,len);
		}

		devs[i].fd = open(devs[i].dev, mode);
		if (devs[i].fd == -1) {
			fprintf(stderr, "failed to open [%s]\n", devs[i].dev);
			perror("perror:");
			break;
		}
		devs[i].sectors = ll_seek(devs[i].fd, 0, SEEK_END) >> SECTOR_SHIFT;
		if (devs[i].sectors == 0) {
			devs[i].sectors = get_blockcount( devs[i].fd );
		}
		devs[i].sectors &= ~(MFS_BLOCK_ROUND-1);
		total_size += devs[i].sectors;
		i++;
		devlist += len+1;
		if (!p) break;
	}	
	count_devs = i; /*JPB*/
}

void add_dev_map(char *mapping)
{
	char *p;

	if (mapping[0] == ':') {
		vserver = open_socket_out(mapping+1, VSERVER_PORT);
		if (vserver == -1) {
			fprintf(stderr,"Failed to connect to %s\n", mapping+1);
			exit(1);
		}
		return;
	}

	dev_list = strdup(mapping);
	mfs_dev = strdup(dev_list);
	if ((p = strchr(mfs_dev, ' '))) *p = 0;
}

void io_dev_info(void)
{
	int i;

	for (i=0; devs[i].dev; i++) {
		fprintf(stderr, "%s has %ld sectors\n", 
		       devs[i].dev,
		       devs[i].sectors);
	}
}

int io_vserver(void)
{
	return vserver;
}

void io_need_bswap(int set)
{
	need_bswap = set;
}

u32 io_total_size(void)
{
	if (partition_total_size()) {
		total_size = partition_total_size();
	}	
	return total_size;
}


/* read bytes from a sector, handling partial sectors */
void mfs_read_partial(void *buf, u32 sec, u64 size)
{
	char tmp[SECTOR_SIZE];
	if (size >= SECTOR_SIZE) {
		u32 n = size>>SECTOR_SHIFT;
		mfs_read_sectors(buf, sec, n);
		sec += n;
		n <<= SECTOR_SHIFT;
		buf += n;
		size -= n;
	}
	if (size == 0) return;
	mfs_read_sectors(tmp, sec, 1);
	memcpy(buf, tmp, size);
}


/* write bytes to a sector, handling partial sectors */
void mfs_write_partial(void *buf, u32 sec, u64 size)
{
	char tmp[SECTOR_SIZE];
	if (size >= SECTOR_SIZE) {
		u32 n = size>>SECTOR_SHIFT;
		mfs_write_sectors(buf, sec, n);
		sec += n;
		n <<= SECTOR_SHIFT;
		buf += n;
		size -= n;
	}
	if (size == 0) return;
	mfs_read_sectors(tmp, sec, 1);
	memcpy(tmp, buf, size);
	mfs_write_sectors(tmp, sec, 1);
}

/*******************************************************************************
    ppchacker 01/18/2002.  Let users know whether byte-swapping is being done.
*******************************************************************************/
int io_get_need_bswap(void)
{
	return need_bswap;
}











