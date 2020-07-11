extern crate ovit;
extern crate rayon;
extern crate tivo_media_file_system;

use fuse_mt::{
    DirectoryEntry, FileAttr, FileType, FilesystemMT, RequestInfo, ResultEmpty, ResultEntry,
    ResultOpen, ResultReaddir,
};
use ovit::TivoDrive;
use rayon::prelude::*;
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
                                        Some(entry) => {
                                            previous_fsid = entry.fsid;
                                            Some(entry.fsid)
                                        }
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

    fn getattr(&self, _req: RequestInfo, path: &Path, _fh: Option<u64>) -> ResultEntry {
        println!("getattr: {:?}", path);

        let fsid = match get_fsid_from_path(path, self.drive_location.clone()) {
            Ok(fsid) => fsid,
            Err(_err) => {
                println!("Could not get FSID for path {:?}", path);
                return Err(0);
            }
        };

        match get_tivo_drive(self.drive_location.clone())?.get_inode_from_fsid(fsid) {
            Ok(inode) => Ok((
                TTL,
                FileAttr {
                    size: u64::from(inode.size),
                    blocks: u64::from(inode.blocksize),
                    atime: TTL,
                    mtime: Timespec {
                        sec: inode.last_modified.timestamp(),
                        nsec: 0,
                    },
                    ctime: TTL,
                    crtime: TTL,
                    kind: match inode.r#type {
                        MFSINodeType::Dir => FileType::Directory,
                        MFSINodeType::Node => FileType::RegularFile,
                        MFSINodeType::Db => FileType::RegularFile,
                        MFSINodeType::File => FileType::RegularFile,
                        _ => FileType::RegularFile,
                    },
                    perm: 777,
                    nlink: 0,
                    uid: 1000,
                    gid: 1000,
                    rdev: 0,
                    flags: 0,
                },
            )),
            Err(_err) => {
                println!("getattr({:?}): File has an FSID from a parent directory, but INode could not be read. Creating a dummy file to maintain structure.", path);
                Ok((
                    TTL,
                    FileAttr {
                        size: 0,
                        blocks: 0,
                        atime: TTL,
                        mtime: TTL,
                        ctime: TTL,
                        crtime: TTL,
                        kind: FileType::RegularFile,
                        perm: 777,
                        nlink: 0,
                        uid: 1000,
                        gid: 1000,
                        rdev: 0,
                        flags: 0,
                    },
                ))
                // Err(0)
            }
        }
    }

    fn opendir(&self, _req: RequestInfo, path: &Path, _flags: u32) -> ResultOpen {
        println!("opendir path: {:#?}", path);

        let fsid = get_fsid_from_path(path, self.drive_location.clone())?;

        Ok((u64::from(fsid), 0))
    }

    fn readdir(&self, _req: RequestInfo, path: &Path, _fh: u64) -> ResultReaddir {
        println!("readdir path: {:#?}", path);

        let fsid = get_fsid_from_path(path, self.drive_location.clone())?;

        let mut tivo_drive = get_tivo_drive(self.drive_location.clone())?;

        match tivo_drive.get_inode_from_fsid(fsid) {
            Ok(inode) => match inode.get_entries_from_directory(self.drive_location.clone()) {
                Ok(entries) => Ok(entries
                    // .iter()
                    .par_iter()
                    .filter(|entry| entry.name != "")
                    // .filter(
                    //     |entry| match &mut get_tivo_drive(self.drive_location.clone()) {
                    //         Ok(tivo_drive) => match tivo_drive.get_inode_from_fsid(entry.fsid) {
                    //             Ok(inode) => true,
                    //             Err(_err) => false,
                    //         },
                    //         Err(_err) => false,
                    //     },
                    // )
                    .map(|entry| -> DirectoryEntry {
                        DirectoryEntry {
                            kind: match inode.r#type {
                                MFSINodeType::Dir => FileType::Directory,
                                MFSINodeType::Node => FileType::RegularFile,
                                MFSINodeType::Db => FileType::RegularFile,
                                MFSINodeType::File => FileType::RegularFile,
                                _ => FileType::RegularFile,
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

    fn open(&self, _req: RequestInfo, path: &Path, _flags: u32) -> ResultOpen {
        println!("open path: {:#?}", path);

        let fsid = get_fsid_from_path(path, self.drive_location.clone())?;

        Ok((u64::from(fsid), 0))
    }

    fn read(
        &self,
        _req: RequestInfo,
        path: &Path,
        _fh: u64,
        _offset: u64,
        _size: u32,
        result: impl FnOnce(Result<&[u8], i32>),
    ) {
        println!("read path: {:#?}", path);

        match get_fsid_from_path(path, self.drive_location.clone()) {
            Ok(fsid) => match &mut get_tivo_drive(self.drive_location.clone()) {
                Ok(tivo_drive) => match tivo_drive.get_inode_from_fsid(fsid) {
                    Ok(inode) => {
                        if inode.r#type == MFSINodeType::Db {
                            println!("read({:#?}): I'm a database item!", path);
                        }
                        match inode.get_data(self.drive_location.clone()) {
                            Ok(data) => result(Ok(&data)),
                            Err(_err) => result(Err(0)),
                        }
                    }
                    Err(_err) => result(Err(0)),
                },
                Err(_err) => result(Err(0)),
            },
            Err(_err) => result(Err(0)),
        };
    }
}
