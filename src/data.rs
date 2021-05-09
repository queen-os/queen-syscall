#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct TimeSpec {
    pub secs: i64,
    pub nanos: i32,
}

impl TimeSpec {
    #[inline]
    pub const fn new(secs: i64, nanos: i32) -> Self {
        Self { secs, nanos }
    }

    #[inline]
    pub const fn zero() -> Self {
        Self::new(0, 0)
    }

    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.secs == 0 && self.nanos == 0
    }
}

impl From<core::time::Duration> for TimeSpec {
    fn from(duration: core::time::Duration) -> Self {
        Self {
            secs: duration.as_secs() as i64,
            nanos: duration.subsec_nanos() as i32,
        }
    }
}

impl From<TimeSpec> for core::time::Duration {
    fn from(ts: TimeSpec) -> Self {
        core::time::Duration::new(ts.secs as u64, ts.nanos as u32)
    }
}