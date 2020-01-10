# oViT Project

The oViT project represents the center point for an attempt at reverse engineering 1st Generation TiVo storage, using modern techniques and technologies. oViT's end goal is to successfully extract recorded programs from an early-2000's era TiVo hard drive, and play the extracted files back.

Because this project is currently more research project and less polished software, this README will contain all of the discovered and known information about the 1st Generation TiVo hard drive.

## The Hard Drive

oViT is being developed from the data from a [Sony SAT T-60 (Series 1) TiVo](http://www.tivopedia.com/model-sony-satt60.php). An ISO created using `dd` is being used for reverse engineering, for convenience and to avoid undue labor on the 20 year old mechanical drive.

## Drive Structure and Content

The cloned ISO is 40 Gigabytes large (matching the original hard drive) and contains the following data from the TiVo.

### Apple Partition Map

The hard drive was formatted with an Apple Partition Map in BigEndian format. Running `cargo run -p ovit-tools -- partitions tivo_hdd.iso` shows the following about the [partition map](https://en.wikipedia.org/wiki/Apple_Partition_Map) at the start of the cloned TiVo drive:

| Partition Total | Starting Sector | Sector Size | Name                       | Type                  | Starting Data Sector | Data Sectors | Status   |
| --------------- | --------------- | ----------- | -------------------------- | --------------------- | -------------------- | ------------ | -------- |
| 13              | 1               | 63          | `Apple`                    | `Apple_partition_map` | 0                    | 63           | 0x000033 |
| 13              | 43009349        | 4096        | `Bootstrap 1`              | `Image`               | 0                    | 4096         | 0x000033 |
| 13              | 43013445        | 4096        | `Kernel 1`                 | `Image`               | 0                    | 4096         | 0x000033 |
| 13              | 43017541        | 262144      | `Root 1`                   | `Ext2`                | 0                    | 262144       | 0x000033 |
| 13              | 43279685        | 4096        | `Bootstrap 2`              | `Image`               | 0                    | 4096         | 0x000033 |
| 13              | 43283781        | 4096        | `Kernel 2`                 | `Image`               | 0                    | 4096         | 0x000033 |
| 13              | 43287877        | 262144      | `Root 2`                   | `Ext2`                | 0                    | 262144       | 0x000033 |
| 13              | 43550021        | 131072      | `Linux swap`               | `Swap`                | 0                    | 131072       | 0x000033 |
| 13              | 43681093        | 262144      | `/var`                     | `Ext2`                | 0                    | 262144       | 0x000033 |
| 13              | 43943237        | 1048576     | `MFS application region`   | `MFS`                 | 0                    | 1048576      | 0x000033 |
| 13              | 46040389        | 32158361    | `MFS media region`         | `MFS`                 | 0                    | 32158361     | 0x000133 |
| 13              | 44991813        | 1048576     | `MFS application region 2` | `MFS`                 | 0                    | 1048576      | 0x000033 |
| 13              | 64              | 43009285    | `MFS media region 2`       | `MFS`                 | 0                    | 43009285     | 0x000133 |

### Media File System

At the time of writing oViT is able to serialize the following MFS data types:

- MFS Volume Header
- MFS Zones and Zone Maps
- MFS INode
  - Directories

#### Volume Header

Running `cargo run -p ovit-tools -- header /run/media/kepler/External/tivo_hdd.iso` shows the following information from the drive's volume header:

| Variable               | Value                                       |
| ---------------------- | ------------------------------------------- |
| State                  | 0                                           |
| Checksum               | 2053975265                                  |
| Root FSID              | 1                                           |
| First Partition Size   | 1024                                        |
| Partition List         | /dev/hda10 /dev/hda11 /dev/hda12 /dev/hda13 |
| Total Sectors          | 77263872                                    |
| Zonemap Sector         | 1121                                        |
| Zonemap Backup Sector  | 1048574                                     |
| Zonemap Partition Size | 524288                                      |
| Next FSID              | 12697810                                    |

#### Zones

Running `cargo run -p ovit-tools -- zones tivo_hdd.iso` shows the following about the different zones of the MFS partitions on the drive:

| Sector | Backup Sector | Zonemap Size | Next Zonemap Pointer | Backup Next Zonemap Pointer | Next Zonemap Size | Next Zonemap Partition Size | Next Zonemap Min. Allocation | Logstamp  | Type        | Checksum   | First Sector | Last Sector | Size     | Min. Allocations | Free Space | Bitmap Number |
| ------ | ------------- | ------------ | -------------------- | --------------------------- | ----------------- | --------------------------- | ---------------------------- | --------- | ----------- | ---------- | ------------ | ----------- | -------- | ---------------- | ---------- | ------------- |
| 1121   | 1048574       | 1            | 525410               | 1048565                     | 9                 | 32157696                    | 2048                         | 101141407 | INode       | 881335562  | 1122         | 525409      | 524288   | 524288           | 524288     | 1             |
| 525410 | 1048565       | 9            | 525419               | 1048531                     | 34                | 523072                      | 8                            | 104363088 | Media       | 2999402420 | 1048576      | 33206271    | 32157696 | 2048             | 1122304    | 15            |
| 525419 | 1048531       | 34           | 33206272             | 34254847                    | 1                 | 524288                      | 524288                       | 104363743 | Application | 3907386156 | 525453       | 1048524     | 523072   | 8                | 304760     | 17            |

#### INodes

Running `cargo run -p ovit-tools -- inodes tivo_hdd.iso -c 10` shows the first 10 INodes on the disk, as follows:

| FSID     | Reference Count | Boot Cycles | Boot Seconds | INode | Size | Block Size | Blocks Used | Last Modified           | Type   | Zone | Checksum   | Flags      | Number of Blocks |
| -------- | --------------- | ----------- | ------------ | ----- | ---- | ---------- | ----------- | ----------------------- | ------ | ---- | ---------- | ---------- | ---------------- |
| 0        | 0               | 201         | 96284190     | 0     | 8    | 0          | 0           | 2011-11-01 09:31:13 UTC | Db     | 2    | 874701711  | 1073741824 | 0                |
| 12654953 | 2               | 201         | 99111427     | 1     | 2624 | 0          | 0           | 2011-11-03 08:58:51 UTC | Db     | 2    | 671757100  | 0          | 1                |
| 0        | 0               | 199         | 28813273     | 2     | 8    | 0          | 0           | 2011-08-10 12:52:35 UTC | Db     | 2    | 24436353   | 1073741824 | 0                |
| 0        | 0               | 200         | 32039606     | 3     | 8    | 0          | 0           | 2011-08-28 02:44:29 UTC | Db     | 2    | 1390534620 | 1073741824 | 0                |
| 0        | 0               | 200         | 32043960     | 4     | 8    | 0          | 0           | 2011-08-28 02:53:28 UTC | Db     | 2    | 3751297821 | 1073741824 | 0                |
| 0        | 0               | 201         | 53233679     | 5     | 8    | 0          | 0           | 2011-10-02 10:59:31 UTC | Db     | 2    | 3186161858 | 1073741824 | 0                |
| 0        | 0               | 201         | 66951763     | 6     | 8    | 0          | 0           | 2011-10-12 09:32:39 UTC | Db     | 2    | 3391714744 | 1073741824 | 0                |
| 0        | 0               | 195         | 66562306     | 7     | 8    | 0          | 0           | 2011-03-08 09:48:00 UTC | Db     | 2    | 1760814817 | 1073741824 | 0                |
| 0        | 0               | 196         | 22734645     | 8     | 2128 | 131072     | 2128        | 2011-07-04 13:20:11 UTC | Stream | 1    | 3598525128 | 0          | 0                |
| 0        | 0               | 199         | 5885156      | 9     | 8    | 0          | 0           | 2011-07-23 00:31:46 UTC | Db     | 2    | 3289510782 | 1073741824 | 0                |

#### Files

Below are examples of the first two files successfully viewed from the TiVo Media Filesystem using the oViT FUSE Driver:

`/Server/A0003fc81:3:20:0`:

![PNG image at /Server/A0003fc81:3:20:0](./examples/A0003fc81:3:20:0.png)

`/Server/AR0000a96b:4:0:0.`:

![PNG image at /Server/AR0000a96b:4:0:0](./examples/AR0000a96b:4:0:0.png)

---

# oViT Information

oViT is implemented using Rust, with the goal of creating robust and memory-safe utilities that'll last for years to come. TiVo became a cultural touchstone in the early 2000's. Due to its cultural significance alone it should be possible to archive content from TiVo hard drives and preserve them. The tapes of previous generations are easily accessible and preservable today, the recordings from the TiVo era should be too.

oViT is build using the following libraries:

- [`nom`](https://crates.io/crates/nom) - for parsing Apple Partition Map data and Media File System data
- [`clap`](https://crates.io/crates/clap) - for creating oViT's command line interfaces
- [`chrono`](https://crates.io/crates/chrono) - for parsing time and date information present in the Media File System

## See Also

- [Wikipedia entry on the TiVo Media File System](https://en.wikipedia.org/wiki/TiVo_Media_File_System)

  The Wikipedia entry on the TiVo Media File System provides a decent abstract of the file system format and how it was created.

- [Wikipedia entry on the Apple Partition Map](https://en.wikipedia.org/wiki/Apple_Partition_Map)

  The Wikipedia entry on the Apple Partition Map provides a very good abstract of the partitioning scheme used on 1st Generation TiVo hard drives.

- [MFS Tools on the TiVo Community Forum](https://www.tivocommunity.com/community/index.php?threads/mfs-tools-3-2.529148/)

  MFS Tools is an open source CLI utility for interacting with TiVo Hard Drives. It's primary use case is for upgrading the hard drives in a functioning TiVo unit, not extracting data from the drive of an offline TiVo hard drive. However, the source code serves a good reference.

- [TygerStripe/mfstools on GitHub](https://github.com/TygerStripe/mfstools)

  The C source code for MFS Tools available on GitHub, helpful for reference. The way MFS Tools allocates portions of the storage to C structs is incredibly helpful for understanding how to read the drive. oViT opts for data parsing over allocation, in the name of verbosity and memory safety.

- [_Battling Bit Rot, Link Rot, and Chaos: Hacking an 18 Year Old TiVo Hard Drive_ by Kepler Sticka-Jones](https://keplersj.com/blog/2019-10-12-battling-bit-rot-link-rot-and-chaos-hacking-an-18-year-old-tivo-hard-drive/)

  This October 12, 2019 entry on my blog details how oViT came into being, and explains the project's motivation and the early thought process.

- [_‘They Thought It Was Black Magic’: An Oral History of TiVo_ by Tom Roston](https://onezero.medium.com/they-thought-it-was-black-magic-an-oral-history-of-tivo-7503d0ada8e0)

  Although this article doesn't contain much technical detail about early TiVo devices, it does provide some general context to the creation of TiVo and explains some of the decisions made by the early TiVo development team. Worth a read if you are considering reverse engineering early TiVo data.

## License

Copyright 2019-2020 [Kepler Sticka-Jones](https://keplersj.com/). All Rights Reserved.

Note: Due to the current very unpolished state of the project, I am retaining all rights. In the future I hope to license the code under MIT, when the project becomes more polished and production ready.
