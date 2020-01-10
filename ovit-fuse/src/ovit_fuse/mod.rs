extern crate ovit;
extern crate tivo_media_file_system;

use fuse_mt::{
    DirectoryEntry, FileAttr, FileType, FilesystemMT, RequestInfo, ResultEmpty, ResultEntry,
    ResultOpen, ResultReaddir,
};
use ovit::TivoDrive;
use std::convert::TryInto;
use std::ffi::OsString;
use std::path::{Component, Path};
use time::Timespec;
use tivo_media_file_system::MFSINodeType;

pub struct TiVoFS {
    pub drive_location: String,
}

const TTL: Timespec = Timespec { sec: 1, nsec: 0 };

fn get_tivo_drive(location: String) -> Result<TivoDrive, i32> {
    match TivoDrive::from_disk_image(&location) {
        Ok(drive) => Ok(drive),
        Err(_err) => Err(0),
    }
}

fn get_fsid_from_path(path: &Path, disk_location: String) -> Result<u32, i32> {
    let mut previous_fsid: u32 = 0;

    let fsids: Vec<Option<u32>> = path
        .components()
        .map(|component| match component {
            Component::RootDir => {
                previous_fsid = 1;
                Some(1)
            }
            Component::Normal(path) => {
                if previous_fsid == 0 {
                    return None;
                }

                let native_path = match path.to_owned().into_string() {
                    Ok(string) => string,
                    Err(_err) => return None,
                };

                match &mut get_tivo_drive(disk_location.clone()) {
                    Ok(tivo_drive) => match tivo_drive.get_inode_from_fsid(previous_fsid) {
                        Ok(inode) => {
                            match inode.get_entries_from_directory(disk_location.clone()) {
                                Ok(entries) => {
                                    match entries.iter().find(|entry| entry.name == native_path) {
                                        Some(entry) => Some(entry.fsid),
                                        None => None,
                                    }
                                }
                                Err(_err) => None,
                            }
                        }
                        Err(_err) => None,
                    },
                    Err(_err) => None,
                }
            }
            _ => None,
        })
        .collect();

    if fsids.is_empty() {
        return Err(0);
    }

    match fsids.last() {
        Some(fsid_option) => match fsid_option {
            Some(fsid) => Ok(*fsid),
            None => Err(0),
        },
        None => Err(0),
    }
}

impl FilesystemMT for TiVoFS {
    fn init(&self, _req: RequestInfo) -> ResultEmpty {
        println!("init");
        Ok(())
    }

    fn destroy(&self, _req: RequestInfo) {
        println!("destroy");
    }

    fn getattr(&self, _req: RequestInfo, path: &Path, fh: Option<u64>) -> ResultEntry {
        println!("getattr: {:?}", path);

        let fsid = get_fsid_from_path(path, self.drive_location.clone())?;

        match get_tivo_drive(self.drive_location.clone())?.get_inode_from_fsid(fsid) {
            Ok(inode) => Ok((
                TTL,
                FileAttr {
                    size: 1,
                    blocks: 1,
                    atime: TTL,
                    mtime: Timespec {
                        sec: inode.last_modified.timestamp(),
                        nsec: 0,
                    },
                    ctime: TTL,
                    crtime: TTL,
                    kind: if inode.r#type == MFSINodeType::Dir {
                        FileType::Directory
                    } else {
                        FileType::RegularFile
                    },
                    perm: 777,
                    nlink: 0,
                    uid: 1000,
                    gid: 1000,
                    rdev: 0,
                    flags: 0,
                },
            )),
            Err(_err) => Err(0),
        }
    }

    fn opendir(&self, _req: RequestInfo, path: &Path, _flags: u32) -> ResultOpen {
        println!("opendir path: {:#?}", path);

        let fsid = get_fsid_from_path(path, self.drive_location.clone())?;

        Ok((fsid.try_into().unwrap(), 0))
    }

    fn readdir(&self, _req: RequestInfo, path: &Path, _fh: u64) -> ResultReaddir {
        println!("readdir path: {:#?}", path);

        let fsid = get_fsid_from_path(path, self.drive_location.clone())?;

        let mut tivo_drive = match TivoDrive::from_disk_image(&self.drive_location) {
            Ok(drive) => drive,
            Err(_err) => return Err(0),
        };

        match tivo_drive.get_inode_from_fsid(fsid.try_into().unwrap()) {
            Ok(inode) => match inode.get_entries_from_directory(self.drive_location.clone()) {
                Ok(entries) => Ok(entries
                    .iter()
                    .map(|entry| -> DirectoryEntry {
                        DirectoryEntry {
                            kind: if entry.r#type == MFSINodeType::Dir {
                                FileType::Directory
                            } else {
                                FileType::RegularFile
                            },
                            name: OsString::from(entry.name.clone()),
                        }
                    })
                    .collect()),
                Err(_err) => Err(0),
            },
            Err(_err) => Err(0),
        }
    }
}
