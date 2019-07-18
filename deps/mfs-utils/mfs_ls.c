/*
  media-filesystem ls utility
  tridge@samba.org, January 2001
  released under the Gnu GPL v2
*/

#include "mfs.h"

static int long_list;

static void dir_list(int fsid, int recurse)
{
	struct mfs_dirent *dir;
	u32 count, i;
	// mfs_fsid_info(fsid);
	dir = mfs_dir(fsid, &count);
	printf("dir: fsid=%d count=%d\n", fsid, count);
	if (long_list) {
		printf("      fsid   type           size     name\n      -----------------------------------\n");
	} else {
		printf("      fsid   type     name\n      -----------------------------------\n");
	}
	for (i=0;i<count;i++) {
		if (long_list) {
			printf("   %7d   %-8s %10lld     %s\n", 
			       dir[i].fsid, 
			       mfs_type_string(dir[i].type),
			       mfs_fsid_size(dir[i].fsid),
			       dir[i].name);
		} else {
			printf("   %7d   %-8s %s\n", 
			       dir[i].fsid, 
			       mfs_type_string(dir[i].type),
			       dir[i].name);
		}
	}

	if (recurse) {
		for (i=0;i<count;i++) {
			if (dir[i].type == MFS_TYPE_DIR) {
				printf("\n%s[%d]:\n", 
				       dir[i].name, dir[i].fsid);
				dir_list(dir[i].fsid, 1);
			}
		}
	}

	if (dir) mfs_dir_free(dir);
}



static void usage(void)
{
	printf("\n\
usage: mfs_ls [options] <path|fsid>\n\
\n\
      -R                             recurse\n\
      -l                             long list (with size)\n\
");
	credits();
	exit(1);
}


 int main(int argc, char *argv[])
{
	int fsid;
	int c;
	int recurse=0;

	while ((c = getopt(argc, argv, "Rl")) != -1 ){
		switch (c) {
		case 'R':
			recurse=1;
			break;
		case 'l':
			long_list=1;
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

	mfs_init();
	fsid = mfs_resolve(argv[0]);
	dir_list(fsid, recurse);
	return 0;
}
