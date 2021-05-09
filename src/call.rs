use super::*;

/// Get the current system time
pub fn clock_gettime(clock: usize, ts: &mut TimeSpec) -> Result<usize> {
    unsafe { syscall2(SYS_CLOCK_GETTIME, clock, ts as *mut TimeSpec as usize) }
}