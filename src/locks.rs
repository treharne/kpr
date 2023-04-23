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

// tests for locks
#[cfg(test)]
mod tests {
    use crate::store::{full_path, open_or_create};

    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_lock_guard_locks_file() {
        let filepath = full_path("test_lock_guard_locks_file.txt");
        let _ = std::fs::remove_file(&filepath);
    
        // Open a file and lock it.
        let file = open_or_create(&filepath, false).expect("Could not open store file");
        let lock_guard = LockGuard::new(&file).expect("Could not lock file");
    
        // Try to lock the file again in a separate thread.
        let filepath = filepath.clone();
        let handle = thread::spawn(move || {
            let file = open_or_create(&filepath, false).expect("Could not open store file");
            match file.try_lock_exclusive() {
                Ok(_) => false, // Locking the file succeeded, which is not expected
                Err(_) => true, // Locking the file failed, which is expected
            }
        });
    
        // The other thread should not be able to lock the file.
        let lock_result = handle.join().expect("Thread panicked");
        assert!(lock_result, "The other thread was able to lock the file");
    
        // Drop the lock guard and allow the other thread to open the file.
        drop(lock_guard);
    }

    #[test]
    fn test_lock_guard_unlocks_file() {
        let filepath = full_path("test_lock_guard_unlocks_file.txt");
        let _ = std::fs::remove_file(&filepath);
    
        // Open a file and lock it.
        let file = open_or_create(&filepath, false).expect("Could not open store file");
        let lock_guard = LockGuard::new(&file).expect("Could not lock file");
    
        // Drop the lock guard and allow the other thread to open the file.
        drop(lock_guard);
    
        // Try to open the file again in a separate thread.
        let filepath = filepath.clone();
        let handle = thread::spawn(move || {
            let file = open_or_create(&filepath, false).expect("Could not open store file");
            file
        });
    
        // The other thread should be able to open the file now.
        assert!(handle.join().is_ok());
    }
}
