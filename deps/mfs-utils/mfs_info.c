/*
  media-filesystem diagnostic utility
  tridge@samba.org, January 2001
  released under the Gnu GPL v2
*/

#include "mfs.h"

#include <assert.h>

static char *prog="";
static void usage(void)
{
	fprintf(stderr,"\n\
usage: %s [options] [<path|fsid>]\n\
\n\
   options:\n\
	-f		Fix the superblock crc, if it is incorrect.\n\
	-h		Display this usage info.\n\
        -s|s1           Scan for max tyStream fsid referenced in /Resource\n\
        -s2             Scan for max tyStream fsid NOT referenced in /Recording\n\
        -s3             Run both the s1 and s2 scans.\n\
", prog );
	credits();
	exit(1);
}

//
// Queue of referenced fsid's to dump out after the referencing object
//
typedef struct queue_t {
  int nq;
  int nalloc;
  int *queue;
} queue;

static queue queue_new() {
	queue rv;
	rv.nq = rv.nalloc = 0;
	rv.queue = 0;
	return rv;
}

static queue queue_copy(queue *q) {
	queue rv;
	rv.nq = rv.nalloc = q->nq;
	rv.queue = malloc( q->nq*sizeof(int) );
	memcpy( rv.queue, q->queue, q->nq*sizeof(int) );
	return rv;
}

static void queue_add(queue *q, int fsid) {
	if (q->nq == q->nalloc) {
		q->nalloc = (q->nalloc <= 0) ? 102400 : q->nalloc*2;
		q->queue = realloc( q->queue, q->nalloc*sizeof(int) );
	}
	q->queue[q->nq++] = fsid;
}

static void queue_free(queue *q) {
	free(q->queue);
	q->nalloc = q->nq = 0;
}

//
// Hash table to keep track of FSID's we've seen before
//
typedef struct hash_table_t {
	int *hash_table;
	unsigned int hash_size;
} hash_table;

static hash_table hash_table_new() {
	hash_table rv;
	rv.hash_table = 0;
	rv.hash_size = 0;
	return rv;
}

static int hash_table_search(hash_table *h, int fsid) {
	int i, hash, first;
	if (! h->hash_table) {
		h->hash_size = inode_count();
		h->hash_table = malloc( h->hash_size * sizeof(int) ); 
		assert(h->hash_table);
		for(i=0; i<h->hash_size; i++)
			h->hash_table[i] = -1;
	}
	first = hash = fsid_hash( fsid, h->hash_size );
	do {
		if(h->hash_table[hash] == fsid) return 1;
		if (h->hash_table[hash] != -1)
			hash = (hash+1)%h->hash_size;
	} while (hash != first && h->hash_table[hash] != -1);
	if (h->hash_table[hash] != -1)
		fprintf( stderr, "hash table full!?\n" );
	else
		h->hash_table[hash] = fsid;
	return 0;
}

static void hash_table_free(hash_table *h) {
	if (h->hash_table) {
		free(h->hash_table);
		h->hash_table = 0;
		h->hash_size = 0;
	}
}


static queue *qptr;
static hash_table *hptr;
void obj_callback(int fsid, struct mfs_subobj_header *obj,
		  struct mfs_attr_header *attr, void *data)
{
	int i;
	char *p = data;
	struct mfs_obj_attr *objattr;
	
	if (!attr) {
		return;
	}

	switch (attr->eltype>>6) {
	case TYPE_FILE:
		// Save on queue of objects to process later
		for (i=0;i<(attr->len-4)/4;i++)
			queue_add(qptr, ntohl(*(int *)&p[i*4]));
		break;

	case TYPE_OBJECT:
		objattr = (struct mfs_obj_attr *)p;
		for (i=0;i<(attr->len-4)/sizeof(*objattr);i++) {
			int  id = ntohl(objattr->fsid);
			if (id != fsid)
				queue_add(qptr, id);
			objattr++;
		}
		break;
	}
}

typedef void (*stream_fn)(int fsid);
static void traverse(queue *q, hash_table *h, int fsid, stream_fn sfn ) {


	if (hash_table_search(h,fsid)) return;
	fprintf( stderr, ".");
	switch(mfs_fsid_type(fsid)) {
	case MFS_TYPE_DIR:
	{
		struct mfs_dirent *dir;
		u32 count, i;
		dir = mfs_dir(fsid, &count);
		for (i=0;i<count;i++) {
			traverse(q, h, dir[i].fsid, sfn);
		}
		if (dir) mfs_dir_free(dir);
		break;
	}
	case MFS_TYPE_STREAM:
	{
		if (sfn)
			sfn(fsid);
		break;
	}
	case MFS_TYPE_OBJ:
	{
		// parse object
		int i;
		u32 size = mfs_fsid_size(fsid);
		char *buf = alloca(size);
		queue q1;

		mfs_fsid_pread(fsid, buf, 0, size);
		qptr = q;
		parse_object(fsid, buf, obj_callback);
		q1 = queue_copy(q);
		q->nq = 0;
		for(i=0; i<q1.nq; i++) {
			traverse( q, h, q1.queue[i], sfn );
		}
		queue_free(&q1);
		break;
	}
	}
}

static void stream_scan1()
{
	const char *path = "/Resource";
	int mx_fsid = 0;
	u64 mx_sz = 0;
	u64 size;
	queue q;
	hash_table h;

	void sfn(int fsid) {
		struct mfs_inode inode;
		mfs_load_inode( fsid, &inode );
		if (fsid > mx_fsid) mx_fsid = fsid;
		size = inode.size;
		if (inode.units == 0x20000)
			size <<= 17;
		if (size > mx_sz) mx_sz = size;
		fprintf( stderr, ".");
	}
	
	// Recursively traverse path and update the max fsid/size for
	// each stream we see.
	fprintf( stderr, "traversing %s... ", path );
	q = queue_new();
	h = hash_table_new();
	traverse(&q, &h, mfs_resolve(path), sfn );
	queue_free(&q);
	hash_table_free(&h);
	fprintf( stderr, "\n" );


	// print results
	fprintf( stderr, "max stream fsid under /Resource: %d\n", mx_fsid );
	fprintf( stderr, "max stream size under /Resource: %uMB\n", (unsigned int)(mx_sz/(1024*1024)));
}

void check_inode_fn(struct mfs_inode *inode, void *data)
{
	if (inode->type != MFS_TYPE_STREAM) return;
	if (hash_table_search(hptr,inode->id)) return;
	fprintf( stderr, ".");
	queue_add(qptr, inode->id);
}


static void stream_scan2()
{
	const char *path = "/Recording";
	int i, mx_fsid = 0;
	u64 mx_sz = 0;
	u64 size;
	queue q, q1;
	hash_table h;


	// Recursively traverse path and record all fsid's referenced
	// there in a hash table.
	fprintf( stderr, "traversing %s... ", path );
	q1 = queue_new();
	h = hash_table_new();
	hptr = &h;
	traverse(&q1, &h, mfs_resolve(path),0);
	queue_free(&q1);
	fprintf( stderr, "\n" );

	// Walk through ALL fsid's and record the tyStream fsids not
	// seen under path
	fprintf( stderr, "scanning inodes... ");
	q = queue_new();
	qptr = &q;
	mfs_all_inodes( check_inode_fn, (void *)0 );
	fprintf( stderr, "\n" );
	hash_table_free(&h);

	// Traverse again in case any new streams have appeared (e.g. live cache)
	h = hash_table_new();
	fprintf( stderr, "traversing %s... ", path );
	q1 = queue_new();
	traverse(&q1, &h, mfs_resolve(path),0);
	queue_free(&q1);
	fprintf( stderr, "\n" );

	// Walk through tyStreams in the queue not in the hash
	for(i=0; i<q.nq; i++) {
		int fsid = q.queue[i];
		struct mfs_inode inode;
		if (hash_table_search( &h, fsid)) continue;
		fprintf( stderr, "%d ", fsid );
		mfs_load_inode( fsid, &inode );
		if (fsid > mx_fsid) mx_fsid = fsid;
		size = inode.size;
		if (inode.units == 0x20000)
			size <<= 17;
		if (size > mx_sz) mx_sz = size;
	}
	fprintf( stderr, "\n");
	queue_free(&q);
	hash_table_free(&h);

	// print results
	fprintf( stderr, "max stream fsid excluding /Recordings: %d\n", mx_fsid );
	fprintf( stderr, "max stream size excluding /Recordings: %uMB\n", (unsigned int)(mx_sz/(1024*1024)));
}


int main(int argc, char *argv[])
{
	int fix=0;
	int c;
	int scan = 0;

	while ((c = getopt(argc, argv, "hfsT::")) != -1 ){
		switch (c) {
		case 'h':
			usage();
			break;
		case 'f':
			fix = 1;
			break;
		case 's':
			if (optarg != 0)
				scan = atoi(optarg);
			else
				scan = 1;
			break;
		}
	}

	argc -= optind;
	argv += optind;

	mfs_init_fix(fix);
	mfs_info();

	if (argc > 0) {
		printf("\n");
		mfs_fsid_info(mfs_resolve(argv[0]));
	}

	if (scan & 1) {
		printf("\n");
		stream_scan1();
	}
	if (scan & 2) {
		printf("\n");
		stream_scan2();
	}
	return 0;
}
