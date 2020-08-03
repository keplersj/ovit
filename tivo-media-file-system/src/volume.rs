extern crate apple_partition_map;

use apple_partition_map::ApplePartitionMap;

#[derive(Debug, Clone, Copy)]
pub struct MFSVolume {
    pub disk_sector: u32,
    pub sector_start: u32,
    pub sector_count: u32,
}

#[derive(Debug, Clone)]
pub struct MFSVolumes {
    volumes: Vec<MFSVolume>,
}

impl MFSVolumes {
    pub fn new(partition_map: &ApplePartitionMap) -> MFSVolumes {
        let volumes: Vec<MFSVolume> = partition_map
            .partitions
            .iter()
            .cloned()
            .filter(|partition| partition.r#type == "MFS")
            .fold(vec![], |mut acc, partition| {
                acc.push(MFSVolume {
                    disk_sector: partition.starting_sector,
                    sector_count: partition.sector_size,
                    sector_start: if acc.is_empty() {
                        0
                    } else {
                        acc[acc.len() - 1].sector_start + acc[acc.len() - 1].sector_count
                    },
                });
                acc
            });

        MFSVolumes { volumes }
    }

    pub fn find_sector_volume(&self, sector: u64) -> MFSVolume {
        *self
            .clone()
            .volumes
            .iter()
            // 34254847
            .find(|volume| {
                (volume.sector_start as u64) <= sector
                    && sector <= volume.sector_start as u64 + volume.sector_count as u64
            })
            .expect(&format!(
                "Could not find volume containing sector {}",
                sector
            ))
    }

    pub fn sector_to_disk_location(self, sector: u64) -> u64 {
        let volume = self.find_sector_volume(sector);

        let volume_relative_sector = sector - volume.sector_start as u64;
        let sector_on_disk = volume.disk_sector as u64 + volume_relative_sector;

        sector_on_disk
    }
}
