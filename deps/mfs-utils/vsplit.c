#include "mfs.h"

#define CHUNK_SIZE 256*512

static int audio_fd;
static int video_fd;

static int verbose;

static void parse_chunk(unsigned char buf[CHUNK_SIZE])
{
	int num_recs = buf[0];
	int i;
	unsigned char *p;
	int total_audio=0;
	int total_video=0;
	int ofs=0;

	/* each 128k chunk starts with N 16 byte records that tell you what sorts of things are
	   in the chunk */
	p = &buf[4];
	ofs = 4 + num_recs*16;

	for (i=0;i<num_recs;i++) {
		unsigned size = (p[0]<<8 | p[1])<<4 | (p[2]>>4);
		unsigned type = p[3];

		if (type == 0xe0) {
			write(video_fd, &buf[ofs], size);
			if (verbose) {
				printf("video at 0x%x of size 0x%x\n",
				       ofs, size);
			}
			total_video += size;
			ofs += size;
		}
		if (type == 0xc0) {
			write(audio_fd, &buf[ofs], size);
			if (verbose) {
				printf("audio at 0x%x of size 0x%x\n",
				       ofs, size);
			}
			total_audio += size;
			ofs += size;
		}
		p += 16;
	}

	printf("recs: %d vid: 0x%x aud: 0x%x total: 0x%x ofs=0x%x\n", 
	       num_recs, total_video, total_audio, 
	       total_audio+total_video+num_recs*16+4, ofs);
}

static void usage(void)
{
	printf("vsplit [options] <infile> <videofile> <audiofile>\n");
	printf("options: \n\
      -v                 verbose\n\
      -h                 help\n");
	credits();
	exit(1);
}

 int main(int argc, char *argv[])
{
	char *fname;
	int fd;
	unsigned char buf[CHUNK_SIZE];
	int c;

	while ((c = getopt(argc, argv, "vh")) != -1 ){
		switch (c) {
		case 'v':
			verbose++;
			break;
			
		case 'h':
		default:
			usage();
			exit(1);
		}
	}

	argc -= optind;
	argv += optind;

	if (argc < 3) {
		usage();
	}

	fname = argv[0];
	video_fd = open(argv[1], O_WRONLY|O_CREAT|O_TRUNC, 0644);
	audio_fd = open(argv[2], O_WRONLY|O_CREAT|O_TRUNC, 0644);

	fd = open(fname, O_RDONLY);

	/* discard the first chunk - it often contains garbage */
	read(fd, buf, CHUNK_SIZE);

	while (read(fd, buf, CHUNK_SIZE) == CHUNK_SIZE) {
		parse_chunk(buf);
	}

	close(audio_fd);
	close(video_fd);
	return 0;
}
