/*
  media-filesystem dump utility
  tridge@samba.org, January 2001
  released under the Gnu GPL v2
*/

#include "mfs.h"

static void dump_raw(u32 start, u32 count)
{
	int chunk = 256;
	void *buf = malloc(chunk*SECTOR_SIZE);
	while (count) {
		int n = MIN(count, chunk);
		mfs_read_sectors(buf, start, n);
		write(1, buf, n*SECTOR_SIZE);
		start += n;
		count -= n;
	}
}

static void usage(void)
{
	printf("\n\
usage: mfs_dump [options] <sector> <count>\n\
\n\
options:\n\
        -R            raw data\n\
");         
	credits();
	exit(1);
}

 int main(int argc, char *argv[])
{
	u32 start;
	u32 count;
	int c;
	int raw=0;

	while ((c = getopt(argc, argv, "R")) != -1 ){
		switch (c) {
		case 'R':
			raw=1;
			break;
		default:
			usage();
		}
	}

	argc -= optind;
	argv += optind;

	if (argc < 2) {
		usage();
	}

	start = strtol(argv[0], NULL, 0);
	count = strtol(argv[1], NULL, 0);

	mfs_init();

	if (raw) {
		dump_raw(start, count);
	} else {
		void *buf = malloc(count*SECTOR_SIZE);
		mfs_read_sectors(buf, start, count);
		dump_sectors(buf, count);
		free(buf);
	}

	return 0;
}
