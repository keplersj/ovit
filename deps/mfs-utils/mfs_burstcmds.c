/*
  media-filesystem export file sectors for S1 cachecard burst transfers
  JB6783/Jamie   May 2005
  released under the Gnu GPL v2
*/
#include <assert.h>
#include <string.h>
#include <stdarg.h>
#include <stdio.h>

#include <sys/ioctl.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <net/if.h>
#include <netdb.h>
#include <netinet/in.h>
#include <arpa/inet.h>

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
	-h		Display this usage info.\n\
        -i <ip>         IP address for burst mode commands\n\
	-o <path>	Write output to the specified file.\n\
        -R <path|fsid>  Export the parts of a recording FSID.\n\
\n\
   Output defaults to stdout if not present on the command line.\n\
", prog );
	credits();
	exit(1);
}

static char *get_ipaddr()
{
  int                sfd;
  struct ifreq       ifr;
  struct sockaddr_in *sin = (struct sockaddr_in *) &ifr.ifr_addr;

  memset(&ifr, 0, sizeof ifr);

  if (0 > (sfd = socket(AF_INET, SOCK_STREAM, 0))) {
    perror("socket()");
    return 0;
  }

  strcpy(ifr.ifr_name, "eth0");
  sin->sin_family = AF_INET;

  if (0 == ioctl(sfd, SIOCGIFADDR, &ifr)) {
	  return inet_ntoa(sin->sin_addr);
  }

  return 0;
}

int main(int argc, char *argv[])
{
	int fd = STDOUT_FILENO;
	FILE *file = 0;
	int c;
	int append = 0;
	char *output = 0;
	char *ipaddr = 0;
	int *parts = 0;
	char *showPath = 0;
	int i,n,skip;
	run_desc runs[1024];
	const int max_runs = sizeof(runs)/sizeof(runs[0]);

	prog = argv[0];
	while ((c = getopt(argc, argv, "aho:R:v")) != -1 ){
		switch (c) {
		case 'a':
			append=1;
			break;

		case 'h':
			usage();
			break;

		case 'i':
			ipaddr = strdupa(optarg);
			break;

		case 'o':
			output = strdupa(optarg);
			break;

		case 'R':
			showPath = strdupa(optarg);
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
	if (showPath && argc!=0) {
		usage();
		fprintf( stderr, "%s Command line error: The -R option only makes sense with a single path/fsid\n", 
			 prog );
		exit(1);
	}
	
        if (output) {
		/* output contains a file name */
		int open_flags = O_WRONLY|O_CREAT|O_LARGEFILE;
		open_flags |= append ? O_APPEND : O_TRUNC;
		fd = open(output, open_flags, 0644);
		if (fd == -1) {
                        if (errno)
                                perror(output);
                        exit(1);
                }
        } else {
                if (append)
                        fprintf(stderr, "WARNING: -a makes no sense without -o\n");
        }

	if (!ipaddr) {
		//  Look for MFS_DEVLIST=":host"
		char *p = getenv("MFS_DEVLIST");
		if (p && p[0] == ':')
			ipaddr = p+1;
		else
			ipaddr = get_ipaddr();
	}

	mfs_init();

	if (showPath) {
		generate_xml(showPath);
		parts = get_parts();
	}
	if (parts == 0) {
		parts = (int *) malloc( (argc+1)*sizeof(int) );
		assert(parts);
		for(i=0; i<argc; i++)
			parts[i] = mfs_resolve(argv[i]);
		parts[argc] = -1;
	}

	// Get all the run information
	n=0;
	for(i=0; parts[i] >=0; i++) {
		int j = list_sectors_for_file( parts[i], runs+n, max_runs-n );
		if (j < 0) {
			fprintf( stderr, "list_sectors_for_file failed on FSID: %d", parts[i] );
			break;
		}
		n += j;
	}
	
	// See if any runs can be merged.
	skip = 0;
	for(i=1; i<n; i++) {
		if (runs[i].drive     == runs[i-1-skip].drive     &&
		    runs[i].partition == runs[i-1-skip].partition &&
		    runs[i].start     == runs[i-1-skip].start + runs[i-1-skip].count) {
			runs[i-1-skip].count += runs[i].count;
			skip++;
		} else if (skip != 0)
			runs[i-skip] = runs[i];
		    
	}
	n -= skip;
	
	// print the burst commands
	file = fdopen( fd, "w" );
	for(i=0; i<n; i++) {
		fprintf ( file, "burst //%s/%d:%d:%d-%d part%d.ty\n",
			  ipaddr, runs[i].drive, runs[i].partition,
			  (int)runs[i].start, (int)(runs[i].start + runs[i].count), i );
	}
	if (n > 0) {
		fprintf( file, "copy /b part0.ty");
		for(i=1; i<n; i++)
			fprintf( file, " + /b part%d.ty", i );
		fprintf( file, " stream.ty\n" );
	}
	fclose(file);

	close(fd);
	exit(0);
}

