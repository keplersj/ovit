/*******************************************************************************
    ppchacker 01/18/2002
    The MFS interface for querying attributes returns only single value.
    This API returns the entire array of values.  If the value is TYPE_STRING,
    copies of the strings are made; they should be freed.
*******************************************************************************/

#include <assert.h>
#include "mfs.h"
#include "attribute.h"
#include "log.h"

int 
get_attribute( void *buf, unsigned size, const char *target_subobj_name, 
	       unsigned target_subobj_id, const char *target_attr_name, 
	       mfs_attribute_t *attribute)
{
	struct mfs_obj_header *obj;
	struct mfs_subobj_header *subobj;
	struct mfs_attr_header *attr;
	struct mfs_obj_attr *objattr;
	unsigned char *p, *q, *pend, *qend, *r, *rend;
	const char *subobj_name, *attr_name;
	unsigned *pint;
	int found;
	int len, alen;

	found = 0;

	obj = (struct mfs_obj_header *) buf;
	pend = (unsigned char *) buf + ntohl(obj->size);
	
	for(p = (unsigned char *) buf + sizeof(struct mfs_obj_header); p < pend && ! found; p += ntohs(subobj->len)) {
		subobj = (struct mfs_subobj_header *) p;
		subobj_name = schema_type(ntohs(subobj->obj_type));
		if (!subobj_name) /*JPB*/
			subobj_name = "";
		if (strcmp(subobj_name, target_subobj_name) != 0) {
			continue;
		}
		if (target_subobj_id != 0xffffffff && ntohl(subobj->id) != target_subobj_id) {
			continue;
		}

		qend = p + ntohs(subobj->len);
		for(q = p + sizeof(struct mfs_subobj_header); q < qend && ! found; q += alen) {
			attr = (struct mfs_attr_header *) q;
			attr_name = schema_attrib(ntohs(subobj->obj_type), attr->attr);
			alen = len = ntohs(attr->len);
			alen = (alen + 3) & ~3;

			if (strcmp(attr_name, target_attr_name) != 0) {
				continue;
			}

			/* this is the one! */
			found = 1;
			r = q + sizeof(struct mfs_attr_header);
			rend = r + len;
			attribute->type = attr->eltype >> 6;
			attribute->n = 0;

			switch (attribute->type) {
			case TYPE_STRING:
				while (r < rend-4) {
					assert(attribute->n < MFS_MAX_ARRAY_LEN);
					attribute->u.string[attribute->n++] = 
					  strcpy(malloc(strlen((char*)r) + 1), (char*)r);
					r += strlen((char*)r) + 1;
				}
				break;

			case TYPE_INT:
			case TYPE_FILE:
				while (r < rend-4) {
					assert(attribute->n < MFS_MAX_ARRAY_LEN);
					pint = (unsigned int *) r;
					attribute->u.integer[attribute->n++] = ntohl(*pint);
					r += sizeof(unsigned);
				}
				break;

			case TYPE_OBJECT:
				while (r < rend-4) {
					assert(attribute->n < MFS_MAX_ARRAY_LEN);
					objattr = (struct mfs_obj_attr *) r;
					attribute->u.object[attribute->n].fsid = ntohl(objattr->fsid);
					attribute->u.object[attribute->n].subobj = ntohl(objattr->subobj);
					attribute->n++;
					r += sizeof(struct mfs_obj_attr);
				}
				break;
			}
		}
	}

	return found;
}

int
get_attribute_fsid( int fsid, const char *target_subobj_name, 
		    unsigned target_subobj_id,  
		    const char *target_attr_name, 
		    mfs_attribute_t *attribute) 
{
  void *buf;
  unsigned size;
  int found;

  if (mfs_fsid_type(fsid) != MFS_TYPE_OBJ) {
    fprintf(stderr, "%d is not an object\n", fsid);
    exit(1);
  }
  size = mfs_fsid_size(fsid);
  buf = malloc(size);
  mfs_fsid_pread(fsid, buf, 0, size);
  found = get_attribute( buf, size, target_subobj_name, target_subobj_id, target_attr_name, attribute );
  free(buf);
  return found;
}

void 
attr_release(mfs_attribute_t *attr)
{
	if (attr->type == TYPE_STRING) {
		int i;

		for (i = 0; i < attr->n; ++i)
			free(attr->u.string[i]);
	}
}

// caller must free result
char *
get_tivo_version()
{
	int		fsid = mfs_resolve("/SwSystem/ACTIVE");
	mfs_attribute_t	attr;
	char *retval = 0;

	if (!fsid ||
	    !get_attribute_fsid(fsid, "SwSystem", 0xffffffff, "Name", &attr) ||
	    attr.type != TYPE_STRING) {
		fprintf(stderr, "Can't find TiVo version!\n");
		return strdup("UNKNOWN");
	}
	retval = strdup(attr.u.string[0]);
	attr_release(&attr);
	return retval;
}

