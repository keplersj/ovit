/*
  media-filesystem object dump
  tridge@samba.org, January 2001
  released under the Gnu GPL v2
*/

#include "mfs.h"
#include <time.h>
#include <assert.h>

static int option_human = 0;
struct tm * (*timefn)(const time_t *CLOCK) = gmtime;
int option_local = 0;
static int tzoff = 0;
static int last = 0;

// Forward reference
static void dumpobj(int fsid, int fileobj, int recurse, const char *path);

//
// Queue of referenced fsid's to dump out after the referencing object
//
#define MAX_QUEUE 10240
static int fsid_queue[MAX_QUEUE];
static int nq = 0;

static void queue_add(int fsid) {
	assert(nq < MAX_QUEUE);
	fsid_queue[nq++] = fsid;
}

static void process_queue(int recurse, int fileobj)
{
	if (recurse) {
		int i;
		int n = nq;
		int *q = alloca( n*sizeof(int) );
		assert(q);
		memcpy( q, fsid_queue, n*sizeof(int) );
		nq = 0;
		for(i=0; i<n; i++)
			dumpobj( q[i], fileobj, 1, "" );
	} else
		nq = 0;
}



static void dump_callback(int fsid, struct mfs_subobj_header *obj,
			  struct mfs_attr_header *attr, void *data)
{
	int i;
	char *p = data;
	struct mfs_obj_attr *objattr;
	
	int intvalue = 0;
	struct tm *timeptr;
	time_t date_time;
	static char wday_name[7][3] = {
		"Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"
	};
	static char mon_name[12][3] = {
		"Jan", "Feb", "Mar", "Apr", "May", "Jun",
		"Jul", "Aug", "Sep", "Oct", "Nov", "Dec"
	};

	if (!attr) {
		if (last) printf("}\n");
		last = fsid;
		printf("%s %d/%d %s{\n",
		       schema_type(obj->obj_type), fsid, obj->id,
		       obj->flags?"PRIMARY ":"");
		return;
	}

	if (schema_attrib(obj->obj_type,attr->attr)) {
		printf("\t%s[%d]=", 
		       schema_attrib(obj->obj_type,attr->attr), attr->attr);
	} else {
		printf("\t[%d]=", attr->attr);
	}
	switch (attr->eltype>>6) {
	case TYPE_STRING:
		for (i=0;i<attr->len-4;) {
			char *s = (char *)&p[i];
			printf("%s ", s);
			i += strlen(s)+1;
		}
		break;

	case TYPE_FILE:
	  // Save on queue of objects to process later
		for (i=0;i<(attr->len-4)/4;i++)
			queue_add(ntohl(*(int *)&p[i*4]));
		// fall through

	case TYPE_INT:

		for (i=0;i<(attr->len-4)/4;i++) {
			intvalue = ntohl(*(int *)&p[i*4]);
			printf("%d ", intvalue);
		}
		if (option_human) {
			if (strstr(schema_attrib(obj->obj_type,attr->attr), "Date")) {
				date_time = (time_t)(intvalue*86400 + tzoff);
				timeptr = timefn(&date_time);
				printf(" (%.3s %.3s%3d %d)",
				       wday_name[timeptr->tm_wday],
				       mon_name[timeptr->tm_mon],
				       timeptr->tm_mday,
				       1900 + timeptr->tm_year);
			}
			if (strstr(schema_attrib(obj->obj_type,attr->attr), "Time")) {
				date_time = (time_t)(intvalue + tzoff);
				timeptr = timefn(&date_time);
				printf(" (%.2d:%.2d:%.2d)",
				       timeptr->tm_hour,
				       timeptr->tm_min,
				       timeptr->tm_sec);
			}
			if (strstr(schema_attrib(obj->obj_type,attr->attr), "Duration")) {
				date_time = (time_t)(intvalue);
				timeptr = gmtime(&date_time);
				printf(" (%.2d:%.2d:%.2d)",
				       timeptr->tm_hour,
				       timeptr->tm_min,
				       timeptr->tm_sec);
			}
		}
		break;

	case TYPE_OBJECT:
		objattr = (struct mfs_obj_attr *)p;
		for (i=0;i<(attr->len-4)/sizeof(*objattr);i++) {
			int  fsid = ntohl(objattr->fsid);
			queue_add(fsid);
			printf("%d/%d ",
			       fsid,
			       (int) ntohl(objattr->subobj));
			objattr++;
		}
		break;
	}
	printf("\n");
}

//
// Hash table to keep track of FSID's we've seen before
//
static int *hash_table = 0;
static unsigned int hash_size = 0;
static int seen_before(int fsid) {
	int i, hash, first;
	if (! hash_table) {
		hash_size = inode_count();
		hash_table = malloc( hash_size * sizeof(int) ); 
		assert(hash_table);
		for(i=0; i<hash_size; i++)
			hash_table[i] = -1;
	}
	first = hash = fsid_hash( fsid, hash_size );
	do {
		if(hash_table[hash] == fsid) return 1;
		if (hash_table[hash] != -1)
			hash = (hash+1)%hash_size;
	} while (hash != first && hash_table[hash] != -1);
	if (hash_table[hash] != -1)
		fprintf( stderr, "hash table full!?\n" );
	else
		hash_table[hash] = fsid;
	return 0;
}

static void free_hash() {
	if (hash_table) {
		free(hash_table);
		hash_table = 0;
		hash_size = 0;
	}
}


//
// Dump a buffer in hex
//
static void hexdump( char *buf, unsigned addroff, unsigned len) {
	int i, j;
	for(i=0; i<len; ) {
		printf( "%08x", i+addroff );
		for(j=0; j<8 && i<len-1; j++, i+=2) {
			printf( " %04x", *((u16 *)(buf+i)));
		}
		if (j<8 && i<len) {
			printf( "  %04x", *((u8 *)(buf+i)));
			i++;
		}
		putchar('\n');
	}
}


static void dumpobj(int fsid, int fileobj, int recurse, const char *path)
{
	void *buf;
	u32 size;
	int type;
	if (seen_before(fsid)) return;
	if (! mfs_valid_fsid(fsid)) {
	  fprintf( stderr, "Skipping invalid fsid: %d\n", fsid );
	  return;
	}

	type = mfs_fsid_type(fsid);
	switch (type) {
	case MFS_TYPE_DIR:
		if (recurse) {
			u32 count, i;
			struct mfs_dirent *dir = mfs_dir(fsid,&count);
			for(i=0; i<count; i++) {
				int len;
				char *newpath = alloca( strlen(path) + strlen(dir[i].name) + 2);
				assert(newpath);
				strcpy( newpath, path );
				strcat( newpath, dir[i].name );
				len = strlen(newpath);
				while (len>0 && newpath[len-1]== ' ') /* strip trailing blanks */
					newpath[--len]=0;
				strcat( newpath, "/" );
				dumpobj( dir[i].fsid, fileobj, 1, newpath );
			}
			if (dir) mfs_dir_free(dir);
		}
		break;

	case MFS_TYPE_STREAM:
		// do nothing with streams
		break;

	case MFS_TYPE_FILE:
		if (fileobj) {
			u32 off = 0;
			const u32 bsize = 128*1024;
			printf( "tyFile %d {\n", fsid );
			// hex dump,
			size = mfs_fsid_size(fsid);
			buf = alloca( bsize );
			if (buf) {
				for(off=0; off<size; off+= bsize) {
					int l = size-off;
					if (l > bsize) l = bsize;
					mfs_fsid_pread(fsid, buf, off, bsize);
					hexdump( buf, off, bsize );
				}
			} else {
				fprintf( stderr, "memory allocation failed allocating %d bytes for fsid: %d\n", size, fsid );
			}
			printf("}");
		}
		break;

	case MFS_TYPE_OBJ:
		// parse object
		size = mfs_fsid_size(fsid);
		buf = alloca(size);
		mfs_fsid_pread(fsid, buf, 0, size);
		last = 0;
		parse_object(fsid, buf, dump_callback);
		printf("}\n");
		process_queue(recurse, fileobj);
	}
}



static void usage(void)
{
	fprintf( stderr, "\n\
usage: mfs_dumpobj [options] <path|fsid>\n\
\n\
      -h			     display humnan readable dates, times, and durations\n\
      -f                             hexdump file objects\n\
      -l			     convert dates and times to local TZ\n\
      -r                             recusive\n\
");
	credits();
	exit(1);
}


int main(int argc, char *argv[])
{
	int fsid;
	int c;
	int recurse = 0;
	int fileobj = 0;

	mfs_init();

	while ((c = getopt(argc, argv, "fhlr")) != -1 ){
		switch (c) {
		case 'f':
			fileobj = 1; /* hexdump file objects */
			break;

		case 'h':
			option_human = 1;
			break;

		case 'l':
#ifdef TIVO
			tzoff = tzoffset();
#else
			timefn = localtime;
#endif
			break;
		case 'r':
			recurse = 1;
			break;

		default:
			usage();
		}
	}

	argc -= optind;
	argv += optind;

	if (argc < 1) {
		usage();
	}

	fsid = mfs_resolve(argv[0]);
	dumpobj(fsid,fileobj,recurse,argv[0]);
	free_hash();

	return 0;
}
