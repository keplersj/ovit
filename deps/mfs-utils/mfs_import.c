/*
  media-filesystem import utility

  developed by 
  tivodvlpr@hotmail.com, August 2002
  released under the Gnu GPL v2
  
  this work is based on work by tridge@samba.org

  07/15/03: cleaned and optimized by embeem <mbm@alt.org>

*/

#include <sys/types.h>
#include <sys/socket.h>
#include <sched.h>
#include <time.h>
#include <unistd.h>
#include <limits.h>

#include "mfs.h"

#ifdef MSG_WAITALL
static 	int use_recv = 1;
#endif

static inline ssize_t readall( int fd, void *buf, size_t count ) 
{
	unsigned char *p = buf;
	ssize_t rc = 0;
	ssize_t ret = 0;
	
#ifdef MSG_WAITALL
	// Use recv on a socket to avoid reading in small MTU sized chunks
	// MSG_WAITALL will return the whole thing in one recv call.
	if (use_recv) {
		while (count > 0) {
			ret = recv( fd, p, count, MSG_WAITALL );
			if (ret <= 0) {
			  //	perror("recv error, falling back to read loop. ");
				use_recv=0;
				break;
			}
			p += ret;
			count -= ret;
			rc += ret;
		}
	}
	if (!use_recv) 
#endif
	{
		// 
		// non socket case.
		//
		while (count > 0) {
			ret = read( fd, p, count );
			if (ret <= 0) {
				if (errno != EINTR && errno != EAGAIN)
					return ret;
				continue;
			}
			p += ret;
			count -= ret;
			rc += ret;
		}
	}
	return rc;
}

//  
// Overwrite stream <fsid> with data from <fd>, starting at byte offset
// <start> in the fsid and overwriting <count> bytes.
//
// <nbufs> is the chunk size, in units of 512byte sectors.  If <=0, a
// default is used (currently 256).
//
// <delayms> is a delay to impose between chunks.  -1 means no delay, 0 means just a sched_yield,
// positive is the number of milliseconds to sleep.
//
// If <verbose> is true progress and other status messages will be printed on
// stderr.  Otherwise, only error messages will come out there.
//
// Returns the number of bytes written, or -1 on error.
//
// 

static int overwrite_stream(const u32 fsid, const int fd, u64 start, 
			    u64 count, int delayms, u32 nbufs, int verbose )
{
        struct mfs_inode inode;
        int run;
        int pct, last_pct=0;
        u64 ofs=0, size;
	u64 total;
	struct timespec delay;
	int nb = (nbufs>0) ? nbufs : 256;
	ssize_t ret = 0;

        unsigned char *buf = alloca(nb*SECTOR_SIZE);
	if (!buf) {
		fprintf( stderr, "Couldn't allocate buffer: %d\n", 
			 nb*SECTOR_SIZE );
		exit(1);
	}

	if (delayms > 0) {
		delay.tv_sec = delayms/1000;
		delayms %= 1000;
		delay.tv_nsec = delayms*1000000;
	}

        mfs_load_inode(fsid, &inode);
        size = mfs_fsid_size(fsid);
	if (start>size) {
		return 0;
	}
	if (start+count>size || count==0) {
		count = size-start;
	}
	total = count;

	if (verbose) {
#ifdef TIVO_S1
	  if (size < ULONG_MAX && start < ULONG_MAX && count < ULONG_MAX) 
		fprintf(stderr,
			"importing fsid %d of size %lu starting at offset %lu for %lu bytes\n", 
			fsid, (unsigned long)size, (unsigned long)start, (unsigned long)count);
	  else
#endif
		fprintf(stderr,
			"importing fsid %d of size %llu starting at offset %llu for %lld bytes\n", 
			fsid, size, start, count);
	}

	// Special case for short files: data is in the inode itself.
	if(inode.num_runs == 0) {
		if (count > nb*SECTOR_SIZE) {
			fprintf( stderr, "inode data too big for buffer!?  Output truncated.\n" );
			count = nb*SECTOR_SIZE;
		}
		
		ret = readall(fd, buf, count );
		if (ret <= 0) {
			fprintf(stderr,"read failed.\n");
			exit(1);
		} else if (ret != count) {
			fprintf(stderr,"short read: %d/%d\n", 
				ret, (int)count);
		}
		mfs_fsid_pwrite( fsid, buf, start, count );
		return count;
	}

        for (run=0;count>0 && run<inode.num_runs;run++) {
                int len=inode.u.runs[run].len;   //number of sectors left
                int sec=inode.u.runs[run].start; //sector to write to

		if (start > ofs+((u64)len<<SECTOR_SHIFT)) { // are we there yet?
			ofs +=((u64)len<<SECTOR_SHIFT);
			continue; /* nyet */
		}
                while (len>0 && count>0) {
			int rlen, blen, ret;
			if (start > ofs) {
				// Find first sector we want
				int bofs = start-ofs;
				int sofs = bofs>>SECTOR_SHIFT;
				int bmod = bofs&(SECTOR_SIZE-1);
				ofs += bofs;
				len -= sofs;
				sec += sofs;
				// Partial first sector requires a read/modify/write
				if (bmod) {
					int blen = MIN(SECTOR_SIZE-bmod,count);
					int ret;
					mfs_read_sectors(buf,sec,1);
					ret = readall(fd, buf+bmod, blen );
					if (ret <= 0) {
						fprintf(stderr,"read failed.\n");
						exit(1);
					} else if (ret != blen) {
						fprintf(stderr,"short read: %d/%d\n", 
							ret, blen);
					}
					mfs_write_sectors(buf,sec,1);
					sec++;
					len--;
					count -= ret;
					ofs += ret;
				}
			}
                        rlen = MIN(nb,len);  //number of sectors to read this round
			blen = MIN(rlen<<SECTOR_SHIFT,count);
                        ret  = readall(fd,buf,blen); //number of BYTES read
			
                        if (ret < 0) {
				perror("read failed:");
				return -1;
			}

                        ofs += ret;
			count -= ret;
			mfs_write_partial(buf, sec, ret);

			if (ret != blen) { // EOF on input
			  fprintf( stderr, "input stream truncated early:  %lld out of %lld bytes received\n", 
				   (total-count), count );
			  break;
			}

                        ret >>= SECTOR_SHIFT; //convert to SECTORS

                        len -= ret;
                        sec += ret;


			// Report progress
			if (verbose) {
				pct = (100 * (total-count))/total;
				if (pct != last_pct) {
					fprintf(stderr,"%d%%\r", pct);
					fflush(stderr);
					last_pct = pct;
				}
			}


			// Delay, if requested
			if (delayms > 0)  /* play nice: throttle bandwidth */
				nanosleep( &delay, NULL ); 
			else if (delayms == 0)
				sched_yield();
                }
        }
	return total-count;
}

static char *prog="";
static void usage(void)
{
	fprintf(stderr,"\n\
usage: %s [options] <path|fsid> [<src>]\n\
\n\
   options:\n\
        -c <count>          number of bytes (defaults to size of fsid)\n\
	-n <count>	    number of sectors to use for buffering\n\
	-p <priority>       Priority  0: ts; 1-99 RT FIFO\n\
        -s <start>          start offset, in bytes (defaults to 0)\n\
	-r <ms>		    rate control (throttle)\n\
                              -'ve  : no delay (default)\n\
                              0     : sched_yield() between chunks\n\
                              +'ve  : # of ms to delay between chunks\n\
	-v		    verbose progress messages on stderr\n\
\n\
   <src> defaults to stdin if not present on the command line.\n\
", prog );
	credits();
	exit(1);
}


 int main(int argc, char *argv[])
{
	int fsid=0, fd=STDIN_FILENO;
	int c;
	u64 start=0;
	u64 count=0;
	int delayms = -1;
	u32 nbufs = 256;	/* number of sectors to use for I/O buffering  */
	int verbose = 0;
	int ret;
	int i;

	prog = argv[0];

	fprintf( stderr, "starting mfs_import with args:\n");
	for(i=0; i<argc; i++) {
	  fprintf( stderr, "%s ", argv[i] );
	}
	fprintf( stderr, "\n");
	while ((c = getopt(argc, argv, "hvs:c:r:n:p:")) != -1 ){
		switch (c) {
		case 'h':
			usage();
			break;

		case 'c':
			count = strtoll(optarg, NULL, 0);
			break;			

		case 'p':
			fixPriority(strtoll(optarg, NULL, 0));
			break;

		case 'n':
			nbufs = strtoll(optarg, NULL, 0);
			break;

		case 's':
			start = strtoll(optarg, NULL, 0);
			break;

		case 'r':
			delayms = strtoll(optarg, NULL, 0);
			break;

		case 'v':
			verbose = 1;
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

	if (fsid == 0)
	{
		fprintf(stderr, "Error: The path or fsid %s is invalid\n", argv[1]);
		exit(1);
	}

	if ( argc>=2 && strcmp(argv[1],"-")!=0 &&
	     (fd = open(argv[1], O_RDONLY)) < 0) {
		perror(argv[1]);
		return 1;
	} 
	ret = overwrite_stream(fsid,fd,start,count,delayms,nbufs,verbose);
	return (ret > 0 ) ? 0 : 1;
}

