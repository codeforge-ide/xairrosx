use crate::println;
use self::ramdisk::Ramdisk;
use self::vfs::FileSystem;
use alloc::sync::Arc;
use lazy_static::lazy_static;

pub mod ramdisk;
pub mod vfs;

lazy_static! {
    pub static ref ROOT_FS: Arc<dyn FileSystem> = Arc::new(Ramdisk::new());
}

pub fn init() {
    // The lazy_static macro will handle the initialization of the root filesystem.
    // We can access it through the ROOT_FS static.
    println!("Initialized ramdisk filesystem: {}", ROOT_FS.name());
}
