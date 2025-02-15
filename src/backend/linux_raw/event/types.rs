use crate::backend::c;
use bitflags::bitflags;

bitflags! {
    /// `EFD_*` flags for use with [`eventfd`].
    ///
    /// [`eventfd`]: crate::io::eventfd
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct EventfdFlags: c::c_uint {
        /// `EFD_CLOEXEC`
        const CLOEXEC = linux_raw_sys::general::EFD_CLOEXEC;
        /// `EFD_NONBLOCK`
        const NONBLOCK = linux_raw_sys::general::EFD_NONBLOCK;
        /// `EFD_SEMAPHORE`
        const SEMAPHORE = linux_raw_sys::general::EFD_SEMAPHORE;
    }
}
