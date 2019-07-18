/*
  media-filesystem export utility
  tridge@samba.org, January 2001
  released under the Gnu GPL v2
*/
#include <assert.h>
#include <string.h>
#include <stdarg.h>
#include <stdio.h>

#include "mfs.h"
#include "tar.h"


static char *prog="";
static void usage(void)
{
	fprintf(stderr,"\n\
usage: %s [options] [<path|fsid> ...]\n\
\n\
   options:\n\
	-a		Open output file for appending (requires [-o path])\n\
	-c <count>      # of bytes to copy (defaults to allocated size of fsid)\n\
	-d		Demux\'s audio on the fly instead of exporting TY mux\n\
	-h		Display this usage info.\n\
	-n <count>	# of sectors to use for buffering.  Defaults to 256\n\
	-o <path|address:port>	Write output to the specified file or to a\n\
				TCP connection to the specified host and port\n\
	-p <priority>	Priority  0: ts; 1-99 RT FIFO\n\
	-r <ms>		Rate control (throttle)\n\
				-'ve  : no delay (default)\n\
				0     : sched_yield() between chunks\n\
				+'ve  : # of ms to delay between chunks\n\
        -R <path|fsid>  Export the parts of a recording FSID.\n\
	-s <start>	Start offset, in bytes (defaults to 0)\n\
	-t		tar/tmf output\n\
	-x		Include showing.xml.  With -R, computed internally,\n\
                          without -R, read from standard in.\n\
        -X              Output only showing.xml (requires -R).\n\
	-v		Verbose progress messages on stderr\n\
\n\
   Output defaults to stdout if not present on the command line.\n\
", prog );
	credits();
	exit(1);
}

static int verb=0;
static void verbose(const char *fmt, ...) {
	va_list ap;
	if (!verb) return;
	va_start(ap,fmt);
	vfprintf( stderr, fmt, ap );
	va_end(ap);
}

int main(int argc, char *argv[])
{
	int fd = STDOUT_FILENO;
	int fsid;
	int c;
	u64 start=0;
	u64 count=0;
	int rate = -1;
	u32 nbufs = 256;	/* number of sectors to use for I/O buffering  */
	int append = 0;
  int demux_audio = 0;
  int tar=0;
	int xml = 0;
	int xmlonly = 0;
	char *output = 0;
	int *parts = 0;
	char *showPath = 0;
	char *xml_buf=0;
	int xml_len=0;
	tar_record tar_header;
	int i;

	prog = argv[0];
	while ((c = getopt(argc, argv, "ac:dhn:o:p:r:R:s:txXv")) != -1 ){
		switch (c) {
		case 'a':
			append=1;
			break;

		case 'c':
			count = strtoll(optarg, NULL, 0);
			break;			

		case 'd':
		  demux_audio = 1;
		  break;

		case 'h':
			usage();
			break;

		case 'n':
			nbufs = strtoll(optarg, NULL, 0);
			break;

		case 'o':
			output = strdupa(optarg);
			break;

		case 'p':
			fixPriority(strtoll(optarg, NULL, 0));
			break;

		case 'r':
			rate = strtoll(optarg, NULL, 0);
			break;

		case 'R':
			showPath = strdupa(optarg);
			break;

		case 's':
			start = strtoll(optarg, NULL, 0);
			break;

		case 't':
			tar = 1;
			break;
			
		case 'x':
			xml = 1;
			break;

		case 'X':
			xmlonly = 1;
			break;

		case 'v':
			verb = 1;
			break;

		default:
			usage();
		}
	}

	argc -= optind;
	argv += optind;

	// Sanity check args
	if (!showPath && argc <1) {
		usage();
		fprintf( stderr, "%s Command line error: The command line must include a path or fsid.\n", 
			 prog );
		exit(1);
	}
	if ( xmlonly && !showPath ) {
		usage();
		fprintf( stderr, "%s Command line error: The -X option only makes sense when combined with -R\n", 
			 prog );
		exit(1);
	}
	if (showPath && argc!=0) {
		usage();
		fprintf( stderr, "%s Command line error: The -R option only makes sense with a single path/fsid\n", 
			 prog );
		exit(1);
	}
	if ( (start!=0 || count!=0) && (showPath || argc>1) ) {
		usage();
		fprintf( stderr, "%s Command line error: The -s and -c option are only supported with a single path/fsid\n", 
			 prog );
		exit(1);
		
	}
	
        if (output) {
                char *portstr;
                portstr = strchr(output, ':');
                if (!portstr) {
                        /* output contains a file name */
                        int open_flags = O_WRONLY|O_CREAT|O_LARGEFILE;
                        open_flags |= append ? O_APPEND : O_TRUNC;
                        fd = open(output, open_flags, 0644);
                } else {
                        /* output contains an ip_address:port */
                        *portstr = 0;   // null terminate the ip address in output
                        ++portstr;
                        verbose("opening socket to %s on %s\n", output, portstr);  fflush(stderr); // DEBUG
                        fd = open_socket_out(output, atoi(portstr));
                        verbose("socket opened %d\n", fd);  fflush(stderr); // DEBUG
                }
                if (fd == -1) {
                        if (errno)
                                perror(output);
                        exit(1);
                }
        } else {
                if (append)
                        fprintf(stderr, "WARNING: -a makes no sense without -o\n");
        }

	mfs_init();

	if (xml || xmlonly) {
		verbose("getting showing.xml contents\n");
		xml_buf = showPath ? generate_xml(showPath) : read_xml();
		xml_len = xml_buf ? strlen(xml_buf) : 0;
		if (xml_len == 0) {
			fprintf( stderr, "%s Error getting showing.xml\n", prog );
			exit(1);
		}
		if (tar) {
			create_tarheader(&tar_header, "showing.xml", xml_len);
			write(fd, &tar_header, sizeof(tar_header));
			write(fd,xml_buf,xml_len);
			free(xml_buf);
			write_tar_padding(fd, xml_len);
		}
	}
	if (showPath) {
		if (xml_buf == 0) generate_xml(showPath);
		parts = get_parts();
	}
	if (parts == 0) {
		parts = (int *) malloc( (argc+1)*sizeof(int) );
		assert(parts);
		for(i=0; i<argc; i++)
			parts[i] = mfs_resolve(argv[i]);
		parts[argc] = -1;
	}

	if (!xmlonly) {
		for( i = 0; parts[i] != -1; i++)
		{
			char buffer[64];
			u64 size = 0;

			fsid = parts[i];

			if (tar) {

				/* Create and write tar header for each part in seq */
				snprintf(buffer, sizeof(buffer),"part%02d.ty", i);
				size = mfs_fsid_size(fsid);
				create_tarheader(&tar_header, buffer, size);
				write(fd, &tar_header, sizeof(tar_header));
			}

			export_file(fsid,fd, start, count, rate, nbufs, verb, demux_audio);
		
			if (tar)
				/* Write out slack bytes */
				write_tar_padding(fd, size); 
		}

		if (tar) {
			/* Write out the "EOF" header block */
			memset(&tar_header, 0, sizeof(tar_header));
			write(fd, &tar_header, sizeof(tar_header));
		}
	}
	if (!tar && (xml || xmlonly)) {
		const char *typlusdelim = "################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################";
		const size_t l = strlen(typlusdelim);
		if (!xmlonly)
			write(fd,typlusdelim,l);
		write(fd,xml_buf,xml_len);
		if (!xmlonly)
			write(fd,typlusdelim,l);
		free(xml_buf);
	}

	close(fd);
	exit(0);
}

