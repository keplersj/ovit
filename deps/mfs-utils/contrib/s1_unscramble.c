/*
  media-filesystem unscramble utility

  developed by 
  tivodvlpr@hotmail.com, August 2002
  released under the Gnu GPL v2

  this work is based on work by tridge@samba.org

  07/15/03: cleaned and optimized by embeem <mbm@alt.org>
  01/24/04: changed for unscramble (DarkHelmet)

*/

extern int scramble_present;	/* global variable in utils.c */

#include "mfs.h"
#define BUFSIZE 0x20000

void unscramble_stream(const u32 fsid) {
	struct mfs_inode inode;
	int run;
	unsigned char buf[BUFSIZE];
	int pct, last_pct=0;
	u64 ofs=0;
	u64 size;

	mfs_load_inode(fsid, &inode);
	size = mfs_fsid_size(fsid);

	for (run=0;run<inode.num_runs;run++) {
		int len=inode.u.runs[run].len;   //number of sectors left
		int sec=inode.u.runs[run].start; //sector to write to

		while (len>0) {
			int rlen = MIN(BUFSIZE>>SECTOR_SHIFT,len);  //number of sectors to read this round
			int ret  = rlen<<SECTOR_SHIFT; //number of BYTES read

			if (ofs == 0) {
				scramble_present = 0;
				memset(buf, 0, ret);
				mfs_read_sectors(buf, sec, rlen);
				if (buf[0] == 0xf5 && buf[1] == 0x46 && buf[2] == 0x7a && buf[3] == 0xbd) {
					fprintf(stderr, "Disk data is NOT scrambled!\n");
					return;
				}
				scramble_present = 1;
				memset(buf, 0, ret);
				mfs_read_sectors(buf, sec, rlen);
				if (buf[0] == 0xf5 && buf[1] == 0x46 && buf[2] == 0x7a && buf[3] == 0xbd) {
					fprintf(stderr, "unscramble.o kernel module appears to have cached the key!\n");
					fprintf(stderr, "unscrambling fsid %d of size %lld\n", fsid, size);
				} else {
					fprintf(stderr, "OOPS! unscramble.o either not loaded or hasn't cached the key! 0x%02x 0x%02x 0x%02x 0x%02x\n", buf[0], buf[1], buf[2], buf[3]);
					return;
				}
			}
			mfs_read_sectors(buf, sec, rlen);
			mfs_write_sectors(buf, sec, rlen);

			len -= rlen;
			sec += rlen;

			ofs += ret;
			pct = (100 * ofs) / size;
			if (pct != last_pct) {
				fprintf(stderr, "%d%%\r", pct);
				fflush(stderr);
				last_pct = pct;
			}
		}
	}
}

static void usage(void)
{
	printf("\nusage: mfs_unscramble <path|fsid>\n");
	exit(1);
}


 int main(int argc, char *argv[])
{
	int fsid=0;

	if (argc != 2) {
		usage();
	}

	mfs_init();
	fsid = mfs_resolve(argv[1]);

	if (fsid == 0)
	{
		fprintf(stderr, "Error: The path or fsid %s is invalid\n", argv[1]);
		exit(1);
	}

	unscramble_stream(fsid);

	return 0;
}

