# oViT Project

The oViT project represents the center point for an attempt at reverse engineering 1st Generation TiVo storage, using modern techniques and technologies. oViT's end goal is to successfully extract recorded programs from an early-2000's era TiVo hard drive, and play the extracted files back.

Because this project is currently more research project and less polished software, this README will contain all of the discovered and known information about the 1st Generation TiVo hard drive.

## The Hard Drive

oViT is being developed from the data from a [Sony SAT T-60 (Series 1) TiVo](http://www.tivopedia.com/model-sony-satt60.php). An ISO created using `dd` is being used for reverse engineering, for convenience and to avoid undue labor on the 20 year old mechanical drive.

## Drive Structure and Content

The cloned ISO is 40 Gigabytes large (matching the original hard drive) and contains the following data from the TiVo.

### Apple Partition Map

The hard drive was formatted with an Apple Partition Map in BigEndian format. The partition map at the start of the drive contains the following information about the drive:

| Partition Name             | Partition Type        |
| -------------------------- | --------------------- |
| `Apple`                    | `Apple_partition_map` |
| `Bootstrap 1`              | `Image`               |
| `Kernel 1`                 | `Image`               |
| `Root 1`                   | `Ext2`                |
| `Bootstrap 2`              | `Image`               |
| `Kernel 2`                 | `Image`               |
| `Root 2`                   | `Ext2`                |
| `Linux swap`               | `Swap`                |
| `/var`                     | `Ext2`                |
| `MFS application region`   | `MFS`                 |
| `MFS media region`         | `MFS`                 |
| `MFS application region 2` | `MFS`                 |
| `MFS media region 2`       | `MFS`                 |

### Media File System

At the time of writing oViT is able to serialize the following MFS data types:

- MFS Volume Header
- MFS Zones and Zone Maps
- MFS INode

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
