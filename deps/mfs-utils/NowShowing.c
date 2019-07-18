/*
  media-filesystem streams list
  tridge@samba.org, January 2001
  released under the Gnu GPL v2
*/

#include "mfs.h"

static void usage(void)
{
	printf("\n\
usage: NowShowing\n\
\n\
");
	credits();
	exit(1);
}

 int main(int argc, char *argv[])
{
	int c;

	while ((c = getopt(argc, argv, "")) != -1 ){
		switch (c) {
		default:
			usage();
		}
	}

	argc -= optind;
	argv += optind;
	mfs_init();
	generate_NowShowing(fileno(stdout));

	return 0;
}
