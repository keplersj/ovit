/*
  media-filesystem object dump
  tridge@samba.org, January 2001
  released under the Gnu GPL v2
*/

#include "mfs.h"

static int pull_files;
static int num_vids;
static int vid_fsid[1000];

static void dump_video(int fsid)
{
	int vid;

	printf("VideoClip/1/%s/%d {\n",
	       query_string(fsid, "ServerId"),
	       query_int(fsid, "ServerVersion"));
	printf("\tName: {%s}\n", 
	       query_string(fsid, "Name"));
	vid = query_int(fsid, "File");
	printf("\tFile: File of size %d/131072\n",
	       (int)(mfs_fsid_size(vid)/131072));
	printf("}\n\n");
	if (pull_files) {
		int fd;
		char *dest = query_string(fsid, "Name");
		if ( (fd = open(dest, O_WRONLY|O_CREAT|O_TRUNC|O_LARGEFILE, 0644)) <0)
			perror(dest);
		else 
			export_file(vid, fd , 0, 0, -1, 256, 0, 0 );
		close(fd);
	}
}

static void getvids(int fsid, const char *name)
{
	struct mfs_obj_attr *obj;
	int count, i;

	obj = query_object(fsid, name, &count);
	if (!obj || obj[0].fsid == 0) {
//		obj = query_object(fsid, name, &count);
	}
	if (!obj || obj[0].fsid == 0) return;

	for (i=0;i<count;i++) {
		char *path;
		int count2;
		struct mfs_obj_attr *vobj;
		asprintf(&path, "%s/%d/VideoClip", name, obj[i].subobj);
		vobj = query_object(fsid, path, &count2);
		vid_fsid[num_vids++] = vobj->fsid;
		free(path);
		free(vobj);
	}
	free(obj);
}

static void gethead(int fsid, const char *name)
{
	struct mfs_obj_attr *obj;
	int count, i;

	obj = query_object(fsid, name, &count);
	if (!obj || obj[0].fsid == 0) return;

	printf("\t%s: ", name);
	for (i=0;i<count;i++) {
		printf("LoopSetClip/%d ", obj[i].subobj);
	}
	printf("\n");
	free(obj);
}

static void getstate(int fsid, const char *name)
{
	struct mfs_obj_attr *obj;
	int count, i;

	obj = query_object(fsid, name, &count);
	if (!obj || obj[0].fsid == 0) return;

	for (i=0;i<count;i++) {
		char *path1, *path2, *path3;
		int vid, count2;
		struct mfs_obj_attr *vobj;

		asprintf(&path1, "%s/%d/Entrance", name, obj[i].subobj);
		asprintf(&path2, "%s/%d/Exit", name, obj[i].subobj);
		asprintf(&path3, "%s/%d/VideoClip", name, obj[i].subobj);
		printf("\tSubrecord LoopSetClip/%d {\n",
		       obj[i].subobj);
		printf("\t\tEntrance: %d\n", query_int(fsid, path1));
		printf("\t\tExit: %d\n", query_int(fsid, path2));
		
		vobj = query_object(fsid, path3, &count2);
		vid = vobj->fsid;

		vid_fsid[num_vids++] = vid;

		printf("\t\tVideoClip: VideoClip/1/%s\n", 
		       query_string(vid, "ServerId"));
		printf("\t}\n");
		free(path1);
		free(path2);
		free(path3);
		free(vobj);
	}
	printf("\n");
	free(obj);
}

static void getslice(int fsid)
{
	int i;

	if (mfs_fsid_type(fsid) != MFS_TYPE_OBJ) {
		fprintf(stderr,"%d is not an object\n", fsid);
		exit(1);
	}

	printf("Guide type=3\n\n");

	getvids(fsid, "State");
	getvids(fsid, "Trans");

//	fprintf(stderr,"num_vids=%d\n", num_vids);

	for (i=0;i<num_vids;i++) {
		if (i==0 || vid_fsid[i] != vid_fsid[i-1]) {
			dump_video(vid_fsid[i]);
		}
	}

	printf("LoopSet/1/%s/%d {\n",
	       query_string(fsid, "ServerId"),
	       query_int(fsid, "ServerVersion"));
	printf("\tName: {%s}\n", query_string(fsid, "Name"));
	gethead(fsid, "State");
	gethead(fsid, "Trans");
	getstate(fsid, "State");
	getstate(fsid, "Trans");
	printf("}\n\n");
}


static void usage(void)
{
	printf("\n"
"usage: mfs_getslice [options] <path|fsid>\n"
);
	credits();
	exit(1);
}


 int main(int argc, char *argv[])
{
	int fsid;
	int c;

	while ((c = getopt(argc, argv, "f")) != -1 ){
		switch (c) {
		case 'f':
			pull_files = 1;
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
	getslice(fsid);

	return 0;
}
