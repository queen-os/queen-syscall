use super::*;
use crate::flags::*;
use cstr_core::CString;

/// Get the current system time
pub fn clock_gettime(clock: usize, ts: &mut TimeSpec) -> Result<usize> {
    unsafe { syscall2(SYS_CLOCK_GETTIME, clock, ts as *mut TimeSpec as usize) }
}

/// Open a file
pub fn open(path: &str, flags: OpenFlags, mode: usize) -> Result<usize> {
    open_at(AT_FDCWD, path, flags, mode)
}

/// Open a file
pub fn open_at(dirfd: usize, path: &str, flags: OpenFlags, mode: usize) -> Result<usize> {
    let path = CString::new(path).unwrap();
    unsafe {
        syscall4(
            SYS_OPENAT,
            dirfd,
            path.as_ptr() as usize,
            flags.bits(),
            mode,
        )
    }
}

/// Close a file
pub fn close(fd: usize) -> Result<usize> {
    unsafe { syscall1(SYS_CLOSE, fd) }
}

/// Read from a file descriptor into a buffer
pub fn read(fd: usize, buf: &mut [u8]) -> Result<usize> {
    unsafe { syscall3(SYS_READ, fd, buf.as_mut_ptr() as usize, buf.len()) }
}

/// Write a buffer to a file descriptor
pub fn write(fd: usize, buf: &[u8]) -> Result<usize> {
    unsafe { syscall3(SYS_WRITE, fd, buf.as_ptr() as usize, buf.len()) }
}

/// Seek to `offset` bytes in a file descriptor
pub fn lseek(fd: usize, offset: i64, whence: u8) -> Result<usize> {
    unsafe { syscall3(SYS_LSEEK, fd, offset as usize, whence as usize) }
}

/// Read from a file descriptor into a buffer
pub fn pread(fd: usize, buf: &mut [u8], pos: usize) -> Result<usize> {
    unsafe { syscall4(SYS_PREAD64, fd, buf.as_mut_ptr() as usize, buf.len(), pos) }
}

/// Write a buffer to a file descriptor
pub fn pwrite(fd: usize, buf: &[u8], pos: usize) -> Result<usize> {
    unsafe { syscall4(SYS_PWRITE64, fd, buf.as_ptr() as usize, buf.len(), pos) }
}

/// Sync a file descriptor to its underlying medium
pub fn fsync(fd: usize) -> Result<usize> {
    unsafe { syscall1(SYS_FSYNC, fd) }
}

/// Sync a file descriptor to its underlying medium
pub fn fdatasync(fd: usize) -> Result<usize> {
    unsafe { syscall1(SYS_FDATASYNC, fd) }
}

/// Truncate or extend a file to a specified length
pub fn ftruncate(fd: usize, len: usize) -> Result<usize> {
    unsafe { syscall2(SYS_FTRUNCATE, fd, len) }
}

/// Truncate or extend a file to a specified length
pub fn truncate(path: &str, len: usize) -> Result<usize> {
    let path = CString::new(path).unwrap();
    unsafe { syscall2(SYS_TRUNCATE, path.as_ptr() as usize, len) }
}

/// Get the current working directory
pub fn getcwd(buf: &mut [u8]) -> Result<usize> {
    unsafe { syscall2(SYS_GETCWD, buf.as_mut_ptr() as usize, buf.len()) }
}

/// Change the process's working directory
pub fn chdir(path: &str) -> Result<usize> {
    let path = CString::new(path).unwrap();
    unsafe { syscall1(SYS_CHDIR, path.as_ptr() as usize) }
}

/// Yield the process's time slice to the kernel
pub fn sched_yield() -> Result<usize> {
    unsafe { syscall0(SYS_SCHED_YIELD) }
}

pub fn nanosleep(time: &TimeSpec) -> Result<usize> {
    unsafe { syscall1(SYS_NANOSLEEP, time as *const _ as usize) }
}

pub fn exit(code: usize) -> Result<usize> {
    unsafe { syscall1(SYS_EXIT, code) }
}
