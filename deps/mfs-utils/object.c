/*
  media-filesystem object parse code
  tridge@samba.org, January 2001
  released under the Gnu GPL v2
*/

#include "mfs.h"

static int parse_attr(char *p, int obj_type, int fsid, 
		      struct mfs_subobj_header *obj, object_fn fn)
{
	struct mfs_attr_header *attr;
	int ret;

	attr = (struct mfs_attr_header *)p;
	attr->len = ntohs(attr->len);

	p += sizeof(*attr);

	fn(fsid, obj, attr, p);

	ret = (attr->len+3)&~3;
	attr->len = htons(attr->len);
	return ret;
}

static void parse_subobj(void *p, u16 type, int len, int fsid,
			 struct mfs_subobj_header *obj, object_fn fn)
{
	int ofs=0;
	while (ofs < len) {
		ofs += parse_attr(p+ofs, type, fsid, obj, fn);
	}
}

/* this is the low-level interface to parsing an object. It will call fn() on
   all elements in all subobjects */
void parse_object(int fsid, void *buf, object_fn fn)
{
	char *p;
	u32 ofs;
	struct mfs_obj_header *obj = buf;
	int i=0;

	byte_swap(obj, "i2");

	p = buf;
	ofs = sizeof(*obj);

	/* now the subobjects */
	while (ofs < obj->size) {
		struct mfs_subobj_header *subobj = buf+ofs;
		byte_swap(subobj, "s6 i1");
		fn(fsid, subobj, NULL, NULL);
		parse_subobj(buf+ofs+sizeof(*subobj), 
			     subobj->obj_type,
			     subobj->len-sizeof(*subobj), fsid, subobj, fn);
		ofs += subobj->len;
		i++;
		byte_swap(subobj, "s6 i1");
	}

	byte_swap(obj, "i2");
}

/* give a string for a obj type */
char *object_typestr(int objtype)
{
	switch (objtype) {
	case TYPE_INT: return "int";
	case TYPE_STRING: return "string";
	case TYPE_OBJECT: return "object";
	case TYPE_FILE: return "file";
	}
	return "UNKNOWN";
}
