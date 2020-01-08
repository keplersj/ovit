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

#### Zones

Running `cargo run -p ovit-tools -- zones tivo_hdd.iso` shows the following about the different zones of the MFS partitions on the drive:

| Sector | Backup Sector | Zonemap Size | Next Zonemap Pointer | Backup Next Zonemap Pointer | Next Zonemap Size | Next Zonemap Partition Size | Next Zonemap Min. Allocation | Logstamp  | Type        | Checksum   | First Sector | Last Sector | Size     | Min. Allocations | Free Space | Bitmap Number |
| ------ | ------------- | ------------ | -------------------- | --------------------------- | ----------------- | --------------------------- | ---------------------------- | --------- | ----------- | ---------- | ------------ | ----------- | -------- | ---------------- | ---------- | ------------- |
| 1121   | 1048574       | 1            | 525410               | 1048565                     | 9                 | 32157696                    | 2048                         | 101141407 | INode       | 881335562  | 1122         | 525409      | 524288   | 524288           | 524288     | 1             |
| 525410 | 1048565       | 9            | 525419               | 1048531                     | 34                | 523072                      | 8                            | 104363088 | Media       | 2999402420 | 1048576      | 33206271    | 32157696 | 2048             | 1122304    | 15            |
| 525419 | 1048531       | 34           | 33206272             | 34254847                    | 1                 | 524288                      | 524288                       | 104363743 | Application | 3907386156 | 525453       | 1048524     | 523072   | 8                | 304760     | 17            |

---

# oViT Information

oViT is implemented using Rust

## See Also

- [Wikipedia entry on the TiVo Media File System](https://en.wikipedia.org/wiki/TiVo_Media_File_System)

  The Wikipedia entry on the TiVo Media File System provides a decent abstract of the file system format and how it was created.

- [Wikipedia entry on the Apple Partition Map](https://en.wikipedia.org/wiki/Apple_Partition_Map)

  The Wikipedia entry on the Apple Partition Map provides a very good abstract of the partitioning scheme used on 1st Generation TiVo hard drives.

- [MFS Tools on the TiVo Community Forum](https://www.tivocommunity.com/community/index.php?threads/mfs-tools-3-2.529148/)

  MFS Tools is an open source CLI utility for interacting with TiVo Hard Drives. It's primary use case is for upgrading the hard drives in a functioning TiVo unit, not extracting data from the drive of an offline TiVo hard drive. However, the source code serves a good reference.

- [_Battling Bit Rot, Link Rot, and Chaos: Hacking an 18 Year Old TiVo Hard Drive_ by Kepler Sticka-Jones](https://keplersj.com/blog/2019-10-12-battling-bit-rot-link-rot-and-chaos-hacking-an-18-year-old-tivo-hard-drive/)

  This October 12, 2019 entry on my blog details how oViT came into being, and explains the project's motivation and the early thought process.

- [_‘They Thought It Was Black Magic’: An Oral History of TiVo_ by Tom Roston](https://onezero.medium.com/they-thought-it-was-black-magic-an-oral-history-of-tivo-7503d0ada8e0)

  Although this article doesn't contain much technical detail about early TiVo devices, it does provide some general context to the creation of TiVo and explains some of the decisions made by the early TiVo development team. Worth a read if you are considering reverse engineering early TiVo data.

## License

Copyright 2019-2020 [Kepler Sticka-Jones](https://keplersj.com/). All Rights Reserved.

Note: Due to the current very unpolished state of the project, I am retaining all rights. In the future I hope to license the code under MIT, when the project becomes more polished and production ready.
