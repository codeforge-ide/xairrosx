use super::vfs::{self, FsError, InodeKind, Metadata, Result};
use alloc::sync::Arc;
use alloc::vec::Vec;
use spin::Mutex;
use alloc::string::{String, ToString};
use alloc::collections::BTreeMap;

const MAX_FILES: usize = 1024;
const MAX_FILE_SIZE: usize = 4096;
const RAMDISK_SIZE: usize = MAX_FILES * MAX_FILE_SIZE;

static RAMDISK_DATA: Mutex<[u8; RAMDISK_SIZE]> = Mutex::new([0; RAMDISK_SIZE]);
static INODES: Mutex<Vec<RamdiskInode>> = Mutex::new(Vec::new());

#[derive(Clone)]
struct RamdiskInode {
    id: usize,
    kind: InodeKind,
    data: Arc<Mutex<Vec<u8>>>,
    children: Arc<Mutex<BTreeMap<String, usize>>>,
}

impl RamdiskInode {
    fn new(id: usize, kind: InodeKind) -> Self {
        Self {
            id,
            kind,
            data: Arc::new(Mutex::new(Vec::new())),
            children: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }
}

impl vfs::Inode for RamdiskInode {
    fn open(&self) -> Result<Arc<dyn vfs::File>> {
        Ok(Arc::new(RamdiskFile::new(self.clone())))
    }

    fn metadata(&self) -> Result<Metadata> {
        Ok(Metadata {
            kind: self.kind,
            size: self.data.lock().len() as u64,
        })
    }

    fn kind(&self) -> InodeKind {
        self.kind
    }

    fn readdir(&self) -> Result<Vec<vfs::DirEntry>> {
        if self.kind != InodeKind::Directory {
            return Err(FsError::NotADirectory);
        }
        let children = self.children.lock();
        let inodes = INODES.lock();
        let mut entries = Vec::new();
        for (name, &inode_id) in children.iter() {
            entries.push(vfs::DirEntry {
                name: name.clone(),
                inode: Arc::new(inodes[inode_id].clone()),
            });
        }
        Ok(entries)
    }

    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
}

struct RamdiskFile {
    inode: RamdiskInode,
    position: Mutex<u64>,
}

impl RamdiskFile {
    fn new(inode: RamdiskInode) -> Self {
        Self {
            inode,
            position: Mutex::new(0),
        }
    }
}

impl vfs::File for RamdiskFile {
    fn read(&self, buf: &mut [u8]) -> Result<usize> {
        let mut pos = self.position.lock();
        let data = self.inode.data.lock();
        let len = core::cmp::min(buf.len(), data.len() - *pos as usize);
        buf[..len].copy_from_slice(&data[*pos as usize..*pos as usize + len]);
        *pos += len as u64;
        Ok(len)
    }

    fn write(&self, buf: &[u8]) -> Result<usize> {
        let mut pos = self.position.lock();
        let mut data = self.inode.data.lock();
        let len = buf.len();
        if *pos as usize + len > data.len() {
            data.resize(*pos as usize + len, 0);
        }
        data[*pos as usize..*pos as usize + len].copy_from_slice(buf);
        *pos += len as u64;
        Ok(len)
    }

    fn seek(&self, pos: vfs::SeekFrom) -> Result<u64> {
        let mut current_pos = self.position.lock();
        let new_pos = match pos {
            vfs::SeekFrom::Start(offset) => offset,
            vfs::SeekFrom::End(offset) => (self.inode.data.lock().len() as i64 + offset) as u64,
            vfs::SeekFrom::Current(offset) => (*current_pos as i64 + offset) as u64,
        };
        *current_pos = new_pos;
        Ok(new_pos)
    }

    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
}

pub struct Ramdisk;

impl Ramdisk {
    pub fn new() -> Self {
        let mut inodes = INODES.lock();
        if inodes.is_empty() {
            inodes.push(RamdiskInode::new(0, InodeKind::Directory));
        }
        Ramdisk
    }
}

impl vfs::FileSystem for Ramdisk {
    fn root(&self) -> Result<Arc<dyn vfs::Inode>> {
        Ok(Arc::new(INODES.lock()[0].clone()))
    }

    fn name(&self) -> &str {
        "ramdisk"
    }
}
