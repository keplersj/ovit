/*
  media-filesystem object query code
  tridge@samba.org, January 2001
  released under the Gnu GPL v2
*/
#include <time.h>
#include "mfs.h"

static struct {
	int fsid;
	void *buf;
	int size;
} loaded = {0};

static void load_object(int fsid)
{
	if (fsid == loaded.fsid) return;

	if (loaded.buf) free(loaded.buf);
	loaded.fsid = fsid;
	loaded.size = mfs_fsid_size(fsid);
	loaded.buf = malloc(loaded.size);
	mfs_fsid_pread(fsid, loaded.buf, 0, loaded.size);
}


/* return the data portion of a part of an object */
void *query_part(int fsid, int subobj, const char *name, int *len)
{
	void *ret = NULL;
	void callback(int fsid, struct mfs_subobj_header *obj,
			     struct mfs_attr_header *attr, void *data)
	{
		if (!attr) return;
		if (!(obj->flags && subobj == -1) && !(obj->id == subobj)) return;
		if (strcmp(schema_attrib(obj->obj_type, attr->attr), name)) return;
		*len = attr->len;
		ret = data;
		// fprintf(stderr, "len=%d ret=%*.*s\n", *len, *len, *len, ret);
	}

	if (mfs_fsid_type(fsid) != MFS_TYPE_OBJ) {
		fprintf(stderr,"%d is not an object\n", fsid);
		exit(1);
	}

	load_object(fsid);
	parse_object(fsid, loaded.buf, callback);
	return ret;
}

/* query a subobject path starting at fsid returning the data in the
   tail of the path */
static int query_subobj_path(int fsid, int subobj, const char *path, int *len, 
		       void *ret[], const int max)
{
	int i=0;
	int idx = 0;
	const char *tok, *end;
	if (max <=0) return 0;
	
	end = strchr(path,'/');
	tok = (end) ? strndupa(path,end-path) : path;

	ret[idx] = query_part(fsid, subobj, tok, &len[idx]);
	if (!ret[idx]) return idx;
	if (end) {
		struct mfs_obj_attr *objattr = ret[idx];
		int count = (len[idx]-4)/sizeof(*objattr);
		for (i=0;i<count;i++) {
			int n;
			fsid = ntohl(objattr->fsid);
			subobj = ntohl(objattr->subobj);
			n = query_subobj_path( 
				fsid, subobj, end+1, len+idx,
				ret+idx, max-idx );
			idx += n;
			objattr++;
		}
	} else
		idx++;
	return idx;
}

char *query_string(int fsid, char *path)
{
	int len = 0;
	void *p = {0};
	int n = query_subobj_path(fsid, -1, path, &len, &p, 1);
	return (n==1) ? (char *)p : 0;
}

int query_int(int fsid, char *path)
{
	int len = 0;
	void *p = 0;
	int n = query_subobj_path(fsid, -1, path, &len, &p, 1);
	return (n==1) ? ntohl(*((int*)p)) : 0;
}

int query_int_list(int fsid, char *path, int ret[], int max)
{
	int i;
	int *len = alloca( max*sizeof(int) );
	void **p = alloca( max*sizeof(void *) );
	int n=query_subobj_path(fsid, -1, path, len, p, max);
	for(i=0; i<n; i++)
		ret[i] = ntohl(*((int*)p[i]));
	return n;
}

struct mfs_obj_attr *query_object(int fsid, const char *path, int *count)
{
	int len, i;
	struct mfs_obj_attr *ret = NULL;
	struct mfs_obj_attr *p;
	void *pvoid;
	int n = query_subobj_path(fsid, -1, path, &len, &pvoid, 1);
	if (n<=0 || !pvoid) return ret;
	p = (struct mfs_obj_attr*) pvoid;
	*count = (len-4)/8;
	ret = calloc(*count, sizeof(*ret));
	for (i=0;i<*count;i++) {
		ret[i] = p[i];
		ret[i].fsid = ntohl(ret[i].fsid);
		ret[i].subobj = ntohl(ret[i].subobj);
	}
	return ret;
}

#define MAX_PARTS 1024
void query_streams(const char *path)
{
	struct mfs_dirent *dir;
	int parts[MAX_PARTS];
	u32 count, i, j;
	const char *ep;

	dir = mfs_dir(mfs_resolve(path), &count);
	for (i=0;i<count;i++) {
		int n = query_int_list(dir[i].fsid, "Part/File", parts, MAX_PARTS );
		fprintf(stdout, "%d\t", dir[i].fsid );
		fprintf(stdout,"%s\t", 
		       query_string(dir[i].fsid, "Showing/Program/Title"));
		ep = query_string(dir[i].fsid, "Showing/Program/EpisodeTitle");
		fprintf(stdout, "%s", ep?ep:"" );
		for(j=0;j<n; j++)
			fprintf(stdout, "\t%d", parts[j] );
		fprintf(stdout,"\n");
	}
	if (dir) mfs_dir_free(dir);
}
