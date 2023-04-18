use fs2::FileExt;
use std::fs::File;
use std::ops::Drop;

pub struct LockGuard<'a> {
    file: &'a File,
}

impl<'a> LockGuard<'a> {
    pub fn new(file: &'a File) -> std::io::Result<Self> {
        file.lock_exclusive()?;
        Ok(LockGuard { file })
    }
}

impl<'a> Drop for LockGuard<'a> {
    fn drop(&mut self) {
        let _ = self.file.unlock();
    }
}