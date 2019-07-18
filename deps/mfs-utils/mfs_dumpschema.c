/*
  media-filesystem schema dump
  tridge@samba.org, June 2001
  released under the Gnu GPL v2
*/

#include "mfs.h"

 int main(int argc, char *argv[])
{
  //	mfs_init();
	dump_schema(stdout);

	return 0;
}
