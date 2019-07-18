/*******************************************************************************
    ppchacker 01/18/2002
    The MFS interface for querying attributes returns only single value.
    This API returns the entire array of values.  If the value is TYPE_STRING,
    copies of the strings are made; they should be freed.
*******************************************************************************/
typedef struct attribute_struct {
	int type;
	int n;
	union {
		char *string[MFS_MAX_ARRAY_LEN];
		int integer[MFS_MAX_ARRAY_LEN];
		struct mfs_obj_attr object[MFS_MAX_ARRAY_LEN];
	} u;
} mfs_attribute_t;


int 
get_attribute( void *buf, unsigned size, const char *target_subobj_name, 
	       unsigned target_subobj_id, const char *target_attr_name, 
	       mfs_attribute_t *attribute);


int
get_attribute_fsid( int fsid, const char *target_subobj_name, 
		    unsigned target_subobj_id,  
		    const char *target_attr_name, 
		    mfs_attribute_t *attribute);

void 
attr_release(mfs_attribute_t *attr);

char *get_tivo_version();

