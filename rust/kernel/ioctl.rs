// SPDX-License-Identifier: GPL-2.0

//! `ioctl()` number definitions.
//!
//! C header: [`include/asm-generic/ioctl.h`](srctree/include/asm-generic/ioctl.h)
<<<<<<< HEAD

#![expect(non_snake_case)]
use crate::build_assert;

/// Build an ioctl number, analogous to the C macro of the same name.
#[inline(always)]
const fn _IOC(dir: u32, ty: u32, nr: u32, size: usize) -> u32 {
    build_assert!(dir <= uapi::_IOC_DIRMASK, "Invalid direction value");
    build_assert!(ty <= uapi::_IOC_TYPEMASK, "Invalid type value");
    build_assert!(nr <= uapi::_IOC_NRMASK, "Invalid number value");
    build_assert!(size <= (uapi::_IOC_SIZEMASK as usize), "Invalid size value");

    (dir << uapi::_IOC_DIRSHIFT)
        | (ty << uapi::_IOC_TYPESHIFT)
        | (nr << uapi::_IOC_NRSHIFT)
        | ((size as u32) << uapi::_IOC_SIZESHIFT)
}

/// Build an ioctl number for an argumentless ioctl.
#[inline(always)]
pub const fn _IO(ty: u32, nr: u32) -> u32 {
    _IOC(uapi::_IOC_NONE, ty, nr, 0)
}

/// Build an ioctl number for a read-only ioctl.
#[inline(always)]
pub const fn _IOR<T>(ty: u32, nr: u32) -> u32 {
    _IOC(uapi::_IOC_READ, ty, nr, core::mem::size_of::<T>())
}

/// Build an ioctl number for a write-only ioctl.
#[inline(always)]
pub const fn _IOW<T>(ty: u32, nr: u32) -> u32 {
    _IOC(uapi::_IOC_WRITE, ty, nr, core::mem::size_of::<T>())
}

/// Build an ioctl number for a read-write ioctl.
#[inline(always)]
pub const fn _IOWR<T>(ty: u32, nr: u32) -> u32 {
    _IOC(
        uapi::_IOC_READ | uapi::_IOC_WRITE,
        ty,
        nr,
        core::mem::size_of::<T>(),
    )
}

/// Get the ioctl direction from an ioctl number.
pub const fn _IOC_DIR(nr: u32) -> u32 {
    (nr >> uapi::_IOC_DIRSHIFT) & uapi::_IOC_DIRMASK
}

/// Get the ioctl type from an ioctl number.
pub const fn _IOC_TYPE(nr: u32) -> u32 {
    (nr >> uapi::_IOC_TYPESHIFT) & uapi::_IOC_TYPEMASK
}

/// Get the ioctl number from an ioctl number.
pub const fn _IOC_NR(nr: u32) -> u32 {
    (nr >> uapi::_IOC_NRSHIFT) & uapi::_IOC_NRMASK
}

/// Get the ioctl size from an ioctl number.
pub const fn _IOC_SIZE(nr: u32) -> usize {
    ((nr >> uapi::_IOC_SIZESHIFT) & uapi::_IOC_SIZEMASK) as usize
}

// Example usage
const MY_DEVICE_TYPE: u32 = 0x1234;
const MY_IOCTL_NUMBER: u32 = 0x01;

// Creating IOCTL numbers
const IOCTL_MY_DEVICE_READ: u32 = _IOR<MyDataType>(MY_DEVICE_TYPE, MY_IOCTL_NUMBER);
const IOCTL_MY_DEVICE_WRITE: u32 = _IOW<MyDataType>(MY_DEVICE_TYPE, MY_IOCTL_NUMBER);
