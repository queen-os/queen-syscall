use bitflags::bitflags;

bitflags! {
    pub struct OpenFlags: usize {
        /// read only
        const RDONLY = 0;
        /// write only
        const WRONLY = 1;
        /// read write
        const RDWR = 2;
        /// create file if it does not exist
        const CREATE = 1 << 6;
        /// error if CREATE and the file exists
        const EXCLUSIVE = 1 << 7;
        /// truncate file upon open
        const TRUNCATE = 1 << 9;
        /// append on each write
        const APPEND = 1 << 10;
        /// close on exec
        const CLOEXEC = 1 << 19;
    }
}

impl OpenFlags {
    #[inline]
    pub fn readable(&self) -> bool {
        let b = self.bits() & 0b11;
        b == OpenFlags::RDONLY.bits() || b == OpenFlags::RDWR.bits()
    }

    #[inline]
    pub fn writable(&self) -> bool {
        let b = self.bits() & 0b11;
        b == OpenFlags::WRONLY.bits() || b == OpenFlags::RDWR.bits()
    }

    #[inline]
    pub fn is_append(&self) -> bool {
        self.contains(OpenFlags::APPEND)
    }
}

bitflags! {
    pub struct AtFlags: usize {
        const EMPTY_PATH = 0x1000;
        const SYMLINK_NOFOLLOW = 0x100;
    }
}

bitflags! {
    pub struct CloneFlags: usize {
        const CSIGNAL =         0x000000ff;
        const VM =              0x00000100;
        const FS =              0x00000200;
        const FILES =           0x00000400;
        const SIGHAND =         0x00000800;
        const PTRACE =          0x00002000;
        const VFORK =           0x00004000;
        const PARENT =          0x00008000;
        const THREAD =          0x00010000;
        const NEWNS	 =          0x00020000;
        const SYSVSEM =         0x00040000;
        const SETTLS =          0x00080000;
        const PARENT_SETTID =   0x00100000;
        const CHILD_CLEARTID =  0x00200000;
        const DETACHED =        0x00400000;
        const UNTRACED =        0x00800000;
        const CHILD_SETTID =    0x01000000;
        const NEWCGROUP =       0x02000000;
        const NEWUTS =          0x04000000;
        const NEWIPC =          0x08000000;
        const NEWUSER =         0x10000000;
        const NEWPID =          0x20000000;
        const NEWNET =          0x40000000;
        const IO =              0x80000000;
    }
}

/// Pathname is interpreted relative to the current working directory(CWD)
pub const AT_FDCWD: usize = -100isize as usize;

