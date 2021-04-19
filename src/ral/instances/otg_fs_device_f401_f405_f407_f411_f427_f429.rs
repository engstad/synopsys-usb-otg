#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB on the go full speed
//!
//! Used by: stm32f401, stm32f405, stm32f407, stm32f411, stm32f427, stm32f429

#[cfg(not(feature = "nosync"))]
pub use super::super::peripherals::otg_fs_device_v1::Instance;
pub use super::super::peripherals::otg_fs_device_v1::RegisterBlock;
pub use super::super::peripherals::otg_fs_device_v1::{
    DAINT, DAINTMSK, DCFG, DCTL, DIEPCTL0, DIEPCTL1, DIEPCTL2, DIEPCTL3, DIEPEMPMSK, DIEPINT0,
    DIEPINT1, DIEPINT2, DIEPINT3, DIEPMSK, DIEPTSIZ0, DIEPTSIZ1, DIEPTSIZ2, DIEPTSIZ3, DOEPCTL0,
    DOEPCTL1, DOEPCTL2, DOEPCTL3, DOEPINT0, DOEPINT1, DOEPINT2, DOEPINT3, DOEPMSK, DOEPTSIZ0,
    DOEPTSIZ1, DOEPTSIZ2, DOEPTSIZ3, DSTS, DTXFSTS0, DTXFSTS1, DTXFSTS2, DTXFSTS3, DVBUSDIS,
    DVBUSPULSE,
};

/// Access functions for the OTG_FS_DEVICE peripheral instance
pub mod OTG_FS_DEVICE {
    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50000800,
        _marker: ::core::marker::PhantomData,
    };
}

/// Raw pointer to OTG_FS_DEVICE
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OTG_FS_DEVICE: *const RegisterBlock = 0x50000800 as *const _;
