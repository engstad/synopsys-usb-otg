[![crates.io](https://img.shields.io/crates/d/synopsys-usb-otg.svg)](https://crates.io/crates/synopsys-usb-otg)
[![crates.io](https://img.shields.io/crates/v/synopsys-usb-otg.svg)](https://crates.io/crates/synopsys-usb-otg)
[![Build Status](https://travis-ci.com/stm32-rs/synopsys-usb-otg.svg?branch=master)](https://travis-ci.com/stm32-rs/synopsys-usb-otg)

# `synopsys-usb-otg`

> [usb-device](https://github.com/mvirkkunen/usb-device) implementation for Synopsys USB OTG IP cores.

This project is a successor to the [great work](https://github.com/mvirkkunen/stm32f103xx-usb)
started by [@mvirkkunen](https://github.com/mvirkkunen).

## Supported microcontrollers

* `STM32F429xx` (OTG_FS and OTG_HS in FS mode)
* `STM32F401xx`
* `STM32F446xx` (OTG_FS and OTG_HS in FS mode)
* `STM32F723xx` (OTG_FS and OTG_HS with internal HS PHY)
* `STM32H7xxxx` (OTG1_HS and OTG2_HS in FS mode, and OTG1_HS with external HS PHY)
* And others...


## Usage

This driver is intended for use through a device hal library.
Such hal library should implement `UsbPeripheral` for the corresponding USB peripheral object.
This trait declares all the peripheral properties that may vary from one device family to the other.
Additionally, hal should pass `fs` of `hs` feature to the `synopsys-usb-otg` library to
define a peripheral type:
* `fs` - for FullSpeed peripherals
* `hs` - for HighSpeed peripherals

Only one peripheral type can be selected at the moment.

## Examples

See the [usb-otg-workspace](https://github.com/Disasm/usb-otg-workspace) repo for different device-specific examples.
