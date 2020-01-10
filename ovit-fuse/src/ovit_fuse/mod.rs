extern crate ovit;

use fuse_mt::{
    FileAttr, FileType, FilesystemMT, RequestInfo, ResultEmpty, ResultEntry, ResultOpen,
    ResultReaddir,
};
use ovit::TivoDrive;
use std::convert::TryInto;
use std::path::Path;
use time::Timespec;

pub struct TiVoFS {
    tivo_drive: TivoDrive,
}

impl TiVoFS {
    pub fn from_drive_location(location: &str) -> Result<TiVoFS, String> {
        let tivo_drive = TivoDrive::from_disk_image(location)?;

        Ok(TiVoFS { tivo_drive })
    }
}

const TTL: Timespec = Timespec { sec: 1, nsec: 0 };

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

        Ok((
            TTL,
            FileAttr {
                size: 1,
                blocks: 1,
                atime: TTL,
                mtime: TTL,
                ctime: TTL,
                crtime: TTL,
                kind: FileType::Directory,
                perm: 777,
                nlink: 0,
                uid: 1000,
                gid: 1000,
                rdev: 0,
                flags: 0,
            },
        ))
    }

    fn opendir(&self, _req: RequestInfo, path: &Path, _flags: u32) -> ResultOpen {
        println!("opendir path: {:#?}", path);
        if path == Path::new("/") {
            Ok((1, 0))
        } else {
            Err(0)
        }
    }

    fn readdir(&self, _req: RequestInfo, path: &Path, fsid: u64) -> ResultReaddir {
        println!("readdir path: {:#?}", path);

        // match self
        //     .tivo_drive
        //     .get_inode_from_fsid(fsid.try_into().unwrap())
        // {
        //     Ok(inode) => Ok(vec![]),
        //     Err(_err) => Err(0),
        // }

        Ok(vec![])
    }
}
