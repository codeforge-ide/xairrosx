use alloc::sync::Arc;
use alloc::vec::Vec;
use alloc::string::String;

#[derive(Debug)]
pub enum FsError {
    NotFound,
    NotADirectory,
    NotAFile,
    PermissionDenied,
    AlreadyExists,
    IoError,
}

pub type Result<T> = core::result::Result<T, FsError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InodeKind {
    File,
    Directory,
}

#[derive(Debug, Clone)]
pub struct Metadata {
    pub kind: InodeKind,
    pub size: u64,
    // Add more metadata here later, like permissions, timestamps, etc.
}

pub struct DirEntry {
    pub name: String,
    pub inode: Arc<dyn Inode>,
}

pub trait FileSystem: Send + Sync {
    fn root(&self) -> Result<Arc<dyn Inode>>;
    fn name(&self) -> &str;
}

pub trait Inode: Send + Sync {
    fn open(&self) -> Result<Arc<dyn File>>;
    fn metadata(&self) -> Result<Metadata>;
    fn kind(&self) -> InodeKind;
    fn readdir(&self) -> Result<Vec<DirEntry>>;
    fn as_any(&self) -> &dyn core::any::Any;
}

pub enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
}

pub trait File: Send + Sync {
    fn read(&self, buf: &mut [u8]) -> Result<usize>;
    fn write(&self, buf: &[u8]) -> Result<usize>;
    fn seek(&self, pos: SeekFrom) -> Result<u64>;
    fn as_any(&self) -> &dyn core::any::Any;
}
