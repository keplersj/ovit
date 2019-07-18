/*
  media-filesystem export utility
  tridge@samba.org, January 2001
  released under the Gnu GPL v2
*/

#include "mfs.h"


void my_export_file(FILE *log, u32 fsid, char *dest, u64 start, u64 count, u64 chunk )
{
	void *buf;
	int bufsize = 128*1024;
	int n;
	u64 ofs;
	u64 size, total=0;
	int pct, last_pct;
	unsigned char *ptr;
   int zeroBlocks;
   int numZeros;
	int index;
	int wchunk;

	ofs = start;

#if 0
	fd = open(dest, O_WRONLY|O_CREAT|O_TRUNC|O_LARGEFILE, 0644);
	if (fd == -1) {
		perror(dest);
		exit(1);
	}
#endif

	buf = malloc(bufsize);
	mfs_readahead(1);
	size = mfs_fsid_size(fsid);
	fprintf(log, "exporting fsid %d of size %lld to %s\n", fsid, size, dest);
	if (start > size) {
		printf("start beyond EOF!\n");
		exit(1);
	}
	if (start+count>size || count==0) {
		count = size-start;
	}
	if (start || count) {
		fprintf(log, "starting at %lld for %lld bytes\n", start, count);
	}

	last_pct=0;

	if ( chunk != 0 )
   {
		ofs = chunk * bufsize;
   }
	while (total<count &&
	       (n=mfs_fsid_pread(fsid, buf, ofs, MIN(bufsize,count-total))) > 0) {


#if 0

		if (write(fd, buf, n) != n) {
			fprintf(stderr,"failed to write to %s\n", dest);
			break;
		}
#endif

		ofs += n;
		total += n;

		ptr = buf;
		numZeros = 0;
#if 1
		printf( "%x %x %x %x %x %x %x %x\n",
			ptr[ 0 ], ptr[ 1 ], ptr[ 2 ], ptr[ 3 ],
			ptr[ 4 ], ptr[ 5 ], ptr[ 6 ], ptr[ 7 ] );
#endif
		for( index = 0 ; index < 8 ; index++ )
		{
			if ( ptr[ index ] == 0 )
			{
				numZeros++;
			}
		}
		if ( numZeros == 8 )
		{
			zeroBlocks++;
			wchunk = ofs / 128 / 1024;
			printf( "Zero Chunk at %d\n", wchunk );
		}

	   if ( chunk != 0 )
      {
			break;
		}

		pct = (100*total)/count;
		if (pct != last_pct) {
			fprintf(log, "%d%%\r", pct);
			fflush(log);
			last_pct = pct;
		}
	}
#if 0
	close(fd);
#endif
}


static void usage(void)
{
	printf("\n"
"usage: mfs_export [options] <path|fsid> <dest>\n"
"\n"
"   options:\n"
"        -s <start>                     start offset\n"
"        -c <count>                     number of bytes (defaults to all)\n"
"        -z <chunk>                     chunk to read (defaults to all)\n"
);         
	credits();
	exit(1);
}


 int main(int argc, char *argv[])
{
	int fsid;
	int c;
	u64 start=0;
	u64 count=0;
	u64 chunk=0;

	while ((c = getopt(argc, argv, "hs:c:z:")) != -1 ){
		switch (c) {
		case 'h':
			usage();
			break;

		case 's':
			start = strtoll(optarg, NULL, 0);
			break;

		case 'c':
			count = strtoll(optarg, NULL, 0);
			break;			

		case 'z':
			chunk = strtoll(optarg, NULL, 0);
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

	mfs_init();
	fsid = mfs_resolve(argv[0]);
	my_export_file(stdout, fsid, argv[1], start, count, chunk);
	return 0;
}
