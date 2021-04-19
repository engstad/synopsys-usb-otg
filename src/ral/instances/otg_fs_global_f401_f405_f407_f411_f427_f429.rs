#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB on the go full speed
//!
//! Used by: stm32f401, stm32f405, stm32f407, stm32f411, stm32f427, stm32f429

#[cfg(not(feature = "nosync"))]
pub use super::super::peripherals::otg_fs_global_v1::Instance;
pub use super::super::peripherals::otg_fs_global_v1::RegisterBlock;
pub use super::super::peripherals::otg_fs_global_v1::{
    CID, DIEPTXF0, DIEPTXF1, DIEPTXF2, DIEPTXF3, GAHBCFG, GCCFG, GINTMSK, GINTSTS, GNPTXSTS,
    GOTGCTL, GOTGINT, GRSTCTL, GRXFSIZ, GRXSTSP, GRXSTSR, GUSBCFG, HPTXFSIZ,
};

/// Access functions for the OTG_FS_GLOBAL peripheral instance
pub mod OTG_FS_GLOBAL {
    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50000000,
        _marker: ::core::marker::PhantomData,
    };
}

/// Raw pointer to OTG_FS_GLOBAL
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OTG_FS_GLOBAL: *const RegisterBlock = 0x50000000 as *const _;
