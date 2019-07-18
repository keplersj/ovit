/*
  Generate NowShowing list for TyTool
  jamie@xmission.com  Dec 2004

  Part of the vplay distribution  Media-filesystem export utility
  tridge@samba.org, January 2001
  released under the Gnu GPL v2

*/

#include <time.h>

#include "mfs.h"
#include "attribute.h"
#include "log.h"

/** 
 *  Fetch an fsid, if it isn't a subobject of the fsid we already have 
 */
static int
get_object( int fsid, void *buf, unsigned size, int newfsid, 
	    void **bufout, unsigned *sizeout) {
	if (fsid == newfsid) {
		*bufout = buf;
		*sizeout = size;
	} else {
		if (newfsid <= 0) {
			fprintf( stderr, "%d is not a valid fsid\n", newfsid );
			return 0;
		}
		if (mfs_fsid_type(newfsid) != MFS_TYPE_OBJ) {
			fprintf(stderr, "%d is not an object\n", newfsid);
			return 0;
		}
		*sizeout = mfs_fsid_size(newfsid);
		*bufout = malloc(*sizeout);
		mfs_fsid_pread(newfsid, *bufout, 0, *sizeout);
	}
	return 1;
}

static void
generate_NowShowing_internal(int fsid, void *bufMain, unsigned sizeMain, 
			     char *out, int maxlen)
{
	int		j=0;
	mfs_attribute_t	showingAttr, partAttr, sizeAttr, programAttr, dateAttr, 
		timeAttr, titleAttr, etitleAttr, fileAttr, partIndexAttr, 
		partCountAttr, tmsidAttr, seriesAttr, episodicAttr, stationAttr,
		callSignAttr, selTypeAttr;
	size_t          l;
	struct tm *tm;
	char dow[4];
	void *bufShowing=0, *bufProgram=0, *bufPart=0, *bufSeries=0, 
		*bufStation=0;
	unsigned int sizeShowing,  sizeProgram, sizePart, sizeSeries, sizeStation;
	time_t secs;
	int has_title=0, has_etitle=0, has_partindex=0, has_partcount=0, has_tmsid=0,
		has_callsign=0;
	int episodic = 0;
	char title[132] = {0};
	char etitle[132] = {0};
	char *callSign = 0;
	char *tmsid = 0;
	int tmstype=0;
	int selType = 3;

	if (!get_attribute(bufMain,sizeMain, "Recording", -1, 
			   "Showing", &showingAttr)) {
		fprintf( stderr, "fsid=%d has no Showing attribute\n", fsid);
		goto out;
	}
	if (!get_attribute(bufMain,sizeMain, "Recording", -1, 
			   "Part", &partAttr)) {
		fprintf( stderr, "fsid=%d has no Part attribute\n",fsid);
		goto out;
	}
	if (!get_attribute(bufMain,sizeMain, "Recording", -1, 
			   "StreamFileSize", &sizeAttr)) {
		fprintf( stderr, "fsid=%d has no StreamFileSize attribute\n",fsid);
		goto out;
	}
	if (get_attribute(bufMain,sizeMain, "Recording", -1, 
			  "SelectionType", &selTypeAttr))
		selType = selTypeAttr.u.integer[0];
	if (!get_object( fsid, bufMain, sizeMain, 
			 showingAttr.u.object[0].fsid, 
			 &bufShowing, &sizeShowing)) {
		fprintf( stderr, "fsid=%d has bad Showing attribute fsid\n",fsid);
		goto out;
	}
	if (!get_attribute(bufShowing,sizeShowing, "Showing", 
			   showingAttr.u.object[0].subobj, 
			   "Program", &programAttr )) {
		fprintf( stderr, "fsid=%d has no Program attribute\n",fsid);
		goto out;
	}
	if (!get_attribute(bufShowing,sizeShowing, "Showing", 
			   showingAttr.u.object[0].subobj, 
			   "Date", &dateAttr )) {
		fprintf( stderr, "fsid=%d has no Date attribute\n",fsid);
		goto out;
	}
	if (!get_attribute(bufShowing,sizeShowing, "Showing", 
			   showingAttr.u.object[0].subobj, 
			   "Time", &timeAttr )) {
		fprintf( stderr, "fsid=%d has no Time attribute\n",fsid);
		goto out;
	}
	has_partindex = get_attribute(bufShowing,sizeShowing, "Showing", 
				      showingAttr.u.object[0].subobj, 
				      "PartIndex", &partIndexAttr );
	has_partcount = get_attribute(bufShowing,sizeShowing, "Showing", 
				      showingAttr.u.object[0].subobj, 
				      "PartCount", &partCountAttr );
	
	if (!get_object( showingAttr.u.object[0].fsid, bufShowing, sizeShowing, 
			 programAttr.u.object[0].fsid, 
			 &bufProgram, &sizeProgram )) {
		fprintf( stderr, "fsid=%d has bad Program attribute fsid\n",fsid);
		goto out;
		
	}
	has_title = get_attribute(bufProgram, sizeProgram, "Program", 
				  programAttr.u.object[0].subobj, 
				  "Title", &titleAttr );
	if (selType == 10 || selType == 5) {
		if (!has_title)
			strncpy( title, "Manual Recording", sizeof(title) );
		else
			snprintf( title, sizeof(title), "Manual: %s", 
				  titleAttr.u.string[0] );
	} else 
		strncpy( title, titleAttr.u.string[0], sizeof(title) );

	has_etitle  = get_attribute(bufProgram, sizeProgram, "Program", 
				    programAttr.u.object[0].subobj, 
				    "EpisodeTitle", &etitleAttr );
	has_tmsid  = get_attribute(bufProgram, sizeProgram, "Program", 
				   programAttr.u.object[0].subobj, 
				   "TmsId", &tmsidAttr );
	if (get_attribute(bufProgram,sizeProgram,"Program",
			  programAttr.u.object[0].subobj, 
			  "Series", &seriesAttr )) {
		if (get_object( programAttr.u.object[0].fsid, bufProgram, sizeProgram, 
			    seriesAttr.u.object[0].fsid, 
				&bufSeries, &sizeSeries )) {
			if (get_attribute(bufSeries, sizeSeries, "Series", 
				  seriesAttr.u.object[0].subobj, 
					  "Episodic", &episodicAttr ))
				episodic = episodicAttr.u.integer[0];
			if (bufSeries != bufProgram) free(bufSeries);
		}
	}
	if (get_attribute(bufShowing,sizeShowing,"Showing",
			  showingAttr.u.object[0].subobj, 
			  "Station", &stationAttr )) {
		if (!get_object( showingAttr.u.object[0].fsid, bufShowing, sizeShowing, 
				 stationAttr.u.object[0].fsid, 
				 &bufStation, &sizeStation )) {
			fprintf( stderr, "fsid=%d has bad Station attribute fsid\n",fsid);
		} else {
			if ((has_callsign=get_attribute(bufStation, sizeStation, "Station",
							stationAttr.u.object[0].subobj,
							"CallSign", &callSignAttr ))){
				callSign = callSignAttr.u.string[0];
			}
			if (bufStation && bufStation != bufShowing) free(bufStation);
		}
	}

	tmsid = tmsidAttr.u.string[0];
	if (has_tmsid && strncmp("MV",tmsid,2)==0)
		tmstype=0;
	else if (!has_tmsid || tmsid==0 || tmsid[0]==0)
		tmstype=episodic;
	else
		tmstype=1;
	if (episodic==1 || tmstype==1) {
		char partstr[128] = {0};
		if (has_partindex && has_partcount)
			snprintf( partstr,sizeof(partstr), " (%d of %d)", 
				  partIndexAttr.u.integer[0], partCountAttr.u.integer[0] );
		if (has_etitle)
			snprintf( etitle, sizeof(etitle), "%s%s", 
				  etitleAttr.u.string[0], partstr );
	}

	if (bufProgram != bufShowing) free(bufProgram);
	if (bufShowing != bufMain) free(bufShowing);
	secs = dateAttr.u.integer[0]*86400 + timeAttr.u.integer[0] + tzoffset();
	tm = gmtime(&secs);
	while (tm->tm_year > 100)
		tm->tm_year -= 100;
	strftime( dow, sizeof(dow), "%a", tm );
	snprintf( out, maxlen, 
		  "<Title>:<%s>"
		  "<Day>:<%s>"
		  "<Date>:<%d/%d>"
		  "<Year>:<%d/%d/%02d>"
		  "<Station>:<%s>"
		  "<EpisodeTitle>:<%s>"
		  "<FSID>:<%d/%d>"
		  "<TyStream>:<",
		  title,
		  dow, 
		  tm->tm_mon+1, tm->tm_mday, 
		  tm->tm_mon+1, tm->tm_mday, tm->tm_year, 
		  (callSign?callSign:""), etitle, 
		  showingAttr.u.object[0].fsid, 
		  showingAttr.u.object[0].subobj );
	l = strlen(out);
	for (j = 0; j < partAttr.n; ++j) {
		if (!get_object( fsid, bufMain, sizeMain, partAttr.u.object[j].fsid,
				 &bufPart, &sizePart )) {
			fprintf( stderr, "fsid=%d has bad Part attribute fsid\n",fsid);
			goto out;
		}
		get_attribute(bufPart, sizePart, "RecordingPart", 
			      partAttr.u.object[j].subobj,
			      "File", &fileAttr);
		snprintf( out+l, maxlen-l, "/%d", fileAttr.u.integer[0] );
		l = strlen(out);
		if (bufPart != bufMain) free(bufPart);
	}
	snprintf( out+l, maxlen-1, "><TotalSize>:<%d>\n", 
		  sizeAttr.u.integer[0]/1024 );
	attr_release(&titleAttr);
out:
	if (has_callsign) attr_release(&callSignAttr);
	if (has_etitle) attr_release(&etitleAttr);
	if (has_tmsid) attr_release(&tmsidAttr);
}

/**  
 *     Generate a NowShowing string for a single fsid 
 */
static int 
generate_1_NowShowing(int fsid, char *ret, unsigned retsize)
{
	void *buf;
	unsigned size;

	if ( fsid<=0  || mfs_fsid_type(fsid) != MFS_TYPE_OBJ) {
		if (fsid<=0) {
			fprintf(stderr, "Bad recording id: %d\n", fsid);
		} else {
			fprintf(stderr, "Bad recording id: %d  type: %d\n", fsid, mfs_fsid_type(fsid) );
		}
		return 0;
	}

	if (get_object( -1, 0, 0, fsid, &buf, &size )) {
		generate_NowShowing_internal(fsid, buf, size, ret, retsize );
		free(buf);
	} else {
		fprintf( stderr, "couldn't get top level Recording object fsid=%d\n", fsid );
		return 0;
	}
	return 1;
}

/** 
 *    Generate the complete NowShowing list and write it to a file descriptor 
 */
int generate_NowShowing( int fd ) 
{
	int i;
	const char *path = 0;
	static const char *paths[] = {
		"/Recording/NowShowingByClassic",
		"/Recording/NowShowing",
		"/Recording/Complete",
	};
	struct mfs_dirent *dir;
	u32 count;

	for(i=0; i<sizeof(paths)/sizeof(paths[0]); i++)
		if (mfs_resolve(paths[i]) != 0) {
			path = paths[i];
			break;
		}
	if (!path) return 1;
	dir = mfs_dir(mfs_resolve(path), &count);
	if (!dir) return 1;

	for (i=0;i<count;i++) {
		char buf[1024] = {0};
		if (generate_1_NowShowing(dir[i].fsid, buf, sizeof(buf) ))
			write( fd, buf, strlen(buf) );
	}
	if (dir) mfs_dir_free(dir);
	return 0;
}
