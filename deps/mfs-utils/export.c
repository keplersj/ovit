/*
  media-filesystem object export code
  tridge@samba.org, September 2002
  released under the Gnu GPL v2

  11/05/04: new version adapted from mfs_import. jamie@DDB/AO

*/

#include <sys/types.h>
#include <sys/socket.h>
#include <sched.h>
#include <time.h>
#include <unistd.h>

#include "mfs.h"
#include "log.h"

#define AUDIO_DEMUX

#ifdef AUDIO_DEMUX
extern int tyda_demux_chunk(char * buf, 
                            int    size,
                            int    out_fd);
extern int tyda_init(void);
#endif

// Demux audio for a complete TY buffer
static inline ssize_t demux_audio_writeall( int fd, void *buf, size_t count ) 
{
	/*-------------------------------------------------------------------------
	  Pass chunk to audio demuxer, who will actually write the data out to the
	  file/socket.
	  -------------------------------------------------------------------------*/
	if (0 == tyda_demux_chunk(buf, count, fd))
	{
		return count;
	}
	else
	{
		/*-----------------------------------------------------------------------
		  Error
		  -----------------------------------------------------------------------*/
		return -1;
	}
}

// Write a complete buffer to a file descriptor, handling partial writes
static inline ssize_t writeall( int fd, void *buf, size_t count ) 
{
	unsigned char *p = buf;
	int rc = 0;
	ssize_t ret = 0;

	while (count > 0) {
		ret = write( fd, p, count );
		if (ret < 0) {
			if (errno != EINTR && errno != EAGAIN)
			{
				return ret;
			}
			continue;
		}

		p += ret;
		count -= ret;
		rc += ret;
	}

	return rc;
}

//  
// Read from <fsid> and write to <fd>, starting at byte offset <start> and
// reading <count> bytes.
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
// Returns the number of bytes written to fd, or -1 on error.
//
// 
int export_file(const u32 fsid, const int fd, u64 start, 
		u64 count, int delayms, u32 nbufs, int verbose, int demux_audio )
{
	struct mfs_inode inode;
	int run, ret, rlen, blen;
	int pct, last_pct=0;
	u64 ofs=0, size;
	u64 total;
	struct timespec delay;
	int nb = (nbufs>0) ? nbufs : 256;
	ssize_t (*writeall_func)(int fd, void *buf, size_t count) = writeall;
	unsigned char *buf = alloca(nb*SECTOR_SIZE);

	if (!buf) {
		fprintf( stderr, "Couldn't allocate buffer: %d\n", 
			 nb*SECTOR_SIZE );
		exit(1);
	}

	if (demux_audio) {
		/* Init audio demuxer structures */
		tyda_init();
		writeall_func = demux_audio_writeall;
	}
	if (delayms > 0) {
		delay.tv_sec = delayms/1000;
		delayms %= 1000;
		delay.tv_nsec = delayms*1000000;
	}

	mfs_load_inode(fsid, &inode);
	size = mfs_fsid_size(fsid);
	if (start>size) {
		return -1;
	}
	if (start+count>size || count==0) {
		count = size-start;
	}
	total = count;

	if (verbose)
		fprintf(stderr,
			"exporting fsid %d of size %lld starting at offset %lld for %lld bytes\n", 
			fsid, size, start, count);
	mfs_readahead(1);

	// Special case for short files: data is in the inode itself.
	if(inode.num_runs == 0) {
		if (count > nb*SECTOR_SIZE) {
			fprintf( stderr, "inode data too big for buffer!?  Output truncated.\n" );
			count = nb*SECTOR_SIZE;
		}
		mfs_fsid_pread( fsid, buf, start, count );
		ret = (*writeall_func)(fd,buf,count); //number of BYTES read
		if (ret < 0 || ret != count) {
			perror("write failed:");
			return -1;
		}
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
			if (start > ofs) {
				// Find first sector we want
				int bofs = start-ofs;
				int sofs = bofs>>SECTOR_SHIFT;
				int bmod = bofs&(SECTOR_SIZE-1);
				ofs += bofs;
				len -= sofs;
				sec += sofs;
				// Partial first sector
				if (bmod) {
					blen = MIN(SECTOR_SIZE-bmod,count);
					mfs_read_sectors(buf,sec,1);
					ret = (*writeall_func)( fd, buf+bmod, blen);
					if (ret <= 0) {
						perror("write failed:");
						return -1;
					} else if (ret != blen) {
						fprintf(stderr,"short write: %d/%d\n", 
							ret, blen );
					}
					sec++;
					len--;
					count -= ret;
					ofs += blen;
				}
			}
			rlen = MIN(nb,len);  //number of sectors to read this round
			blen = MIN(rlen<<SECTOR_SHIFT,count);

			mfs_read_partial(buf, sec, blen);
			if (demux_audio)
			{
				ret = demux_audio_writeall(fd,buf,blen); //number of BYTES read
			}
			else
			{
				ret  = writeall(fd,buf,blen); //number of BYTES read
			}
			if (ret < 0 || ret != blen) {
				perror("write failed:");
				return -1;
			}
			ofs += ret;
			count -= ret;
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


int list_sectors_for_file(const u32 fsid,  run_desc runs[], int maxruns )
{
	struct mfs_inode inode;
	int run;
	u64 size;

	mfs_load_inode(fsid, &inode);
	size = mfs_fsid_size(fsid);

	// Special case for short files: data is in the inode itself.
	if ( inode.num_runs == 0 )
	{
		// We dont process short files ... not sure how short inode support would would with burst!
		fprintf(stderr, "LISTSECTORS was asked to list a short inode! Not supported.");
		return -1;
	}

	for ( run=0; run<inode.num_runs && run<maxruns; run++ )
	{
		runs[run] = mfs_list_sectors( inode.u.runs[run].start, inode.u.runs[run].len  );
	}
	return run;
}

