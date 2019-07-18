/*
  media-filesystem streams list
  tridge@samba.org, January 2001
  released under the Gnu GPL v2
*/

#include "mfs.h"

static void usage(void)
{
	printf("\n\
usage: mfs_streams <path>\n\
\n\
");
	credits();
	exit(1);
}


 int main(int argc, char *argv[])
{
	const char *path = NULL;
	int c;

	while ((c = getopt(argc, argv, "")) != -1 ){
		switch (c) {
		default:
			usage();
		}
	}

	argc -= optind;
	argv += optind;

	if (argc >= 1) {
		path = argv[0];
	}

	mfs_init();

	if (!path) {
		int i;
		static const char *paths[] = {
			"/Recording/NowShowingByClassic",
			"/Recording/NowShowing",
			"/Recording/Complete",
		};
		static const int count = sizeof(paths)/sizeof(paths[0]);

		for(i=0; i<count; i++)
			if (mfs_resolve(paths[i]) != 0) {
				path = paths[i];
				break;
			}
	}

	printf("Listing streams in %s\n", path);
	query_streams(path);

	return 0;
}
