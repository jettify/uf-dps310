//!
//! DPS3xx embedded-hal I2C driver crate
//!
//! A platform agnostic driver to interface with the dps3xx barometric pressure & temp sensor.
//! This driver uses I2C via [embedded-hal]. Note that the dps3xx also supports SPI, however that
//! is not (yet) implemented in this driver.
//!
//! [embedded-hal]: https://docs.rs/embedded-hal

#![no_std]

mod bus;
mod calibration;
mod config;
mod device;
mod device_internal;
mod register;

pub use config::{
    Config, PressureRate, PressureResolution, TemperatureRate, TemperatureResolution,
};
pub use device::{
    calc_busy_time_ms, calc_busy_time_units, calc_total_wait_ms, Calibrated, Configured, DPS3xx,
    Error, InitInProgress, InitPoll, IsConfigured, MeasurementMode, Status, Unconfigured,
    BUSYTIME_FAILSAFE_MS, BUSYTIME_SCALING, MAX_BUSYTIME_UNITS,
};
pub use register::Register;
