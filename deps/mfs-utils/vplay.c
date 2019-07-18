/* play a video given a blkmap file

   tridge@samba.org, December 2000

   released under the Gnu General Public License version 2 
*/

#include "mfs.h"

#define CHUNK_SIZE (256*512)

static int verbose;

static void parse_chunk(unsigned char buf[CHUNK_SIZE],
			int video_fd, int audio_fd)
{
	int num_recs = buf[0];
	int i;
	unsigned char *p;
	int total_audio=0;
	int total_video=0;
	int ofs=0;

	/* each 128k chunk starts with N 16 byte records that tell you
	   what sorts of things are in the chunk */
	p = &buf[4];
	ofs = 4 + num_recs*16;

	for (i=0;i<num_recs;i++) {
		unsigned size = (p[0]<<8 | p[1])<<4 | (p[2]>>4);
		unsigned type = p[3];
		
		if (verbose) {
			printf("type 0x%x of size 0x%x\n", type, size);
		}

		if (type == 0xe0) {
			write(video_fd, &buf[ofs], size);
			total_video += size;
			ofs += size;
		} else if (type == 0xc0) {
			write(audio_fd, &buf[ofs], size);
			total_audio += size;
			ofs += size;
		} else if (verbose) {
			printf("unknown type 0x%x of size 0x%x\n", type, size);
		}
		p += 16;
	}
}


static void usage(void) 
{
	printf("\n\
    vplay <fsid> <vidfile>\n\
 OR vplay -p <fsid>\n\
 \n\
 options:\n\
    -s <start>          starting chunk\n\
    -c <count>          number of chunks to play\n\
    -v                  be verbose\n\
\n\
 use the 2nd form for direct playback on a TiVo\n\
\n\
 vplay takes a fsid for stream files. See liststreams\n\
");
	credits();
	exit(1);
}


 int main(int argc, char *argv[])
{
	int vfd=-1, fsid;
	char buf[CHUNK_SIZE];
	int chunk=0, count=0;
	char *vidfile;
	int playback = 0;
	int c;
	int audio_fd=-1, video_fd=-1;

	while ((c = getopt(argc, argv, "vphs:c:")) != -1 ){
		switch (c) {
		case 'v':
			verbose++;
			break;

		case 'p':
			playback = 1;
			break;

		case 's':
			chunk = atoi(optarg);
			break;

		case 'c':
			count = atoi(optarg);
			break;
			
		case 'h':
		default:
			usage();
			exit(1);
		}
	}

	printf("optind=%d argc=%d\n", optind, argc);

	argc -= optind;
	argv += optind;

	if (argc < (playback?1:2)) usage();

	fsid = atoi(argv[0]);

	mfs_init();
	
	if (playback) {
		video_fd = open("/dev/mpeg0v", O_WRONLY);
		audio_fd = open("/dev/mpeg0a", O_WRONLY);
	} else {
		vidfile = argv[1];

		vfd = open(vidfile, O_WRONLY|O_CREAT|O_TRUNC, 0644);
		if (vfd == -1) {
			perror(vidfile);
			exit(1);
		}
	}

	if (count == 0) {
		count = mfs_fsid_size(fsid)>>17;
	}

	printf("fsid %d has %lld chunks\n",
	       fsid, mfs_fsid_size(fsid)>>17);

	while (mfs_fsid_pread(fsid, buf, ((u64)chunk+1)<<17, CHUNK_SIZE) ==
	       CHUNK_SIZE && count) {
		if (playback) {
		  parse_chunk((unsigned char *)buf, video_fd, audio_fd);
		} else if (write(vfd, buf, CHUNK_SIZE) != CHUNK_SIZE) {
			printf("write failed\n");
			exit(1);
		}
		printf("chunk %d\r", chunk);
		chunk++;
		count--;
	}
	printf("chunk %d\n", chunk);
	return 0;
}
