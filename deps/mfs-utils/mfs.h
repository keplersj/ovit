/*
  media-filesystem library
  tridge@samba.org, January 2001
  released under the Gnu GPL v2
*/

#include <pwd.h>
#include <string.h>
#include <stdio.h>
#include <errno.h>
#include <stdlib.h>
#include <signal.h>
#include <unistd.h>
#include <ctype.h>
#include <getopt.h>
#include <sys/types.h>
#include <sys/ioctl.h>
#include <dlfcn.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <netdb.h>
#include <sys/time.h>

#include "compat.h"

#ifdef TIVO_S1
#include <linux/ide-tivo.h>
#include <asm/unistd.h>
#endif /* TIVO_S1 */

#define MAX_ZONES 32
#define MAX_DEVS 64
#define SECTOR_SIZE 512
#define SECTOR_SHIFT 9

#define ZONE_INODE 0
#define ZONE_FILE 1
#define ZONE_STREAM 2

#define MFS_TYPE_DIR 4
#define MFS_TYPE_OBJ 8
#define MFS_TYPE_STREAM 2
#define MFS_TYPE_FILE 1

#define TYPE_INT 0
#define TYPE_STRING 1
#define TYPE_OBJECT 2
#define TYPE_FILE 3

#define MFS_FLAGS_CHAIN 0x80000000

#define MFS_FLAGS_PRIMARY 0x2000

#ifndef O_LARGEFILE

#ifdef TIVO_S1
#define O_LARGEFILE 0x8000

#else /* TIVO_S1 */

/* 0x8000 is O_DIRECT on 2.4 - bad. */
#define O_LARGEFILE 0

#endif /* TIVO_S1 */

#endif /* !O_LARGEFILE */

#if defined(TIVO_S2) && (_FILE_OFFSET_BITS != 64)
#error "LARGEFILE support required"
#endif /* defined(TIVO_S2) && (_FILE_OFFSET_BITS != 64) */

/* this sets what rounding of the number of blocks in a partition is
   done. I'm not totally sure, this is right - it oculd be 1024 */
#define MFS_BLOCK_ROUND 1024

#define VSERVER_PORT 8074

typedef unsigned u32;
typedef u_int64_t u64;
typedef unsigned short u16;
typedef unsigned char u8;

#define MFS_CRC_BASE 0xdeadf00d
#define MFS_ROOT_FSID 1

/* length = 232 */
struct mfs_super {
	u32 state;
	u32 magic; /* 0xabbafeed */
	u32 crc;
	u32 fill1;
	u32 geom1; /* 16 */
	u32 geom2; /* 1 */
	u32 geom3; /* 64 */
	u32 fill2[2]; /* unknown */
	/* 0x24 */
	char devlist[128];
	/* 0xa4 */
	u32 total_sectors;
	u32 fill3[7];
	/* 0xc4 */
	u32 zonemap_ptr; /* pointer to first zone map */
	u32 backup_zonemap_ptr; /* backup pointer to first zone map */
	u32 zonemap_size; /* size of first zone map (sectors) */
	u32 fill4[3];
	/* 0xd8 */
	u32 next_fsid; /* the next available fsid to be allocated */
	u32 fill5[2];
};

struct mfs_zone_map {
	u32 sector; /* this sector */
	u32 backup_sector; /* where our backup is */
	u32 zonemap_size; /* how many sectors in this zone map */
	u32 next_zonemap_ptr;
	u32 backup_next_zonemap_ptr;
	u32 next_zonemap_size;
	u32 fill2[2];
	u32 type; 
	u32 fill3;
	u32 crc;
	u32 zone_start;
	u32 next_zone_ptr1;
	u32 zone_size;
	u32 per_chunk;
	u32 fill4[2];
	u32 buddy_size; /* how many orders in buddy maps */
	u32 mapdata[0]; /* variable size */
};

struct mfs_run {
	u32 start;
	u32 len;
};

struct mfs_inode {
	u32 id;
	u32 typexx;
	u32 fill1[3];
	u32 units;
	u32 size;
	u32 used_units;
	u32 used_size;
	/* XXX fill2 used to be fill2[3]. used_* came out of elements
	 * 0 and 1
	 */
	u32 fill2;
	u8  type;
	u8  fill3;
	u16 fill4;
	u32 fill5;
	u32 crc;
	u32 flags;
	u32 num_runs;
	union {
		// cwingert - There are possible more than 24 runs, just fill up
		// the data space
		// struct mfs_run runs[24];
		struct mfs_run runs[56];
		char data[452];
	} u;
};

struct mfs_dirent {
	u32 fsid;
	u8 type;
	char *name;
};


struct mfs_obj_header {
	u32 fill1;
	u32 size;
};

struct mfs_subobj_header {
	u16 len;
	u16 len1; /* hmmm, why the dup length? perhaps for padding? */
	u16 obj_type;
	u16 flags;
	u16 fill[2];
	u32 id;
};

struct mfs_attr_header {
	u8 eltype;
	u8 attr;
	u16 len;
};

struct mfs_obj_attr {
	u32 fsid;
	int subobj;
};

struct vserver_cmd {
	u32	command;
	u32	param1;
	u32	param2;
};

struct bitmap {
	u32 *b;
	int n;
};

#define MFS_CMD_QUIT 0
#define MFS_CMD_READ 1
#define MFS_CMD_WRITE 2
#define MFS_CMD_ZERO 3
#define MFS_CMD_LIST_SECTORS 0100

#ifndef MIN
#define MIN(a,b) ((a)<(b)?(a):(b))
#endif

typedef void (*object_fn)(int fsid, struct mfs_subobj_header *obj, 
                          struct mfs_attr_header *attr, void *data);

/*******************************************************************************
    For io.c:mfs_list_sectors()
*******************************************************************************/

typedef struct run_desc_t {
  u32 drive;			/* drive number 0:Adrive 1:Bdrive */
  u32 partition;		/* partition number */
  u32 start;			/* start sector */
  u32 count;			/* number of sectors */
} run_desc;



#include "proto.h"

#ifdef TIVO_S1
char *strtok_r(char *s, const char *delim, char **ptrptr);
#endif

#ifndef MAX
#define MAX(a, b) ((a)>(b)?(a):(b))
#endif

/*******************************************************************************
    ppchacker: 01/18/2002
*******************************************************************************/
#define MFS_MAX_ARRAY_LEN	128

