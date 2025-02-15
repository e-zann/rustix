//! linux_raw syscalls for PIDs
//!
//! # Safety
//!
//! See the `rustix::backend` module documentation for details.
#![allow(unsafe_code)]
#![allow(clippy::undocumented_unsafe_blocks)]

use crate::backend::conv::ret_usize_infallible;
use crate::pid::{Pid, RawNonZeroPid, RawPid};

#[inline]
pub(crate) fn getpid() -> Pid {
    unsafe {
        let pid = ret_usize_infallible(syscall_readonly!(__NR_getpid)) as RawPid;
        debug_assert!(pid > 0);
        Pid::from_raw_nonzero(RawNonZeroPid::new_unchecked(pid as _))
    }
}
