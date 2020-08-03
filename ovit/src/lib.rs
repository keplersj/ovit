extern crate apple_partition_map;
extern crate rayon;
extern crate tivo_media_file_system;

use apple_partition_map::ApplePartitionMap;
use log::{info, warn};
use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;
use tivo_media_file_system::{
    MFSINode, MFSVolumeHeader, MFSVolumes, MFSZone, MFSZoneMap, MFSZoneType, INODE_CHAINED_FLAG,
};

pub const TIVO_BOOT_MAGIC: u16 = 0x1492;
pub const TIVO_BOOT_AMIGC: u16 = 0x9214;

fn fsid_hash(fsid: u32, size: u32) -> u32 {
    // Prime number used in hash for finding base inode of fsid. (from mfstools)
    const FSID_HASH: u32 = 0x106d9;

    fsid.wrapping_mul(FSID_HASH) & (size)
}

#[derive(Debug)]
pub struct TivoDrive {
    pub source_file: File,
    pub partition_map: ApplePartitionMap,
    pub volume_header: MFSVolumeHeader,
    pub raw_zonemap: MFSZoneMap,
    pub volumes: MFSVolumes,
    pub zonemap: Vec<MFSZone>,
    pub is_byte_swapped: bool,
    inode_count: u32,
}

impl TivoDrive {
    fn check_byte_order(file: &mut File) -> Result<bool, String> {
        let mut buffer = [0; 2];
        match file.read_exact(&mut buffer) {
            Ok(_) => {}
            Err(_) => {
                return Err("Could not read first two bytes from file".to_string());
            }
        };

        match u16::from_be_bytes(buffer[0..2].try_into().unwrap()) {
            TIVO_BOOT_MAGIC => Ok(false),
            TIVO_BOOT_AMIGC => Ok(true),
            _ => Err("Not a TiVo Drive".to_string()),
        }
    }

    pub fn from_disk_image(path: &str) -> Result<TivoDrive, String> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(_) => {
                return Err("Couldn't open drive".to_string());
            }
        };

        let is_byte_swapped = TivoDrive::check_byte_order(&mut file)?;

        let partition_map = ApplePartitionMap::read_from_file(&mut file, is_byte_swapped)?;

        let mfs_partitions: MFSVolumes = MFSVolumes::new(&partition_map);

        let app_region = partition_map
            .partitions
            .iter()
            .find(|partition| partition.r#type == "MFS")
            .unwrap();

        let volume_header =
            MFSVolumeHeader::from_partition(app_region, &mut file, is_byte_swapped)?;

        let raw_zonemap = MFSZoneMap::new(
            path,
            &mfs_partitions,
            volume_header.next_zonemap_sector,
            volume_header.next_zonemap_backup_sector,
            volume_header.next_zonemap_partition_size as usize,
            is_byte_swapped,
        )?;

        // Messy but fine
        let zonemap: Vec<MFSZone> = Vec::from_iter(MFSZoneMap::new(
            path,
            &mfs_partitions,
            volume_header.next_zonemap_sector,
            volume_header.next_zonemap_backup_sector,
            volume_header.next_zonemap_partition_size as usize,
            is_byte_swapped,
        )?);

        let inode_count = zonemap
            .iter()
            .filter(|zone| zone.r#type == MFSZoneType::INode)
            .map(|zone| zone.size)
            .sum::<u32>();

        Ok(TivoDrive {
            source_file: file,
            partition_map,
            volume_header,
            volumes: mfs_partitions,
            raw_zonemap,
            zonemap,
            is_byte_swapped,
            inode_count,
        })
    }

    fn sector_for_inode(&mut self, inode: u32, backup: bool) -> u64 {
        let inode_count = self.inode_count;
        let sector: u64 = u64::from(inode) * 2;

        if inode >= inode_count {
            return 0;
        }

        for zone in self
            .zonemap
            .iter()
            .filter(|zone| zone.r#type == MFSZoneType::INode)
        {
            if sector < zone.size.into() {
                if backup {
                    return sector + zone.backup_sector + 1;
                } else {
                    return sector + zone.first_sector;
                }
            }
        }

        0
    }

    pub fn get_inode_from_fsid(&mut self, queried_fsid: u32) -> Result<MFSINode, String> {
        let inode = fsid_hash(queried_fsid, self.inode_count - 1);
        let sector = self.sector_for_inode(inode, false);

        let volume = self.volumes.find_sector_volume(sector);

        let hashed_inode = MFSINode::from_file_at_sector(
            &mut self.source_file,
            volume.disk_sector.into(),
            sector - u64::from(volume.sector_start),
            self.is_byte_swapped,
        )?;

        if hashed_inode.fsid == queried_fsid {
            return Ok(hashed_inode);
        }

        warn!("Couldn't find INode for FSID {} using hash.", queried_fsid);

        let sector = self.sector_for_inode(inode, true);

        let volume = self.volumes.find_sector_volume(sector);

        let hashed_inode = MFSINode::from_file_at_sector(
            &mut self.source_file,
            volume.disk_sector.into(),
            sector - u64::from(volume.sector_start),
            self.is_byte_swapped,
        )?;

        if hashed_inode.fsid == queried_fsid {
            return Ok(hashed_inode);
        }

        warn!(
            "Couldn't find INode for FSID {} using hash at backup location.",
            queried_fsid
        );

        let inode_id_base = hashed_inode.inode;
        let mut current_inode_id = inode;
        let mut current_inode = hashed_inode;

        while current_inode.fsid != queried_fsid
            && current_inode.flags == INODE_CHAINED_FLAG
            && ((current_inode_id + 1) % self.inode_count) != inode_id_base
        {
            current_inode_id += 1;
            let sector = self.sector_for_inode(current_inode_id, false).into();

            current_inode = MFSINode::from_file_at_sector(
                &mut self.source_file,
                self.volumes.find_sector_volume(sector).disk_sector.into(),
                sector,
                self.is_byte_swapped,
            )?;

            if current_inode.fsid == queried_fsid {
                info!(
                    "Found INode {} for FSID {} using looping.",
                    current_inode.inode, current_inode.fsid
                );
                return Ok(current_inode);
            };
        }

        warn!(
            "Couldn't find INode for FSID {} using looping.",
            queried_fsid
        );

        // Err(format!("Could not get INode for FSID {}", queried_fsid))

        info!(
            "Returning the original hashed INode even though the FSID {} was not found.",
            queried_fsid
        );

        Ok(MFSINode::from_file_at_sector(
            &mut self.source_file,
            self.volumes.find_sector_volume(sector).disk_sector.into(),
            sector.into(),
            self.is_byte_swapped,
        )?)
    }
}
