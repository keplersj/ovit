#!/tvbin/tivosh

# TiVo web server written by Stephen Rothwell (sfr@linuxcare.com.au)
# SendKeys Tivo Remote Mod by Jon Squire (jsquire@justice.loyola.edu)
# Note: SendKeys TiVo remote has currently only been tested on a 
# DirecTiVo, but should be fine on others, if a key doesn't work
# look at your sendkeys.tcl file and makesure that the proper key
# is being sent.
#
# Remount added 11/10/2000 1:44am EST but Jon Squire
#
# Support TiVo software versions 4 and above  2004/12/10  Jamie
#
source $tcl_library/tv/log.tcl
source $tcl_library/tv/mfslib.tcl

global source_dir
set source_dir [file dirname [info script]]

proc strim {str} {
    return [string trim $str "\{\}"]
}

proc defaultval {val1 val2} {
        if { $val2 != "" } {
                return $val2
        } else {
                return $val1
        }
}

proc PrintNames {names} {
    set str ""
    foreach name $names {
        if { [regexp {(.*)\|(.*)} $name junk last first] } {
            if { $str == "" } {
                set str "$first $last"
            } else {
                set str "$str, $first $last"
            }
        }
    }
    return $str
}

proc ForeachMfsFileTrans { idVar nameVar typeVar dirName prefix count body } {
    global errorInfo errorCode

    upvar $idVar   id
    upvar $nameVar name
    upvar $typeVar type

    # Get the first batch of names
    RetryTransaction {
        if { [catch {mfs scan $dirName -start $prefix -count $count} batch] } {
            global errorCode errorInfo
            if { $errorCode == "errNmNameNotFound" } {
                return
            } else {
                error $batch $errorInfo $errorCode
            }
        }
    }

    set done 0
    while { [llength $batch] > 0 } {
        # Execute the body for each item in this batch
        RetryTransaction {
            foreach item $batch {
                set id   [lindex $item 0]
                set name [lindex $item 1]
                set type [lindex $item 2]

                # bail if we're past the entries that start with the given prefix
                if { ! [PrefixMatches $prefix $name] } {
                    set done 1
                    break
                }

                set code [catch {uplevel $body} string]
                if { $code == 1 } {
                    global errorCode errorInfo
                    if { $errorCode == "errTmActiveLockConflict" ||
                         $errorCode == "errFsLockConflict" } {
                        error $batch $errorInfo $errorCode
                    } else {
                        set done 2
                        break
                    }
                } elseif { $code == 3 } {
                    # this is a break in the body.  just return normally
                    set done 3
                    break
                } elseif { $code != 0 } {
                    set done 4
                    break
                }
            }
        }

        switch -exact $done {
            1 {return}
            2 {return -code error -errorinfo $errorInfo \
                      -errorcode $errorCode $string}
            3 {return}
            4 {return -code $code $string}
        }

        # Get the next batch
        set lastName [lindex [lindex $batch end] 1]
        RetryTransaction {
            set batch [mfs scan $dirName -start $lastName -count $count]
            if { $lastName == [lindex [lindex $batch 0] 1] } {
                set batch [lrange $batch 1 end]
            }
        }
    }
}

proc action_nowshowing { chan } {
    global db
    global tzoffset
    global images
    global cache_ns_rec
    global cache_ns_series

    global tivoswversion
    global version


    if {$::version >= 3} {
      # puts "We are version 3+"
      set nowshowingkey "/Recording/NowShowingByClassic"
    } else {
      # puts "We are version 2"
      set nowshowingkey "/Recording/NowShowing"
    }

# This is the first change. Pick the MFS key based on OS version.
    ForeachMfsFileTrans fsid name type "$nowshowingkey" "" 10 {
        set rec [db $db openid $fsid]
        set seltype [dbobj $rec get SelectionType]
        set showing [dbobj $rec get Showing]
        set showingfsid [dbobj $rec gettarget Showing]
        set station [dbobj $showing get Station]
        set stationid [dbobj $station get CallSign]
        set program [dbobj $showing get Program]
        set title [strim [dbobj $program get Title]]
        set seltype [dbobj $rec get SelectionType]

        if { $seltype == 10 || $seltype == 5 } {
            if { $title == "" } {
               set title "Manual Recording"
            } else {
               set title "Manual: $title"
            }
        }
        set episodic 0
        set episode ""
        set series [dbobj $program get Series]
        if { $series != "" } {
           set episodic [dbobj $series get Episodic]
        }

        # 4-19-2002 - A new check to get episode titles working on the DTivo.
        set tmstype 1
        set tmsid [dbobj $program get TmsId]
        if { [string range $tmsid 0 1] == "MV" } {
           set tmstype 0
        }
        if { [string length $tmsid] == 0 } {
           set tmstype $episodic
        }

        if { $episodic == 1  || $tmstype == 1 } {
           set partindex [dbobj $showing get PartIndex]
           set partcount [dbobj $showing get PartCount]
           set partstr ""
           if { $partcount != "" && $partindex != "" } {
              set partstr " ($partindex of $partcount)"
           }
           set episode [strim [dbobj $program get EpisodeTitle]]$partstr
        }


        set seconds [expr [dbobj $showing get Date] * 86400 + [dbobj $showing get Time] + $tzoffset]
        set day [clock format $seconds -format "%a"]
        set date [clock format $seconds -format "%1m/%1d"]
        set year [clock format $seconds -format "%1m/%1d/%2y"]
        set expdate [dbobj $rec get ExpirationDate]
        set expsecs [expr $expdate * 86400 + [dbobj $rec get ExpirationTime]]
        set nowsecs [clock seconds]
        set parts [dbobj $rec get Part]
        set totalSize 0
        set tystreams ""
        puts -nonewline $chan "<Title>:<$title>"
        puts -nonewline $chan "<Day>:<$day>"
        puts -nonewline $chan "<Date>:<$date>"
        puts -nonewline $chan "<Year>:<$year>"
        puts -nonewline $chan "<Station>:<$stationid>"
        puts -nonewline $chan "<EpisodeTitle>:<$episode>"
        puts -nonewline $chan "<FSID>:<$showingfsid>"
        puts -nonewline $chan "<TyStream>:<"
        foreach part $parts {
            set file [dbobj $part get File]
#            puts -nonewline $chan "      <Part>$file</Part>"
            puts -nonewline $chan "/$file"
            if { [catch {mfs streamsize $file} sizes] } {
                    
            } else {
                set cbStream [expr ([lindex $sizes 0] / 1024) * [lindex $sizes 1] / 1024]
#                puts -nonnewline $chan "<Size>:<$cbStream>"
                incr totalSize $cbStream
            }
        }
        puts -nonewline $chan ">"
        puts $chan "<TotalSize>:<$totalSize>"
    }
}

proc get_tzoffset {mfstz dst} {
   if { $mfstz <= 0 } {
      set tz $mfstz
   } else {
      set tzlist "-5 -6 -7 -8 -9 -10 0 1 2 3 4 5 6 7 8 9 10 11 12 -1 -2 -3 -4 -11 -12"
      set tz [lindex $tzlist [expr $mfstz - 1]]
   }
   if { $dst == 2 || $dst == "" } {
      set date [clock format [clock seconds] -format "%1d %u %1m %1H %1M"]
      scan $date "%d %d %d %d %d" dom dow month hour min
      if {$month > 4 && $month < 10} {
         set dlsval 1
      } elseif {$month == 4 && $dom > 7} {
         set dlsval 1
      } elseif {$month == 4 && $dom <= 7 && $dow == 0 && $hour >= 2} {
         set dlsval 1
      } elseif {$month == 4 && $dom <= 7 && $dow != 0 && ($dom-$dow > 0)} {
         set dlsval 1
      } elseif {$month == 10 && $dom < 25} {
         set dlsval 1
      } elseif {$month == 10 && $dom >= 25 && $dow == 0 && $hour < 2} {
         set dlsval 1
      } elseif {$month == 10 && $dom >= 25 && $dow != 0 && ($dom-24-$dow < 1) } {
         set dlsval 1
      } else {
         set dlsval 0
      }
      if {$dlsval == 1} {
         return [expr ($tz+1)*60*60]
      } else {
         return [expr $tz*60*60]
      }
   } else {
      return [expr $tz*60*60]
   }
}

proc GetResourceData { refid startid endid startindex bits } {
   global db

   set keys ""
   set vals ""
   RetryTransaction {
      set sws [db $db open /SwSystem/ACTIVE]
      set offset [expr $refid/65536 - 1]
      set resource [lindex [dbobj $sws get ResourceGroup] $offset]
      set id [dbobj $resource get Id]
      if { $id == $refid } {
         set items [dbobj $resource get Item]
         foreach item $items {
            set id [dbobj $item get Id]
            if { $id >= $startid && $id <= $endid } {
               if { $bits == 1 } {
                  set value [expr 1 << ($id - $startid + $startindex)]
               } else {
                  set value [expr $id - $startid + $startindex]
               }
               set strval [dbobj $item get String]
               if { [string compare $strval "{}"] != 0 } {
                  lappend keys $value
                  lappend vals $strval
               }
            }
         }
      } else {
         puts "Error: refid didn't match"
      }
   }
   return [list $keys $vals]
}

proc GetOSVersion {} {
   global db
   global tivoswversion
   global version

   RetryTransaction {
      set sws [db $db open /SwSystem/ACTIVE]

      set tivoswversion [dbobj $sws get Name]
       if { [regexp {^[0-9]+} $tivoswversion v] } {
	   set ::version $v
       } else {
	   puts "Error: couldn't find TiVo software version"
       }
   }
}

proc DeleteShow {chan recfsid} {
  global db

  if {[string index $recfsid 0] == "/"} {
    set recfsid [string range $recfsid 1 end]
  }

#  puts -nonewline $chan "recfsid = $recfsid\r\n"
  set index [string first "/" $recfsid]
  if { $index != -1 } {
    set index [expr ($index - 1)]
    set recfsid [string range $recfsid 0 $index]
  }

#  puts -nonewline $chan "found it at: $recfsid\r\n"

#  set success 1
  set success [DeleteNowShowingRec $recfsid]

  return $success
}

proc DeleteNowShowingRec { recfsid } {
   global db

   set canceldate [expr [clock seconds] / 86400]
   set canceltime [expr [clock seconds] % 86400]

   RetryTransaction {
      set rec [db $db openid $recfsid]
      set state [dbobj $rec get State]
      if { $state != 4 } {
         return 0
      } else {
         dbobj $rec set CancelReason 12
         dbobj $rec set DeletionDate $canceldate
         dbobj $rec set DeletionTime $canceltime
         set errorstring [dbobj $rec get ErrorString]
         set elength [string length $errorstring]
         if { $elength > 0 } {
            set errorstring [string trim $errorstring "\{\}"]
            dbobj $rec set ErrorString "$errorstring Deleted by user"
         } else {
            dbobj $rec set ErrorString "Deleted by user"
         }
         dbobj $rec set State 5
      }
   }
   return 1
}

proc init_db {} {
   global db

   global genrenums
   global genrevals
   global tvratingnums
   global tvratingvals
   global showingbitnums
   global showingbitvals
   global advisorynums
   global advisoryvals
   global mpaaratingnums
   global mpaaratingvals

   global tzoffset
   global defrecquality

   global cancelreasons
   global selectiontypes
   global states
   global showtypes

   global images

   # This is the 2nd change.
   # Let's put but 2.5 and 3.0 in the same NowShowing.tcl.
   global tivoswversion
   global version

   GetOSVersion

   if {$::version >= 4} {
      # puts "init_db: We are version 4+"

     RetryTransaction {
       set lconfig  [db $db open /State/LocationConfig]
       set tzoffset [dbobj $lconfig get TimeZoneOffset]
     }
   } else {
       if {$::version >= 3} {
	   # puts "init_db: We are version 3"

	   RetryTransaction {
	       set lconfig  [db $db open /State/LocationConfig]
	       set setuptz [dbobj $lconfig get TimeZoneOld]
	       set daylightsavings [dbobj $lconfig get DaylightSavingsPolicy]
	   }
       } else {
	   # puts "init_db: We are version 2"
	   
	   RetryTransaction {
	       set setup [db $db open /Setup]
	       
	       #       set defrecquality [dbobj $setup get RecordQuality]
	       set setuptz [dbobj $setup get TimeZone]
	       set daylightsavings [dbobj $setup get DaylightSavingsPolicy]
	   }
       }
       set tzoffset [get_tzoffset $setuptz $daylightsavings]
   }

#   set cancelreasons "SwitchToLiveTv RecordDifferentShowing StayOnLiveTv InternalError PowerWasOff Expired GotBetterSuggestion DemoMode UnexpectedConflict UserRequestedRecording UserRequestedSeasonPass ExplicitlyDeleted ChannelLineupChanged ProgramGuideChanged RecorderEmergency UserCancelledSeasonPass FuzziesTurnedOff FuzzyStoppedEarly Unknown ProgramSourceConflict ConvertedLiveCache LiveCacheOnlySuccessful ProgramSourceDiskConflict ExplicitlyDeletedFromToDo ProgramSourceModified NotAuthorized NoReRecord NoSignal MaxRecordingsExceeded"
#   set selectiontypes "{Show Recommendation} {Package Recommendation} {By Name} {By Channel} Timer {Fuzzy Show} {Fuzzy Package} Bookmark {Season Pass} {Manual Season Pass} Guide IPreview WishListPass Extended"
#   set states "{To Do} Cancelled {In Progress} {Now Showing} Deleted {To Do}"
#   set showtypes "Serial {Short Film} Special {Limited Series} Series Miniseries {Paid Programming}"
#
#   set images "ExpireNever-256.8.png SelectIcon-256.9.png ThumbUp2-256.8.png ExpireSoon-256.8.png ThumbDn1-256.8.png Expired-256.8.png ThumbDn2-256.8.png ThumbUp3-256.9.png Recording-256.9.png ThumbDn3-256.8.png TivoSuggest-256.8.png SeasonPass.9.png ThumbUp1-256.8.png WishListPass.2.png LargeSquarePredicted3DownThumbs.3.png LargeSquarePredicted2DownThumbs.3.png LargeSquarePredicted1DownThumbs.3.png LargeSquarePredicted1UpThumbs.3.png LargeSquarePredicted2UpThumbs.3.png LargeSquarePredicted3UpThumbs.3.png"
#
#   set genres [GetResourceData 720896 720913 721031 1 0]
#   set genrenums [lindex $genres 0]
#   set genrevals [lindex $genres 1]
#
#   set tvratings [GetResourceData 983040 983065 983070 1 0]
#   set tvratingnums [lindex $tvratings 0]
#   set tvratingvals [lindex $tvratings 1]
#
#   set showingbits [GetResourceData 1572864 1572915 1572935 0 1]
#   set showingbitnums [lindex $showingbits 0]
#   set showingbitvals [lindex $showingbits 1]
#
#   set advisorys [GetResourceData 1048576 1048605 1048614 1 0]
#   set advisorynums [lindex $advisorys 0]
#   set advisoryvals [lindex $advisorys 1]
#
#   set mpaaratings [GetResourceData 65536 65567 65578 1 0]
#   set mpaaratingnums [lindex $mpaaratings 0]
#   set mpaaratingvals [lindex $mpaaratings 1]
#
}

# Get the args so we know what mode we are in.
set arg0 [lindex $argv 0]
set arg1 [lindex $argv 1]
set arg2 [lindex $argv 2]

# puts "0 == $arg0"
# puts "1 == $arg1"
# puts "2 == $arg2"

global db
set dbPoolSize [expr 100 * 1024]
set db [dbopen $dbPoolSize]

set chan stdout

if { $arg0 == "DELETE" } {

#  puts "\r\ndelete string: $arg1\r\n"

  set ok 1
  foreach record [split $arg1 " ,"] {
#    puts "record: $record\r\n"
    set ret [DeleteShow $chan $record]

    if { $ret == 0 } {
      set ok 0
    }
  }

  if { $ok == 1 } {
    puts -nonewline $chan "Delete Successful!\r\n"
  }  else {
    puts -nonewline $chan "Delete Incomplete...\r\n"
  }

} else {

  init_db

  action_nowshowing $chan

}

catch { flush $chan }
close $chan

