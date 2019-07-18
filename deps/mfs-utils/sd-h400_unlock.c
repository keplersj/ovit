/*
  media-filesystem capacity unlock for Toshiba SD-H400
  jamie@xmission.com, Oct 2004

  Based on mfs_dumpobj.c
  tridge@samba.org, January 2001
  released under the Gnu GPL v2

*/

#include "mfs.h"

static int tivoclipKB = 10000000;
static int userKB = -1;
static int hits = 0;

static void unlock_callback(int fsid, struct mfs_subobj_header *obj,
			    struct mfs_attr_header *attr, void *data)
{
	int i;
	char *p = data;
	static int last;
	struct mfs_obj_attr *objattr;
	static const char *lasttype;
	static int lastid;

	if (!attr) {
		if (last) printf("}\n");
		last = fsid;
		printf("%s %d/%d %s{\n",
		       schema_type(obj->obj_type), fsid, obj->id,
		       obj->flags?"PRIMARY ":"");
		lasttype=schema_type(obj->obj_type);
		return;
	}
	const char *attrstr=schema_attrib(obj->obj_type,attr->attr);
	if (attr) {
		printf("\t%s[%d]=", attrstr, attr->attr);
	} else {
		printf("\t[%d]=", attr->attr);
	}
        switch (attr->eltype>>6) {
        case TYPE_STRING:
                for (i=0;i<attr->len-4;) {
                        char *s = (char *)&p[i];
                        printf("%s ", s);
                        i += strlen(s)+1;
                }
                break;
        case TYPE_INT:
        case TYPE_FILE:
		for (i=0;i<(attr->len-4)/4;i++) {
			printf("%d ",(int)ntohl(*(int *)&p[i*4]));
		}
		if (strcmp(lasttype,"DiskPartition")==0) {
			if (attrstr != 0 && strcmp(attrstr,"Id")==0) {
				lastid = ntohl(*(int *)p);
				printf( "  (%s)", lastid==11 ? "TivoClips" : "User" );
			} else if (attrstr != 0 && 
				   strcmp(attrstr,"SizeInKb")==0 &&
				   (lastid == 10 || lastid == 11)) {
				int oldsize = ntohl(*(int *)p);
				int newsize = (lastid == 11) ? tivoclipKB : userKB;
				hits++;
				if (oldsize != newsize) {
					*(int *)p = htonl(newsize);
					printf("\n**new**\t%s[%d]=%d ", 
					       attrstr, attr->attr, newsize );
				}
			}
		}
                break;
        case TYPE_OBJECT:
		objattr = (struct mfs_obj_attr *)p;
                for (i=0;i<(attr->len-4)/sizeof(*objattr);i++) {
                        printf("%d/%d ",
			       (int)ntohl(objattr->fsid),
                               (int)ntohl(objattr->subobj));
			objattr++;
                }
                break;
        }
	printf("\n");
}

static void unlock_obj(int fsid, int test)
{
	void *buf;
	u32 size;
	if (mfs_fsid_type(fsid) != MFS_TYPE_OBJ) {
		fprintf(stderr,"%d is not an object\n", fsid);
		exit(1);
	}
	size = mfs_fsid_size(fsid);
	buf = malloc(size);
	mfs_fsid_pread(fsid, buf, 0, size);
	parse_object(fsid, buf, unlock_callback);
	if (hits == 2 && !test) {
		mfs_fsid_pwrite(fsid, buf, 0, size);
	}
	free(buf);
	printf("}\n");
}


static char *prog;
static void usage(void)
{
	printf(
		"\n\
Usage: %s [-p path] [-c tivoclipsKB] [-u userKB] [-w] [device]\n\
\n\
  This program unlocks the 80 hour lock on a Toshiba SD-H400 TiVo by modifying\n\
  the Active DiskConfiguration object in the Media File System (MFS).\n\
\n\
  The device should be a device file such as /dev/hdc.  If not present, the\n\
  vplay MFS_DEVLIST environment variable is used.\n\
\n\
  The default is to modify \"/Config/DiskConfigurations/Active\" and \n\
  set the TiVoClips size to 10000000 and the User size to -1.  This reserves \n\
  10 million K bytes for tivo clips (ads), and expands the  user area to fill \n\
  the remaining  available space.\n\
\n\
  Without the -w option, the program is running in test mode: it will show you\n\
  the changes it would make, but it won't write them to disk.\n\
\n\
jamie@xmission.com, Oct 2004\n\
based on mfs_dumpobj by tridge@samba.org, January 2001\n\
released under the Gnu GPL v2\n\
", prog
		);
	credits();
	exit(1);
}


// arguments are concatenated to form MFS_DEVLIST
//  memory for the return argument is malloc'd but not freed
static char *cat_args( int argc, char *argv[]) {
	int i;
	int tlen=0;
	char *res, *p;

	for(i=0; i<argc; i++)
		tlen += strlen(argv[i]) + 1;

	p = res = malloc(tlen);
	if (!res) return res;

	res[0] = 0;
	for(i=0; i<argc; i++) {
		strcat( p, argv[i] );
		p += strlen(argv[i]);
		if (i != argc-1) {
		  *p++ = ' ';
		  *p = 0;
		}
	}
	return res;
}

int main(int argc, char *argv[])
{
	int fsid;
	int c;
	prog = argv[0];
	char *path = "/Config/DiskConfigurations/Active";
	int test=1;		/* true for test mode: nowrite  */
	char *dev=0;
	
	while ((c = getopt(argc, argv, "u:c:p:wh")) != -1 ){
		switch (c) {
		case 'c':
			tivoclipKB = atoi(optarg);
			break;
		case 'h':
			usage();
			break;
		case 'p':
			path  = optarg;
			break;
		case 'w':
			test=0;
			break;
		case 'u':
			userKB = atoi(optarg);
			break;
		default:
			usage();
		}
	}

	argc -= optind;
	argv += optind;
	
	if (argc > 0)
		dev = cat_args( argc, argv );
	
	if ((tivoclipKB<0 && userKB<0))
		usage();

	mfs_init_dev(dev);
	fsid = mfs_resolve(path);
	if (fsid == 0) {
		fprintf( stderr, "\n\
  MFS Path \"%s\" does not seem to be valid.\n\
  This disk does not appear to have a  capacity lock that we can remove.\n", path );
		exit(1);
	}
	unlock_obj(fsid,test);


	if (hits==2) {
		printf( "Success! (%scommitted)\n", test?"not ":"");
	} else
		printf( "Failure: the DiskConfigurations object didn't match the expected structure\n");
	return 0;
}
