/* play a video given a blkmap file

   tridge@samba.org, December 2000

   released under the Gnu General Public License version 2 of later
*/
#include "mfs.h"
#include <sys/mman.h>
#include <fcntl.h>
#include <ncurses.h>

#define AUDIO_PROG "mpg123 -q - 2> /dev/null"
#define VIDEO_PROG "mpeg2dec -o xshm > /dev/null 2>&1"

#define CHUNK_SIZE (256*512)
#define NUM_BLKS 6

#define FLAG_IFRAME 1

static pid_t peer_pid;
static int use_ncurses = 1;

struct buffer {
	char *buf;
	int write_ptr;
	int read_ptr;
	int count;
	int fd;
	int delay;
};

static struct buffer abuffer, vbuffer;

struct shmem {
	int size, blk, start;
	int rate;
	int bufsize, delay;
	unsigned current_flags;
	unsigned flags[NUM_BLKS];
	int blknum[NUM_BLKS];
	char data[NUM_BLKS][CHUNK_SIZE];
};

static struct shmem *shmem;

static void *shm_setup(int size)
{
	void *ret;
	int fd, zero=0;
	char template[20] = "/tmp/shm.XXXXXX";

	/* mkstemp() isn't really portable but use it if it exists as
           otherwise you are open to some nasty soft-link /tmp based
           hacks */
	fd = mkstemp(template);
	if (fd == -1) {
		return NULL;
	}
	lseek(fd, size, SEEK_SET);
	write(fd, &zero, sizeof(zero));
	ret = mmap(0, size, PROT_READ | PROT_WRITE, MAP_FILE | MAP_SHARED,
		   fd, 0);
	close(fd);
	unlink(template);
	if (ret == (void *)-1) return NULL;
	return ret;
}

static void push_buffer(struct buffer *buffer, char *buf, int size)
{
	int n;
	fd_set fds;

	void push(struct buffer *obuffer) {
		int n = obuffer->count - obuffer->delay;
		if (shmem->bufsize - obuffer->read_ptr < n) {
			n = shmem->bufsize - obuffer->read_ptr;
		}
		n = write(obuffer->fd, 
			  &obuffer->buf[obuffer->read_ptr], n);
		if (n > 0) {
			obuffer->read_ptr += n;
			obuffer->count -= n;
			if (obuffer->read_ptr == shmem->bufsize) {
				obuffer->read_ptr = 0;
			}
		}
	}


	while (size) {
		if (buffer->count < shmem->bufsize) {
			n = size;
			if (buffer->write_ptr + n > shmem->bufsize)
				n = shmem->bufsize - buffer->write_ptr;
			if (buffer->count + n > shmem->bufsize)
				n = shmem->bufsize - buffer->count;
			memcpy(&buffer->buf[buffer->write_ptr], buf, n);
			buffer->write_ptr += n;
			buffer->count += n;
			buf += n;
			size -= n;
			if (buffer->write_ptr == shmem->bufsize) {
				buffer->write_ptr = 0;
			}
		}

		if (size == 0) break;

		FD_ZERO(&fds);
		if (abuffer.count > abuffer.delay) FD_SET(abuffer.fd, &fds);
		if (vbuffer.count > vbuffer.delay) FD_SET(vbuffer.fd, &fds);
		if (select(MAX(abuffer.fd, vbuffer.fd)+1, 
			   NULL, &fds, NULL, NULL) <= 0) {
			continue;
		}

		if (FD_ISSET(abuffer.fd, &fds)) {
			push(&abuffer);
		}
		if (FD_ISSET(vbuffer.fd, &fds)) {
			push(&vbuffer);
		}
	}
}

static void parse_chunk(unsigned char buf[CHUNK_SIZE], unsigned flags)
{
	int num_recs = buf[0];
	int i;
	unsigned char *p;
	int ofs=0;

	/* each 128k chunk starts with N 16 byte records that tell you
	   what sorts of things are in the chunk */
	p = &buf[4];
	ofs = 4 + num_recs*16;

	for (i=0;i<num_recs;i++) {
		unsigned size = (p[0]<<8 | p[1])<<4 | (p[2]>>4);
		unsigned type = p[3];
		// unsigned subtype = p[2]&0xf;

		if (type == 0xe0) {
			push_buffer(&vbuffer, (char *)&buf[ofs], size);
			flags &= ~FLAG_IFRAME;
			ofs += size;
		} else if (type == 0xc0) {
			push_buffer(&abuffer, (char *)&buf[ofs], size);
			ofs += size;
		}
		p += 16;
	}
}


static void vplayer_play(void)
{
	int next = 0;

	while (1) {
		while (shmem->blknum[next] == -1) usleep(1000);
		
		if (shmem->blknum[next] == -2) break;

		parse_chunk( (unsigned char *)shmem->data[next], shmem->flags[next]);
		shmem->blknum[next] = -1;
		next = (next+1) % NUM_BLKS;
	}
}

static void curses_start(void)
{
	if (use_ncurses) {
		initscr(); cbreak(); noecho();
		nonl();
		intrflush(stdscr, FALSE);
		keypad(stdscr, TRUE);
		nodelay(stdscr, TRUE);
	}
}

static void curses_paint(void)
{
	if (use_ncurses) {
		clear();
		mvprintw(0, 0, "Block %d of %d\n", shmem->blk, shmem->size);
		refresh();
	} else {
		printf("Block %d of %d\r", shmem->blk, shmem->size);
	}
}

static void process_key(int c)
{
	switch (c) {
	case 'q':
		kill(peer_pid, SIGTERM);
		exit(0);
		break;
	case 's':
		shmem->blk += 100;
		if (shmem->blk >= shmem->size) shmem->blk = shmem->size-1;
		break;
	case 'b':
		shmem->blk -= 100;
		if (shmem->blk < 1) shmem->blk = 1;
		break;
	}
}

static void vplayer_fetch(char *fname, int raw)
{
	int next = 0;
	int fsid=0, fd=0;
	u64 size;

	if (shmem->start < 1) shmem->start = 1;

	if (!raw) {
		mfs_init();
		mfs_readahead(1);
		fsid = strtol(fname, NULL, 0);
		size = mfs_fsid_size(fsid);
	} else {
		fd = open(fname, O_RDONLY|O_LARGEFILE);
		if (fd == -1) {
			perror(fname);
			exit(1);
		}
		size = ll_seek(fd, 0, SEEK_END);
		ll_seek(fd, shmem->start*CHUNK_SIZE, SEEK_SET);
	}

	shmem->size = (size >> SECTOR_SHIFT) / 256;
	shmem->blk = shmem->start;
	shmem->current_flags = FLAG_IFRAME;

	curses_start();

	while (shmem->blk < shmem->size) {
		int c;
		while (shmem->blknum[next] != -1) usleep(1000);
		if (raw) {
			read(fd, shmem->data[next], CHUNK_SIZE);
		} else {
			mfs_fsid_pread(fsid, shmem->data[next], 
				       ((u64)(shmem->blk*256))<<SECTOR_SHIFT, CHUNK_SIZE);
		}
		shmem->flags[next] = shmem->current_flags;
		shmem->blknum[next] = shmem->blk;
		shmem->current_flags &= ~FLAG_IFRAME;
		shmem->blk++;
		next = (next+1) % NUM_BLKS;
		if (use_ncurses && (c = getch()) != ERR) {
			process_key(c);
		}
		curses_paint();
	}
	shmem->blknum[next] = -2;
}

static void usage(void)
{
	printf("\n\
   vplayer [options] <fsid>\n\
OR vplayer -l\n\
\n\
options:\n\
         -d <delay>                delay video\n\
         -l                        list fsid of recorded streams\n\
         -a <prog>                 set audio program (default %s)\n\
         -v <prog>                 set video program (default %s)\n\
         -s <start>                set starting block\n\
", AUDIO_PROG, VIDEO_PROG);
	credits();
	exit(1);
}

 int main(int argc, char *argv[])
{
	int i;
	int c, raw=0;
	char *audio_prog = AUDIO_PROG;
	char *video_prog = VIDEO_PROG;

	shmem = shm_setup(sizeof(*shmem));

	shmem->bufsize = 64*1024;

	while ((c = getopt(argc, argv, "d:Rb:la:v:s:n")) != -1) {
		switch (c) {
		case 'd':
			shmem->delay = strtol(optarg, NULL, 0);
			break;
		case 'R':
			raw=1;
			break;
		case 'b':
			shmem->bufsize=strtol(optarg, NULL, 0);
			break;
		case 's':
			shmem->start=strtol(optarg, NULL, 0);
			break;
		case 'l':
			mfs_init();
			query_streams("/Recording/Complete");
			exit(1);

		case 'a': 
			audio_prog = optarg;
			break;

		case 'v': 
			video_prog = optarg;
			break;

		case 'n':
			use_ncurses = 0;
			break;

		default:
			usage();
		}
	}

	argc -= optind;
	argv += optind;

	if (argc < 1) usage();

	for (i=0;i<NUM_BLKS;i++) {
		shmem->blknum[i] = -1;
	}

	if ((peer_pid=fork()) == 0) {
		peer_pid = getppid();
		vplayer_fetch(argv[0], raw);
	} else {
		FILE *audio_f;
		FILE *video_f;

		audio_f = popen(audio_prog, "w");
		if (!audio_f) {
			fprintf(stderr, "Failed to start audio program %s\n", audio_prog);
			exit(1);
		}
		video_f = popen(video_prog, "w");	
		if (!video_f) {
			fprintf(stderr, "Failed to start video program %s\n", video_prog);
			exit(1);
		}

		abuffer.fd = fileno(audio_f);
		vbuffer.fd = fileno(video_f);

		set_nonblocking(abuffer.fd);
		set_nonblocking(vbuffer.fd);

		abuffer.buf = malloc(shmem->bufsize);
		vbuffer.buf = malloc(shmem->bufsize);

		if (shmem->delay > 0) {
			abuffer.delay = shmem->delay;
		} else {
			vbuffer.delay = -shmem->delay;
		}

		vplayer_play();
	}
	
	kill(peer_pid, SIGTERM);

	return 0;
}
