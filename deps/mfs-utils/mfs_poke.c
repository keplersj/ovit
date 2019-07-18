/*
  media-filesystem poke utility
  tridge@samba.org, January 2001
  released under the Gnu GPL v2
*/

#include "mfs.h"

static void usage(void)
{
	printf("\n\
usage: mfs_poke [options] <sector> <offset> <value>\n\
\n\
Poke a 32 bit integer at offset <offset> in sector <sector>\n\
");         
	credits();
	exit(1);
}

 int main(int argc, char *argv[])
{
	u32 sec;
	u32 offset;
	u32 value;
	char buf[SECTOR_SIZE];

	if (argc < 3) {
		usage();
	}

	sec = strtol(argv[1], NULL, 0);
	offset = strtol(argv[2], NULL, 0);
	value = strtol(argv[3], NULL, 0);

	mfs_init();

	value = htonl(value);
	mfs_read_sectors(buf, sec, 1);
	memcpy(buf+offset, &value, 4);
	mfs_write_sectors(buf, sec, 1);

	return 0;
}
