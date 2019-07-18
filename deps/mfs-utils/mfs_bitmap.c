/*
  media-filesystem diagnostic utility
  tridge@samba.org, January 2001
  released under the Gnu GPL v2
*/

#include "mfs.h"

static void usage(void)
{
	printf("\n\
mfs_bitmap <zone>\n\
");
	credits();
	exit(1);
}

 int main(int argc, char *argv[])
{
	int c, i, zone;
	struct bitmap *bm;
	u64 limit = 0;
	u32 used=0;

	while ((c = getopt(argc, argv, "L:")) != -1){
		switch (c) {
		case 'L':
			limit = strtoll(optarg, NULL, 0);
			break;
		}
	}

	argc -= optind;
	argv += optind;

	if (argc < 1) {
		usage();
	}

	mfs_init();

	printf("Generating bitmap with limit %lld\n", limit);

	zone = atoi(argv[0]);

	bm = mfs_zone_bitmap(zone, limit);

	printf("Bitmap for %d blocks\n", bm->n);

	for (i=0;i<(bm->n+31)/32;i++) {
		printf("%08x ", bm->b[i]);
		used += bitcount32(bm->b[i]);
		if ((i+1) % 8 == 0) printf("\n");
	}
	printf("\n");

	printf("%d/%d blocks used (%.1f%% %.1fMB)\n", 
	       used, bm->n, (100.0*used)/bm->n,
	       used*(mfs_zone_size(zone)/bm->n)/2048.0);

	return 0;
}
