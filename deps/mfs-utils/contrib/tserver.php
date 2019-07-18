<?

// USER CONFIGURABLE SECTION... 
  
$tivoip = "192.168.0.23";
$tserverport = "3565";
$tivofile = "stream.ty";


// Hopefully nothing below here needs to be altered...


//simple fn to send a cmd to tserver, and read result back as stringdata
function doCmd($cmd)
{
    global $tivoip, $tserverport;

    $return="";     
      $fp = fsockopen( $tivoip, $tserverport, $errno, $errstr, 5);
      if (!$fp) {
         echo "$errstr ($errno)<br />\n";
      } else {
         stream_set_timeout($fp, 4);
         fwrite($fp, $cmd."\n");
         fflush($fp);

         while(!feof($fp))
         {
           $return .= fread($fp, 20000);
         }
         fclose($fp);
      }
    return $return;
}

//check for keys and create if not there to make it simpler later.
if(!array_key_exists("fsid",$_REQUEST)) $_REQUEST["fsid"] = "";
if(!array_key_exists("get",$_REQUEST)) $_REQUEST["get"] = "";


if( $_REQUEST["fsid"] == "" && $_REQUEST["get"]=="")
{
    //no params... show the list.

        echo "<HTML><HEAD><TITLE>Tserver (list)</TITLE></HEAD>";
        echo "<BODY>";

        $list = doCmd("SHOWING");
        //<Title>:<CSI: NY>
        //<Day>:<Sat>
        //<Date>:<4/30>
        //<Year>:<4/30/05>
        //<Station>:<FIVE>
        //<EpisodeTitle>:<Tanglewood>
        //<FSID>:<2433801/11>
        //<TyStream>:</2454504>
        //<TotalSize>:<116>

        
		$tivo_list_array = split("\n",$list);
		$tivo_prog_array = Array();
		foreach ( $tivo_list_array as $prog_data )
		{
            $prog_data = substr($prog_data, 1);
            if(substr($prog_data, -1)=='\n')
            {
              $prog_data = substr($prog_data, 0, -1);
            }
            if(substr($prog_data, -1)=='>')
            {
              $prog_data = substr($prog_data, 0, -1);
            }

            //echo htmlentities($prog_data);
			$info = split('><', $prog_data);
            //print_r($pair);
            $item = Array();
            for($i=0; $i<9; $i++)
            {
              if(array_key_exists($i, $info))
              {
                $pair = split('>:<',$info[$i]);
                if(array_key_exists(0, $pair) && array_key_exists(1, $pair))
                  $item[$pair[0]] = $pair[1];
              }
            }
            $tivo_prog_array[] = $item;
		}
        //print_r($tivo_prog_array);

        echo "<TABLE>";
        echo "<TR align=\"left\"><TH>Title</TH><TH>Day</TH><TH>Date</TH><TH>Year</TH><TH>Station</TH><TH>Episode</TH><TH>FSID</TH><TH>TyStream</TH><TH>TotalSize</TH></TR>";
        foreach ( $tivo_prog_array as $tivo_prog_data )
        {
            echo "<TR>";
            foreach ( $tivo_prog_data as $key => $value )
            {
              if($key == "TyStream")
              { 
                echo "<TD><A HREF=\"?fsid=".urlencode($value)."\">Get Sectorlist</A></TD>";
              }
              else
                echo "<TD>".$value."</TD>";
            }
            echo "</TR>";
        }
        echo "</table>";
        echo "</BODY></HTML>";

}
else if($_REQUEST["get"]=="")
{

    //no get param.. show fsid sector info... 

    echo "<HTML><HEAD><TITLE>Tserver (fsidinfo)</TITLE></HEAD>";
    echo "<BODY>";

    $list = doCmd("LISTSECTORS ".$_REQUEST["fsid"]);

    $fsid="";
    $fsid_array=Array();
    $fsid_list_array = split("\n",$list);
    foreach ( $fsid_list_array as $fsid_line)
    { 
        if( ereg("FSID:([0-9]+)", $fsid_line, $fsid_line_array ) )
        {
            $fsid=$fsid_line_array[1];
        }else if( ereg("DRV:([0-9]+) PART:([0-9]+) START:([0-9]+) COUNT:([0-9]+)", $fsid_line, $secinfo))
        {
            $fsid_array[$fsid][]=$secinfo;
        }
    }

    echo "<table border=\"1\"><tr><th colspan=\"6\" align=\"left\">fsid</th><tr><th></th><TH>Drive</TH><TH>Sector</TH><TH>Count</TH><TH>Sector+Count</TH><TH>FollowsLast?</TH></TR>";
    foreach ( $fsid_array as $fsid => $all_sector_info )
    {        
        $sec=0;
        echo "<tr bgcolor=\"#efefef\"><td colspan=\"6\">$fsid</td></tr>";
        foreach ( $all_sector_info as $sector_info )
        {
            $device = $sector_info[1];
            $part   = $sector_info[2];
            $sector = $sector_info[3];
            $count  = $sector_info[4];
        
            echo "<TR><td></td><TD>$device:$part</TD><TD>$sector</TD><TD>$count</TD><TD>".($sector+$count)."</TD><TD>";
            if($sec == $sector)
              echo "Y";
            else
              echo "&nbsp;";
            echo "</TD></TR>";

            $sec = $sector + $count;
        }       
    }
    echo "</table>";

    echo "<A HREF=\"?get=".$_REQUEST["fsid"]."\">Show Burst commands for this TYStream</A>";
    echo "</BODY></HTML>";
}else
{

    // must be a request for the burst commands... 

    echo "<HTML><HEAD><TITLE>Tserver (get)</TITLE></HEAD>";
    echo "<BODY>";

    $list = doCmd("LISTSECTORS ".$_REQUEST["get"]);

    $fsid="";
    $fsid_array=Array();
    $fsid_list_array = split("\n",$list);
    foreach ( $fsid_list_array as $fsid_line)
    { 
        if( ereg("FSID:([0-9]+)", $fsid_line, $fsid_line_array ) )
        {
            $fsid=$fsid_line_array[1];
        }else if( ereg("DRV:([0-9]+) PART:([0-9]+) START:([0-9]+) COUNT:([0-9]+)", $fsid_line, $secinfo))
        {
            $fsid_array[$fsid][]=$secinfo;
        }
    }


    $cmd_data = Array();

    
    foreach ( $fsid_array as $fsid => $all_sector_info )
    {        
        $sec=0;
        foreach ( $all_sector_info as $sector_info )
        {
            $drive  = $sector_info[1];
            $part   = $sector_info[2];
            $sector = $sector_info[3];
            $count  = $sector_info[4];

            if($sec == $sector)
            {
                $cmdcount = count($cmd_data);
                $lastcmd = $cmd_data[$cmdcount-1];
                $lastcmd[2] += $count;
                $cmd_data[$cmdcount-1]=$lastcmd;
            }             
            else
              $cmd_data[] = Array($drive.":".$part, $sector, ($sector+$count));

            $sec = $sector + $count;
        }       
    }

    echo "<pre>";
    $ccount=0;
    foreach ($cmd_data as $cmd)
    {
        echo "burst //$tivoip/".$cmd[0].":".$cmd[1]."-".$cmd[2];
        echo " part".$ccount.".ty";
        $ccount++;
        echo "\n";
    }
    echo "copy ";
    for($i=0; $i<$ccount; $i++)
    {
        echo "/b part".$i.".ty ";
        if($i<($ccount-1))
           echo "+ ";
    }
    echo $tivofile;

    echo "</pre>";

    echo "</BODY></HTML>";

}


?>
