/*****************************************************************************
 * ty_audio.c - TiVo ty stream audio demuxer
 *****************************************************************************
 * Copyright (C) 2005 the VideoLAN team
 * Copyright (C) 2005 by Neal Symms (tivo@freakinzoo.com) - February 2005
 * based on code by Christopher Wingert for tivo-mplayer
 * tivo(at)wingert.org, February 2003
 *
 * $Id: ty_audio.c,v 1.1 2006/05/03 14:54:59 jamiepainter Exp $
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA  02111, USA.
 *
 * CODE CHANGES:
 * v1.0.0 - 24-Feb-2005 - Initial release - Series 1 support ONLY!
 * v1.0.1 - 25-Feb-2005 - Added fix for bad GOP headers - Neal
 * v1.0.2 - 26-Feb-2005 - No longer require "seekable" input stream - Neal
 *****************************************************************************/

/*****************************************************************************
 * Preamble
 *****************************************************************************/
                                                                                    
#include <stdlib.h>
#include <stdio.h>
#include <assert.h>
#include <errno.h>
#include <time.h>
#include <fcntl.h>
#include <memory.h>

#ifdef WIN32_CONSOLE_APP
#include <windows.h>  /* Windows needs this for DeleteFile, other Win32 APIs */
#include <io.h>
#else
#include <unistd.h>  /* Linux needs this for write() */
#include <netinet/in.h>
#endif

        
/* --------------------------------------------------------------------------
   Helper macros
-------------------------------------------------------------------------- */
#ifdef _DEBUG
  #define ASSERT(exp) \
              assert(exp)
#else
  #define ASSERT(exp) 1
#endif              

#define MIN(x, y) ( (x) <= (y) ? (x) : (y) )


/*---------------------------------------------------------------------------
  Define to get tons of printf logging
---------------------------------------------------------------------------*/
//#define VERBOSE_DEBUG

/*---------------------------------------------------------------------------
  Define to show non fatal errors
---------------------------------------------------------------------------*/
#define DEBUG_SHOW_ERRORS


/* Tyda multiple instance detection string (for mutex creation).  
   NOTE: This must be a system-wide unique string. */
#define TYDA_INSTANCE_DET_STR   "** Ty Demux Audio Instance Detection String 920112 **"

/* Tyda Version string */
#define TYDA_VERSION_STR            "1.00"

/* Parameter order (indices into the input arg_strs (argv) into main) */
typedef enum {
  TYDA_PARM_APP_NAME     = 0,      /* Application name */
  TYDA_PARM_IN_FILENAME  = 1,      /* Input filename */
  TYDA_PARM_OUT_FILENAME = 2,      /* Ouput filename */

  TYDA_PARM_NUM          = 3       /* Range checking */
} tyda_app_parm_type;


#define TRUE  1
#define FALSE 0


/*-------------------------------------------------------------------------
  Take 4 bytes and treat them as an int. 
-------------------------------------------------------------------------*/
#define U32_AT(p) (int)( (p)[0] << 24 | (p)[1] << 16 | (p)[2] << 8 | (p)[3] )
#define U16_AT(p) (short)( (p)[0] << 8 | (p)[1] )

#define SERIES1_PES_LENGTH  (11)
#define SERIES2_PES_LENGTH  (16)
#ifdef S1_TIVO
#define AUDIO_PES_LENGTH    SERIES1_PES_LENGTH
#else
#define AUDIO_PES_LENGTH    SERIES2_PES_LENGTH
#endif
#define AC3_PES_LENGTH      (14)
#define DTIVO_PTS_OFFSET    (6)
#define SA_PTS_OFFSET       (9)
#define AC3_PTS_OFFSET      (9)

static const unsigned char ty_MPEGAudioPacket[] = { 0x00, 0x00, 0x01, 0xc0 };
static const unsigned char ty_AC3AudioPacket[] = { 0x00, 0x00, 0x01, 0xbd };

/*****************************************************************************
 * Local prototypes
 *****************************************************************************/
static int get_chunk_header();
static int find_es_header( unsigned const char *header,
   unsigned char *buffer, int bufferSize, int *esOffset1 );

/* packet types for reference:
subtype/type
 2/c0: audio data continued
 3/c0: audio packet header (PES header)
 4/c0: audio data (S/A only?)
 9/c0: audio packet header, AC-3 audio
 2/e0: video data continued
 6/e0: video packet header (PES header)
 7/e0: video sequence header start
 8/e0: video I-frame header start
 a/e0: video P-frame header start
 b/e0: video B-frame header start
 c/e0: video GOP header start
 e/01: closed-caption data
 e/02: Extended data services data 
 e/03: ipreview data ("thumbs up to record" signal)
*/

#define TIVO_PES_FILEID   ( 0xf5467abd )
#define AC3_SYNC            0x0b77
#define TIVO_PART_LENGTH  ( 0x20000000 )    /* 536,870,912 bytes */
#define CHUNK_SIZE        ( 128 * 1024 )

typedef struct
{
  int           l_rec_size;
  #ifdef SUPPORT_EXTENDED_DATA
  unsigned char ex1, ex2;
  #endif
  unsigned char rec_type;
  unsigned char subrec_type;
  char b_ext;
} ty_rec_hdr_t;

typedef struct 
{
  int             i_chunk_count;
  int             tivoType;           /* 1 = SA, 2 = DTiVo */
  int             b_mpeg_audio;       /* true if we're using MPEG audio */
  int             i_pes_buf_cnt;      /* how many bytes in our buffer */

  #define MAX_NUM_RECS  750
  ty_rec_hdr_t    rec_hdrs[MAX_NUM_RECS];      /* record headers array */
  int             i_cur_rec;          /* current record in this chunk */
  int             i_num_recs;         /* number of recs in this chunk */
} demux_sys_t;


/* This module's info block */
typedef struct {
  int     found_audio;              /* TRUE == we found our first audio pkt*/

  unsigned char   *chunk_buf;       /* Chunk buffer ptr */
  int     curr_buf_pos;             /* Current chunk buffer position */


  FILE *  in_file;                  /* input file struct */
  int     out_file;                 /* output file descriptor */
  #ifdef WIN32_CONSOLE_APP
  char    in_file_name[MAX_PATH+1]; /* Input file name string + NULL */
  char    out_file_name[MAX_PATH+1];/* Ouptput file name string + NULL */
  #endif

  demux_sys_t p_sys;                /* TY Demux state info */
  int     in_ac3_sync;
} tyda_info_type;

/* ------------------------------------------------------------------------
   File Globals
------------------------------------------------------------------------ */

/* This module's info block */
static tyda_info_type tyda_info;
#ifdef WIN32_CONSOLE_APP
static unsigned char  tyda_chunk_buf[CHUNK_SIZE+1];
#endif


#ifdef WIN32_CONSOLE_APP
/*===========================================================================
FUNCTION    
  file_peek 

DESCRIPTION
  Reads some bytes from a file without moving the position.

RETURN VALUE
  Bytes read

SIDE EFFECTS
  None
===========================================================================*/
int file_peek(
  char *dest, 
    /* Put peeked bytes here */
  unsigned int size, 
    /* Peek this many bytes */
  FILE *file
    /* Peek from this file's current position */
)
{
  int   start_pos;
  size_t bytes_read;

  /* - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

  ASSERT(file != NULL);
  ASSERT(dest != NULL);

  /*-------------------------------------------------------------------------
    Save the current position
  -------------------------------------------------------------------------*/
  start_pos = ftell(file);
  if (start_pos < 0)
  {
    return -1;
  }

  /*-------------------------------------------------------------------------
    Read the requested bytes
  -------------------------------------------------------------------------*/
  bytes_read = fread(dest, 1, size, file);
  if ((bytes_read < 0) && !feof(file))
  {
    /*-----------------------------------------------------------------------
      Read error, but not EOF, so true error
    -----------------------------------------------------------------------*/
    fprintf(stderr, "Error peeking\n");
    return -1;
  }

  /*-------------------------------------------------------------------------
    Seek back to start position
  -------------------------------------------------------------------------*/
  if(fseek(file, start_pos, SEEK_SET) != 0)
  {
    return -1;
  }

  return bytes_read;

} /* file_peek() */
#endif


/*===========================================================================
FUNCTION    
  tyda_init

DESCRIPTION
  Init data structures

RETURN VALUE
  None

SIDE EFFECTS
  None
===========================================================================*/
int tyda_init()
{
  #ifdef WIN32_CONSOLE_APP
  unsigned char p_peek[12];

  /* - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

  /*-------------------------------------------------------------------------
    If we're running on the TIVO itself, we can safely assume the files are
    actual TY files.
  -------------------------------------------------------------------------*/

  /* peek at the first 12 bytes. */
  /* for TY streams, they're always the same */
  if (file_peek(p_peek, 12, tyda_info.in_file) < 12)
  {
    return FALSE;
  }

  if ( U32_AT(p_peek) != TIVO_PES_FILEID ||
       U32_AT(&p_peek[4]) != 0x02 ||
       U32_AT(&p_peek[8]) != CHUNK_SIZE )
  {
      /* doesn't look like a TY file... */
    #if 0
    char *psz_ext = strrchr(tyda_info.in_file_name, '.');

    if( !psz_ext || strcasecmp(psz_ext, ".ty") )
    { 
      return FALSE;
    }
    #endif

    fprintf(stderr, "Does not look like a TY file, bailing\n");
    return FALSE;
  }

  fprintf(stderr, "-- Detected valid TY stream\n" );  
  #endif

  /*-------------------------------------------------------------------------
    Init our global struct
  -------------------------------------------------------------------------*/
  memset(&tyda_info.p_sys, 0, sizeof(tyda_info.p_sys));
  tyda_info.p_sys.b_mpeg_audio = FALSE;

  return TRUE;
}



/* 
=============================================================
============== */
static int find_es_header( 
  unsigned const char *header,
  unsigned char *buffer, 
  int bufferSize, 
  int *esOffset1 )
{
    int count;

    *esOffset1 = -1;
    for( count = 0 ; count < bufferSize ; count++ )
    {
        if ( ( buffer[ count + 0 ] == header[ 0 ] ) &&
             ( buffer[ count + 1 ] == header[ 1 ] ) &&
             ( buffer[ count + 2 ] == header[ 2 ] ) &&
             ( buffer[ count + 3 ] == header[ 3 ] ) )
        {
            *esOffset1 = count;
            return 1;
        }
    }
    return( -1 );
}


/* 
=============================================================
============== */
/* check if we have a full PES header, if not, then save what we have.
 * this is called when audio-start packets are encountered.
 * Returns:
 *     1 partial PES hdr found, some audio data found (buffer adjusted),
 *    -1 partial PES hdr found, no audio data found
 *     0 otherwise (complete PES found, pts extracted, pts set, buffer adjusted) */
/* TODO: fix it so it works with S2 / SA / DTivo / HD etc... */
static int check_sync_pes( int offset, int rec_len,
                           int * pes_length_ptr)
{
  demux_sys_t *p_sys = &tyda_info.p_sys;
  int pts_offset;
  int pes_length = p_sys->b_mpeg_audio?AUDIO_PES_LENGTH:AC3_PES_LENGTH;

  if( p_sys->tivoType == 1 )
  {
    /* SA tivo */
    pts_offset = SA_PTS_OFFSET;
  }
  else
  {
    /* DTivo */
    pts_offset = p_sys->b_mpeg_audio?DTIVO_PTS_OFFSET:AC3_PTS_OFFSET;
  }

  if ( offset < 0 || offset + pes_length > rec_len )
  {
    /* entire PES header not present */
    /* save the partial pes header */
    if( offset < 0 )
    {
      /* no header found, fake some 00's (this works, believe me) */
      p_sys->i_pes_buf_cnt = 4;
      if( rec_len > 4 )
      {
        #ifdef DEBUG_SHOW_ERRORS
        fprintf(stderr, "%d: PES header not found in record of %d bytes!\n",
                p_sys->i_chunk_count, rec_len );
        #endif
      }
      return -1;
    }
                               
    /* copy the partial pes header we found */
    p_sys->i_pes_buf_cnt = rec_len - offset;

    if( offset > 0 )
    {
      /* PES Header was found, but not complete, so trim the end of this record */
      *pes_length_ptr -= rec_len - offset;
      return 1;
    }

    return -1;    /* partial PES, no audio data */
  }

  /* full PES header present, extract PTS */
  /*msg_Dbg(p_demux, "Audio PTS %lld", p_sys->lastAudioPTS );*/
  /* adjust audio record to remove PES header */
  *pes_length_ptr = pes_length;
  return 0;
}


// checks to see if a buf overflow would occur.  1 = no problem, 0 = buf overflow
int tyda_buf_boundschk(int how_far)
{
  return ((tyda_info.curr_buf_pos + how_far) <= CHUNK_SIZE);
}

// seeks in the TY buf.  returns 1 for success, 0 for EOC
int tyda_buf_seek(int how_far)
{
  int success = tyda_buf_boundschk(how_far);
  if (success)
  {
    tyda_info.curr_buf_pos += how_far;
  }
  return success;
}


int tyda_buf_read(unsigned char *dst, int bytes_to_copy)
{
  bytes_to_copy = MIN(bytes_to_copy, CHUNK_SIZE - tyda_info.curr_buf_pos);
  if (bytes_to_copy > 0)
  {
    memcpy(dst, &tyda_info.chunk_buf[tyda_info.curr_buf_pos], bytes_to_copy);
  
    /* Advance the read ptr */
    tyda_info.curr_buf_pos += bytes_to_copy;    
  }

  return bytes_to_copy;
}



/*===========================================================================
FUNCTION    
  tyda_chunk_writeall

DESCRIPTION
  Writes data from the chunk buf to the output file descriptor.

RETURN VALUE
  >= 0: Bytes written
  < 0 : error why we couldn't keep writing

SIDE EFFECTS
  None
===========================================================================*/
int tyda_chunk_writeall(int count)
{
  int actual_bytes_written;
  int tot_bytes_written = 0;

  /* - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

  while (count > 0) 
  {
    // debug only
    if (!tyda_buf_boundschk(count))
    {
      fprintf(stderr,
              "Was told to read past chunk buf.  Bailing before then\n");
      return -1;
    }

    actual_bytes_written = write(tyda_info.out_file, 
                                 &tyda_info.chunk_buf[tyda_info.curr_buf_pos], 
                                 count );
    if (actual_bytes_written < 0)                 
    {
      if (errno != EINTR && errno != EAGAIN)
      {
        fprintf(stderr, "Error writing to output fd, errno = %d\n", errno);
        return actual_bytes_written;
      }
      continue;
    }

    /*-----------------------------------------------------------------------
      Seek past the data written.  Can't fail, since we pre-checked the buf 
      bounds above
    -----------------------------------------------------------------------*/
    (void)tyda_buf_seek(actual_bytes_written);

    /*-----------------------------------------------------------------------
      Need to write this many less bytes next time
    -----------------------------------------------------------------------*/
    count -= actual_bytes_written;

    /*-----------------------------------------------------------------------
      Wrote this many bytes so far
    -----------------------------------------------------------------------*/
    tot_bytes_written += actual_bytes_written;
  }

  return tot_bytes_written;

} /* tyda_chunk_writeall() */


/*===========================================================================
FUNCTION    
  tyda_demux_chunk

DESCRIPTION
  Demux the audio from a TY chunk.  

  Demux code should perform writes to fd until EOF on input buf.
  TyDemux should return # bytes written or a negative value for the error.
  Must handle partial writes, that is, write to fd coming back with EINTR or
  EAGAIN.

RETURN VALUE
  Returns -1 in case of error, 0 in case of EOF (normal exit).

SIDE EFFECTS
  None
===========================================================================*/
int tyda_demux_chunk(
  unsigned char * buf, 
    /* Source TY buffer */
  int    size,
    /* Size of buf, in bytes */
  int    out_fd
    /* Output file descriptor, which may only be able to
       handle partial writes */
)
{
  int              l_rec_size;
  ty_rec_hdr_t     *rec_hdr;

  int              esOffset1;

  int              last_dot = 0;
  int              record_contains_payload = TRUE;

  demux_sys_t      *p_sys = &tyda_info.p_sys;

  int ret_val;

  /* - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

  /*-------------------------------------------------------------------------
    Save the output file descriptor
  -------------------------------------------------------------------------*/
  tyda_info.out_file = out_fd;

  /*-------------------------------------------------------------------------
    Begin parsing at start of chunk buffer
  -------------------------------------------------------------------------*/
  tyda_info.chunk_buf = buf;
  tyda_info.curr_buf_pos = 0;

  /*-------------------------------------------------------------------------
    Handle special case where someone just wants to read a part header
  -------------------------------------------------------------------------*/
  if (size == 16)
  {
    if (tyda_chunk_writeall(size) < size)
    {
      return -1;
    }
    else
    {
      return size;
    }
  }
  else if (size != CHUNK_SIZE)
  {
    /*-----------------------------------------------------------------------
      We currently only support parsing one full TY chunk at a time
    -----------------------------------------------------------------------*/
    fprintf(stderr, 
            "Asked to write %d bytes, != chunk size.  Not supported\n",
            size);
    return -1;
  }

  /*-------------------------------------------------------------------------
    Get this chunk's header
  -------------------------------------------------------------------------*/
  ret_val = get_chunk_header();
  if (ret_val != 1)
  {
    /*-----------------------------------------------------------------------
      Failed to parse header, or EOC
    -----------------------------------------------------------------------*/
    return ret_val;
  }

  /*-------------------------------------------------------------------------
    Parse all the records
  -------------------------------------------------------------------------*/
  while (p_sys->i_cur_rec < p_sys->i_num_recs)
  {
    /*-----------------------------------------------------------------------
      Parse and dump one record of the chunk
    -----------------------------------------------------------------------*/
    rec_hdr = &p_sys->rec_hdrs[ p_sys->i_cur_rec ];
    l_rec_size = rec_hdr->l_rec_size;
  
    if (rec_hdr->b_ext)
    {
      /*---------------------------------------------------------------------
        Extended data (no payload), skip it.
      ---------------------------------------------------------------------*/
      goto skip_reading_data;
    }

    if (l_rec_size <= 0)
    {
      /*---------------------------------------------------------------------
        0 record size.  We're done with this chunk
      ---------------------------------------------------------------------*/
      return 0;
    }
  
    /*-----------------------------------------------------------------------
      This record has a data payload.  Deal with it.
    -----------------------------------------------------------------------*/
    #ifdef VERBOSE_DEBUG
    fprintf(stderr, "Record Type 0x%x/0x%02x %d bytes\n",
                rec_hdr->subrec_type, rec_hdr->rec_type, l_rec_size );
    #endif
  
    if (rec_hdr->rec_type != 0xc0)
    {
      /*---------------------------------------------------------------------
        Video or unknown record.  Seek ahead of it
      ---------------------------------------------------------------------*/
      if (!tyda_buf_seek(l_rec_size))
      {
        /*-------------------------------------------------------------------
          Todo: Why does this fail at end of OTA streams?  Is there no
          XML at the end??
        -------------------------------------------------------------------*/
        return 0;
      }
  
      goto skip_reading_data;
    }
  
    /*-----------------------------------------------------------------------
      Set the Tivo type based on the sub record type.  Only need to do this
      until we're sure of what type of audio we're dealing with.
    -----------------------------------------------------------------------*/
    if (0 == p_sys->tivoType)
    {
      if ( rec_hdr->subrec_type == 0x09 )
      {
        /* set up for AC-3 audio */
        fprintf(stderr, "-- Detected AC-3 audio\n");
        p_sys->tivoType = 2;      /* AC3 is only on dtivo */
      }
      else
      {
        /* set up for MPEG 1 Layer II audio */
        fprintf(stderr, "-- Detected MPEG 1 Layer II Audio\n");
        p_sys->b_mpeg_audio = TRUE;
      } 
  
      tyda_info.found_audio = TRUE;
      fprintf(stderr, "-- Extracting audio |");
    }
  
    /*-----------------------------------------------------------------------
      Parse the audio record payloads based on sub record type
    -----------------------------------------------------------------------*/
    switch (rec_hdr->subrec_type)
    {
    case 0x2:
      /*---------------------------------------------------------------------
        SA or DTiVo Audio Data, no PES hdr (continued block) 
      ---------------------------------------------------------------------*/
      if (p_sys->i_pes_buf_cnt > 0)
      {
        /*-------------------------------------------------------------------
          continue PES if previous was incomplete
        -------------------------------------------------------------------*/
        int i_need = AUDIO_PES_LENGTH - p_sys->i_pes_buf_cnt;
  
        #ifdef VERBOSE_DEBUG
        fprintf(stderr, "%d: Continuing SA/DTivo audio PES header\n",
                p_sys->i_chunk_count);
        #endif
  
        /* do we have enough data to complete? */
        if (i_need < l_rec_size)
        {
          /* advance the block past the PES header (don't want to send it) */
          (void)tyda_buf_seek(i_need);
          p_sys->i_pes_buf_cnt = 0;
        }
        else
        {
          /* don't have complete PES hdr; seek past what we have and get next
             record */
          (void)tyda_buf_seek(l_rec_size);
          p_sys->i_pes_buf_cnt += l_rec_size;
          record_contains_payload = FALSE;
        }
      }
  
      /*---------------------------------------------------------------------
        Dump payload to output file
      ---------------------------------------------------------------------*/
      if (TRUE == record_contains_payload)
      {
        #ifdef VERBOSE_DEBUG
        fprintf(stderr, "Writing SA/DTivo MPEG Audio, %d bytes\n",
               l_rec_size);
        #endif
  
        if (tyda_chunk_writeall(l_rec_size) < l_rec_size)
        {
          return -1;
        }
      }
  
      break;
  
  
    case ( 0x03 ):
      /*---------------------------------------------------------------------
        MPEG Audio with PES Header, either SA or DTiVo 
      ---------------------------------------------------------------------*/
      find_es_header( ty_MPEGAudioPacket, 
                      &tyda_info.chunk_buf[tyda_info.curr_buf_pos],
                      l_rec_size, &esOffset1 );
  
      if ( ( esOffset1 == 0 ) && ( l_rec_size == AUDIO_PES_LENGTH ) )
      {
        /*-------------------------------------------------------------------
          SA PES Header, No Audio Data
        -------------------------------------------------------------------*/
        p_sys->tivoType = 1;
      }
      else
      {
        /*-------------------------------------------------------------------
          DTiVo Audio with PES Header 
        -------------------------------------------------------------------*/
        int pes_length = 0;
        int record_contains_payload = TRUE;
        int bytes_to_write;
  
        p_sys->tivoType = 2;
  
        /* Check for complete PES */
        if (check_sync_pes(esOffset1,
                           l_rec_size, &pes_length) == -1)
        {
          /* partial PES header found, nothing else: we're done with this
             record, so seek ahead of the data */
          #ifdef VERBOSE_DEBUG
          fprintf(stderr, "Partial PES found, no data, skipping pkt\n");
          #endif

          record_contains_payload = FALSE;

          if (!tyda_buf_seek(l_rec_size))
          {
            fprintf(stderr, "Reached EOC before getting to end of record = %d bytes, chunk pos was %d\n",
                    l_rec_size, tyda_info.curr_buf_pos);
            return 0;
          }
        }
  
        /*-------------------------------------------------------------------
          Dump to file
        -------------------------------------------------------------------*/
        bytes_to_write = l_rec_size - pes_length;
        if ( (TRUE == record_contains_payload) &&
             (bytes_to_write > 0) )
        {
          #ifdef VERBOSE_DEBUG
          fprintf(stderr, "Writing DTivo MPEG Audio (stripped PES hdr), %d bytes\n",
                 bytes_to_write);
          #endif
  
          /*-----------------------------------------------------------------
            Remove PES hdr and copy to output
          -----------------------------------------------------------------*/
          (void)tyda_buf_seek(pes_length);
          if (tyda_chunk_writeall(bytes_to_write) < bytes_to_write)
          {
            return -1;
          }
        }
  
      } /* if DTiVo */
  
      break;
  
  
    case ( 0x04 ):
      /*---------------------------------------------------------------------
        SA Audio with no PES Header, dump to file
      ---------------------------------------------------------------------*/
      #ifdef VERBOSE_DEBUG
      fprintf(stderr, "Writing SA MPEG Audio, %d bytes\n",
             l_rec_size);
      #endif
  
      if (tyda_chunk_writeall(l_rec_size) < l_rec_size)
      {
        return -1;
      }
  
      break;
  
  
    case ( 0x09 ):
      {
      int pes_length = 0;
      int record_contains_payload = TRUE;
      int bytes_checked;
      int num_bytes_to_write;
  
      if (p_sys->b_mpeg_audio)
      {
        fprintf(stderr, "\nIgnoring AC-3 record in MPEG stream\n");
        break;
      }
  
      /*---------------------------------------------------------------------
        DTiVo AC3 Audio Data with PES Header
      ---------------------------------------------------------------------*/
      find_es_header( ty_AC3AudioPacket, 
                      &tyda_info.chunk_buf[tyda_info.curr_buf_pos],
                      l_rec_size, &esOffset1 );

      /* Check for complete PES */
      if (check_sync_pes(esOffset1,
                         l_rec_size, &pes_length) == -1)
      {
        /* partial PES header found, nothing else.  We're done with this
           record */
        #ifdef VERBOSE_DEBUG
        fprintf(stderr, 
                "Partial or no PES found in record %d/%d, dropping its payload of %d bytes\n",
                p_sys->i_cur_rec, p_sys->i_num_recs, l_rec_size);
        #endif

        record_contains_payload = FALSE;
        if (!tyda_buf_seek(l_rec_size))
        {
          fprintf(stderr, "Reached EOC before getting to end of record = %d bytes, chunk pos was %d\n",
                  l_rec_size, tyda_info.curr_buf_pos);
          tyda_info.in_ac3_sync = FALSE; // correct?
          return 0;
        }
      }
  
      /*---------------------------------------------------------------------
        Dump to file
      ---------------------------------------------------------------------*/
      if (!record_contains_payload)
      {
        break;
      }
  
      /*---------------------------------------------------------------------
        Throw away the PES hdr (seek ahead of it)
      ---------------------------------------------------------------------*/
      (void)tyda_buf_seek(pes_length);

      num_bytes_to_write = l_rec_size - pes_length;
  
      if (!tyda_info.in_ac3_sync)
      {
        /*-------------------------------------------------------------------
          Check for AC3 sync word
  
          We need to remove data before the AC3 sync word, or VLC cannot
          play it.  Not sure if this is a VLC problem, or standard practice.
        -------------------------------------------------------------------*/
        for (bytes_checked = 0; num_bytes_to_write > 0; 
             bytes_checked += sizeof(short))
        {
          if (U16_AT(&tyda_info.chunk_buf[tyda_info.curr_buf_pos]) == AC3_SYNC)
          {
            /*---------------------------------------------------------------
              Found AC3 sync.  The read ptr and write length were 
              automatically moved ahead, thereby skipping the pre sync word
              data.
            ---------------------------------------------------------------*/
            tyda_info.in_ac3_sync = TRUE;
          }
          else
          {
            /*---------------------------------------------------------------
              Take the failed skip word candidate out of our payload
            ---------------------------------------------------------------*/
            tyda_buf_seek(sizeof(short));
            num_bytes_to_write -= sizeof(short);
          }
  
          if (tyda_info.in_ac3_sync)
          {
            /*---------------------------------------------------------------
              Success, stop searching
            ---------------------------------------------------------------*/
            break;
          }
        }
  
        if (!tyda_info.in_ac3_sync)
        {
          /*-----------------------------------------------------------------
            We failed to find the sync word.  Do not write this pkt out to
            the file.
          -----------------------------------------------------------------*/
          #ifdef DEBUG_VERBOSE
          fprintf(stderr, "Failed to find AC3 sync, skipping pkt\n");
          #endif
          goto skip_reading_data;
        }
      } /* if in AC3 sync */
  
      if (num_bytes_to_write > 0)
      {
  
        #ifdef VERBOSE_DEBUG
        fprintf(stderr, "Writing DTivo AC3 Audio (stripped PES hdr), %d bytes\n",
               num_bytes_to_write);
        #endif
  
        /*-------------------------------------------------------------------
          Remove PES hdr and copy to output
        -------------------------------------------------------------------*/
        if (tyda_chunk_writeall(num_bytes_to_write) < num_bytes_to_write)
        {
          return -1;
        }
      }
  
      } /* Current case scope */
      break;
  
    default:
      /*---------------------------------------------------------------------
        Unknown sub record type, skip it
      ---------------------------------------------------------------------*/
      break;
    } /* Switch on sub record type */
  
  
  skip_reading_data:
  
    /*-----------------------------------------------------------------------
      Advance to the next record
    -----------------------------------------------------------------------*/
    p_sys->i_cur_rec++;
  
    /*-----------------------------------------------------------------------
      Print a dot for every 512 chunks (64 MBytes) parsed
    -----------------------------------------------------------------------*/
    if ( (tyda_info.p_sys.i_chunk_count > last_dot) &&
        (0 == (tyda_info.p_sys.i_chunk_count & 0x1ff)) )
    {
      last_dot = tyda_info.p_sys.i_chunk_count;
      fprintf(stderr, ".");
    }

  } /* while there are still records */

  return 0;

} /* tyda_demux_chunk() */



/*===========================================================================
FUNCTION    
  get_chunk_header

DESCRIPTION
  Parses a chunk header to generate the records describing the chunk's
  data payload.

RETURN VALUE
  1 : success
  0 : hit EOC, or decided to drop it.
  -1: error

SIDE EFFECTS
  None
===========================================================================*/
static int get_chunk_header()
{
  int i_readSize, i_num_recs, i;
  unsigned char packet_header[4];
  unsigned char record_header[16];
  ty_rec_hdr_t *p_rec_hdr;
  demux_sys_t *p_sys = &tyda_info.p_sys;
  int i_payload_size = 0;         /* sum of all records */

  #ifdef VERBOSE_DEBUG
  fprintf(stderr, "parsing ty chunk #%d\n", p_sys->i_chunk_count );
  #endif

  /* read the TY packet header */
  i_readSize = tyda_buf_read(packet_header, 4);
  if ( i_readSize < 4 )
  {
    return 0;
  }

  p_sys->i_chunk_count++;

  #ifdef WIN32_CONSOLE_APP
  if (0 == memcmp(packet_header, "####", 4))
  {
    /*---------------------------------------------------------------------
      We've hit the final XML stuff, no more data, so bail as EOF
    ---------------------------------------------------------------------*/
    fprintf(stderr, "E");

    /* EOF */
    return 0;
  }
  #endif

  /* if it's a PART Header, then we're done */
  if (U32_AT( &packet_header[ 0 ] ) == TIVO_PES_FILEID)
  {
    #ifdef VERBOSE_DEBUG
    fprintf(stderr, "skipping TY PART Header\n" );
    #endif

    if (TRUE == tyda_info.found_audio)
    {
      /*-------------------------------------------------------------------
        Start printing part headers after we've hit the first audio pkt
      -------------------------------------------------------------------*/
      fprintf(stderr, "|");
    }

    return 0;
  }

  /* number of records in chunk (8- or 16-bit number) */
  if (packet_header[3] & 0x80)
  {
    /* 16 bit rec cnt */
    p_sys->i_num_recs = i_num_recs = (packet_header[1] << 8) + 
                                      packet_header[0];
  }
  else
  {
    /* 8 bit reclen - tivo 1.3 format */
    p_sys->i_num_recs = i_num_recs = packet_header[0];
  }
  p_sys->i_cur_rec = 0;

  #ifdef VERBOSE_DEBUG
  fprintf(stderr, "Chunk %d contains %d record headers\n", 
         p_sys->i_chunk_count-1, i_num_recs);
  #endif

  /* parse headers into array */
  if (i_num_recs > MAX_NUM_RECS)
  {
    fprintf(stderr, "Exceeded max TY records, bailing\n");
    return -1;
  }

  for (i = 0; i < i_num_recs; i++)
  {
    i_readSize = tyda_buf_read(record_header, 16);
    if (i_readSize < 16)
    {
      return 0;
    }

    p_rec_hdr = &p_sys->rec_hdrs[i];     /* for brevity */
    p_rec_hdr->rec_type = record_header[3];
    p_rec_hdr->subrec_type = record_header[2] & 0x0f;

    if ((record_header[ 0 ] & 0x80) == 0x80)
    {
      p_rec_hdr->b_ext = TRUE;
    }
    else
    { 
      p_rec_hdr->l_rec_size = ( record_header[ 0 ]   << 8 |
                                record_header[ 1 ] ) << 4 | 
                              ( record_header[ 2 ]   >> 4 );

      i_payload_size += p_rec_hdr->l_rec_size;
      p_rec_hdr->b_ext = FALSE;

      if ( (p_rec_hdr->l_rec_size > CHUNK_SIZE) ||
           (i_payload_size > CHUNK_SIZE) )
      {
        /*---------------------------------------------------------------
          Dunno why, but the end of my TY files have garbage at the 
          end, so I have to ignore when the chunk payload goes crazy
          high.
        ---------------------------------------------------------------*/
        #ifdef VERBOSE_DEBUG
        fprintf(stderr, "Chunk payload (%d bytes) > max, treating as EOF\n", 
               i_payload_size);
        #endif
        return 0;
      }
    }
  } /* end of record-header loop */

  return 1;
} /* get_chunk_header() */

#ifdef WIN32_CONSOLE_APP
/*===========================================================================

FUNCTION    
  tyda_demux_audio

DESCRIPTION
  Parses TY file and dumps audio to a separate file.

DEPENDENCIES
  None.

RETURN VALUE
  TRUE for successful TY audio dumping, else FALSE

SIDE EFFECTS
  None

===========================================================================*/
int tyda_demux_audio(void)
{
  int   bytes_read;
  int   result = 1;
  int   start_time, now;  

  /* - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

  start_time = time(&now);

  /*-------------------------------------------------------------------------
    Open the TY module
  -------------------------------------------------------------------------*/
  if (FALSE == tyda_init())
  {
    return FALSE;
  }

  /*-------------------------------------------------------------------------
    Keep dumping as long until EOF or error
  -------------------------------------------------------------------------*/
  do
  {
    bytes_read = fread(tyda_chunk_buf, 1, CHUNK_SIZE, tyda_info.in_file);
    if (CHUNK_SIZE == bytes_read)
    {
      result = tyda_demux_chunk(tyda_chunk_buf, bytes_read, tyda_info.out_file);
    }
  } while ( (CHUNK_SIZE == bytes_read) && (result >= 0) );


  if (result >= 0)
  {
    fprintf(stderr, "\n\n-- Summary:\n");
    fprintf(stderr, "   TY data   : %4u MBytes (%d chunks)\n",
           tyda_info.p_sys.i_chunk_count * 128 /1024,
           tyda_info.p_sys.i_chunk_count);
    fprintf(stderr, "   Audio data: %4u MBytes\n", 
           tell(tyda_info.out_file)/(1024*1024));
    fprintf(stderr, "   Demux time: %4d seconds\n", time(&now) - start_time);
  }

  if (result < 0)
  {
    fprintf(stderr, "\nError parsing TY, bailing, result = %d\n", result);
    return FALSE;
  }

  fprintf(stderr, "\nDone.\n");

  return TRUE;

} /* tyda_demux_audio() */


/*===========================================================================

FUNCTION    
  tyda_parse_options

DESCRIPTION
  Parses and verifies application command line options

DEPENDENCIES
  None.

RETURN VALUE
  TRUE for successful parameter verification, else FALSE

SIDE EFFECTS
  None

===========================================================================*/
int tyda_parse_options(
  int    arg_count, 
    /* Number of arguments passed in */
  char **arg_strs
    /* Pointer to arg_count number of string arguments passed in */
)
{

  /* - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

  /* ------------------------------------------------------------------------
     Draw nice intro banner
  ------------------------------------------------------------------------ */
  fprintf(stderr, "\n");
  fprintf(stderr, "------------------------------------------------------------------------\n");
  fprintf(stderr, "Ty Demuxer for Audio (tyda) v%s\n", TYDA_VERSION_STR);
  fprintf(stderr, "\n");
  fprintf(stderr, "Dumps the audio from a Tivo TY file\n");
  fprintf(stderr, "Currently supports TY files from Series2 Tivo's only\n");
  fprintf(stderr, "\n");
  fprintf(stderr, "Built on %s, at %s.\n", __DATE__, __TIME__);
  fprintf(stderr, "Email <dbrackma@yahoo.com> with bugs and suggestions.\n");
  fprintf(stderr, "------------------------------------------------------------------------\n");
  fprintf(stderr, "\n");

  /* ------------------------------------------------------------------------
     arg_count should always be >= 1, since the first argument is the name
     of the application.
  ------------------------------------------------------------------------ */
  ASSERT(arg_count >= TYDA_PARM_APP_NAME+1);

  /* ------------------------------------------------------------------------
     If we don't have the right number of parms, display the help banner
     and bail.
  ------------------------------------------------------------------------ */
  if (arg_count < (TYDA_PARM_NUM)) 
  {
    fprintf(stderr, "usage: tyda <input_ty_file> <output_audio_file>\n");
    fprintf(stderr, "\n");
    fprintf(stderr, "\n");

    return FALSE;
  }

  /*-------------------------------------------------------------------------
    Grab the filenames
  -------------------------------------------------------------------------*/
  strcpy(tyda_info.in_file_name, arg_strs[TYDA_PARM_IN_FILENAME]);
  strcpy(tyda_info.out_file_name, arg_strs[TYDA_PARM_OUT_FILENAME]);

  /* ------------------------------------------------------------------------
     Open the input file for reading
  ------------------------------------------------------------------------ */
  tyda_info.in_file = fopen(tyda_info.in_file_name, "rb");
  if (NULL == tyda_info.in_file)
  {
    /* ----------------------------------------------------------------------
       Input file failed to open -- bail
    ---------------------------------------------------------------------- */
    fprintf(stderr, 
            "Failed to open input file: %s, err = %d (probably doesn\'t exist)\n", 
            tyda_info.in_file_name, GetLastError());
    return FALSE;
  }

  /*-------------------------------------------------------------------------
    Open the output file for writing, use io.h streaming
  -------------------------------------------------------------------------*/
  tyda_info.out_file = open(tyda_info.out_file_name, 
                            O_BINARY|O_WRONLY|O_CREAT|O_TRUNC, 0644);

  if (-1 == tyda_info.out_file)
  {
    /*-----------------------------------------------------------------------
       Output file failed to open -- bail
    -----------------------------------------------------------------------*/
    fprintf(stderr, "Failed to open output file: %s, err = %d\n", 
           tyda_info.out_file_name, errno);
    return FALSE;
  }

  /*-------------------------------------------------------------------------
    Options parsed successfully
  -------------------------------------------------------------------------*/
  return TRUE;
} /* tyda_parse_options() */



/*===========================================================================

FUNCTION    MAIN

DESCRIPTION
  Main thread context, entered via the application invocation from the 
  command line.
  
  Performs mucho error checking on the input parms and then starts parsing 
  the input file.

DEPENDENCIES
  None.

RETURN VALUE
  int: Process return code: 0 for success, else failure

SIDE EFFECTS
  None

===========================================================================*/
int main(
  int    arg_count, 
    /* Number of arguments passed in */
  char **arg_strs
    /* Pointer to arg_count number of string arguments passed in */
)
{
  int                       ret_val = 0;

  /* - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

  /*-------------------------------------------------------------------------
    Init globals
  -------------------------------------------------------------------------*/
  memset(&tyda_info, 0, sizeof(tyda_info));
  tyda_info.out_file = -1;

  /* ------------------------------------------------------------------------
     Parse command line options
  ------------------------------------------------------------------------ */
  if (FALSE == tyda_parse_options(arg_count, arg_strs))
  {
    /* ----------------------------------------------------------------------
       Error reading command line options.  Bail.
    ---------------------------------------------------------------------- */
    ret_val = -1;
    goto exit_and_clean_up;
  }

  /*-------------------------------------------------------------------------
    Open the TY file and dump the audio
  -------------------------------------------------------------------------*/
  if (FALSE == tyda_demux_audio())
  {
    /*-----------------------------------------------------------------------
      Error dumping TY file.  Bail.
    -----------------------------------------------------------------------*/
    ret_val = -1;
    goto exit_and_clean_up;
  }


exit_and_clean_up:

  /*-------------------------------------------------------------------------
    Close files
  -------------------------------------------------------------------------*/
  if (tyda_info.in_file)
  {
    fclose(tyda_info.in_file);
  }

  if (tyda_info.out_file != -1)
  {
    close(tyda_info.out_file);
    if (ret_val == -1)
    {
      /*---------------------------------------------------------------------
        Demux failed.  Delete the output file, since its not to be trusted
      ----------------------------------------------------------------------*/
      fprintf(stderr, "Deleting failed output file, \'%s\'",
             tyda_info.out_file_name);
      DeleteFile(tyda_info.out_file_name);
    }
  }

  /* ------------------------------------------------------------------------
     Print an extra line before the final command prompt returns to the user
  ------------------------------------------------------------------------ */
  fprintf(stderr, "\n");

  /* ------------------------------------------------------------------------
     Exit process and return specified return value
  ------------------------------------------------------------------------ */
  return ret_val;

  
} /* main() */
#endif
