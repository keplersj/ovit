/*
  media-filesystem diagnostic utility
  tridge@samba.org, January 2001
  released under the Gnu GPL v2
*/

#include "mfs.h"

 int main(int argc, char *argv[])
{
	int c;
	u64 limit = 0;

	while ((c = getopt(argc, argv, "L:")) != -1 ){
		switch (c) {
		case 'L':
			limit = strtoll(optarg, NULL, 0);
			break;
		}
	}

	argc -= optind;
	argv += optind;

	mfs_init();

	mfs_purge_all(limit);

	return 0;
}
