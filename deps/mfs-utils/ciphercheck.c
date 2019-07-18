/*
  media-filesystem streams list
  tridge@samba.org, January 2001
  released under the Gnu GPL v2
*/

#include "mfs.h"

static void usage(void)
{
	printf("\n\
usage: ciphercheck\n\
\n\
");
	credits();
	exit(1);
}

/**
 *  Check the masterchunk of a stream to see if it appears to be encrypted
 */
static int 
crypto_check( int fsid ) 
{
	static const unsigned char expected[] = { 0xf5, 0x46, 0x7a, 0xbd };
	unsigned char buf[4];
	mfs_fsid_pread( fsid, buf, 0, sizeof(buf) );
	return memcmp(buf,expected,sizeof(expected));
}

#define MAX_PARTS 1024
static void 
ciphercheck(const char *path)
{
	struct mfs_dirent *dir;
	int parts[MAX_PARTS];
	int cso[MAX_PARTS];
	u32 count, i;
	int np,nc;

	fprintf( stdout, "CipherCheck - based on CipherCheck.tcl by AlphaWolf_HK\n\n" );
	dir = mfs_dir(mfs_resolve("/Recording/LiveCache"), &count);
	if (count > 0) {
		np = query_int_list(dir[0].fsid, "Part/File", parts, MAX_PARTS );
		if (dir) mfs_dir_free(dir);
		if (np > 0)
			fprintf( stdout, "TyStream encryption is currently %s.\n",
				 crypto_check(parts[np-1])? "enabled" : "disabled");
	}
	
	fprintf( stdout, "\n\
Here is the status of your current recordings:\n\
\n\
Encrypted CSO Set Stream Name\n\
--------- ------- -----------\n");

	dir = mfs_dir(mfs_resolve(path), &count);
	for (i=0;i<count;i++) {
		np = query_int_list(dir[i].fsid, "Part/File", parts, MAX_PARTS );
		nc = query_int_list(dir[i].fsid, "Part/CommercialSkipOffset", cso, MAX_PARTS );
		if (np > 0) {
			fprintf(stdout,"%s%s%s\n", 
				(crypto_check(parts[np-1]) ? "Yes       " : "No        "),
				(nc==0 || cso[nc-1]==0 ? "No      " : "Yes     "),
				query_string(dir[i].fsid, "Showing/Program/Title"));
		}
	}
	if (dir) mfs_dir_free(dir);
}

 int main(int argc, char *argv[])
{
	const char *path = NULL;
	int c,i;
	static const char *paths[] = {
	  "/Recording/NowShowingByClassic",
	  "/Recording/NowShowing",
	  "/Recording/Complete",
	};
	static const int count = sizeof(paths)/sizeof(paths[0]);

	while ((c = getopt(argc, argv, "")) != -1 ){
		switch (c) {
		default:
			usage();
		}
	}

	mfs_init();
	for(i=0; i<count; i++)
	  if (mfs_resolve(paths[i]) != 0) {
	    path = paths[i];
	    break;
	  }
	ciphercheck(path);
	return 0;
}
