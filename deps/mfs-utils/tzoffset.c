/*
  media-filesystem retreive timezone info
  Jamie@DDB
  released under the Gnu GPL v2

  This version only works for TiVo software versions 4.X and above.  See 
  tzoffset.tcl from the mfs_ftp package if you are inspired to make it work
  with older software versions.
*/
#include "mfs.h"
#include <time.h>
#include <limits.h>
#include <string.h>

// Currently only deals with software versions 3.X and above
int tzoffset() 
{
	static int tzoff = INT_MAX;
	int tzOld = 0;
	int dstOld = 0;
	int useDst = 0;
	int fsid;

	void callback(int fsid, struct mfs_subobj_header *obj,
			     struct mfs_attr_header *attr, void *data)
	{
		if(obj && attr) {
			if (obj->obj_type == 136) { /* LocationConfig object */
				if (attr->attr == 19) /* TimeZoneOffset */
					tzoff = ntohl(*(int *)data);
				else if (attr->attr == 17) /* TimeZoneOldOBSOLETE */
					tzOld = ntohl(*(int *)data);
				else if (attr->attr == 18)  /* DaylightSavingsPolicyOBSOLETE */
					dstOld = ntohl(*(int *)data);
				else if (attr->attr == 21) /* "UseDaylightSavings" */
					useDst = ntohl(*(int *)data);
			} else if (obj->obj_type == 43) { /* Setup object */
				if (attr->attr == 18) /* TimeZoneOBSOLETE */
					tzOld = ntohl(*(int *)data);
				else if (attr->attr == 19) /* DaylightSavingsPolicyOBSOLETE */
					dstOld = ntohl(*(int *)data);
			}
		}
	}

	if (tzoff != INT_MAX) return tzoff;
	fsid = mfs_resolve( "/State/LocationConfig" );
	if (fsid > 0) {
		u32 size = mfs_fsid_size(fsid);
		char *buf=alloca(size);
		mfs_fsid_pread(fsid, buf, 0, size );
		parse_object(fsid, buf, callback);
	} else {
	  fsid = mfs_resolve( "/Setup" );
	  if (fsid > 0) {
		  u32 size = mfs_fsid_size(fsid);
		  char *buf=alloca(size);
		  mfs_fsid_pread(fsid, buf, 0, size );
		  parse_object(fsid, buf, callback);
	  }
	  
	}
	if (tzoff == INT_MAX) {
		/*  Try old formats  (version 3.X) */
		int tz = tzOld;
		static short tzlist[] = {-5,-6,-7,-8,-9,-10,0,1,2,3,4,5,6,7,8,9,10,11,12,-1,-2,-3,-4,-11,-12};
		static size_t l = sizeof(tzlist)/sizeof(tzlist[0]);
		if (tzOld>0 && tzOld <= l)
			tz = tzlist[tzOld];
		tzoff = tz*60*60; /* convert to seconds */
	}

	// See if DST applies;
	if (useDst == 2 || (useDst==INT_MAX && 
			    (dstOld==2 || dstOld==INT_MAX))) {
		time_t secs = time(0);
		struct tm *tm = gmtime(&secs);
		int month=tm->tm_mon+1, 
			dom=tm->tm_mday, 
			dow=tm->tm_wday, 
			hour=tm->tm_hour;
		/**  add an hour for DST if needed.  
		     Based on rules from NowShowing.tcl (USA bias?) */
		if ((month > 4 && month < 10) ||
		    (month == 4 && dom > 7) ||
		    (month == 4 && dom <= 7 && dow == 0 && hour >= 2) ||
		    (month == 4 && dom <= 7 && dow != 0 && (dom-dow > 0)) ||
		    (month == 10 && dom < 25) ||
		    (month == 10 && dom >= 25 && dow == 0 && hour < 2) ||
		    (month == 10 && dom >= 25 && dow != 0 && (dom-24-dow < 1) ))
			tzoff += 60*60;
	}
	return tzoff;
}
