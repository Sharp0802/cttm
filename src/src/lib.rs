#![feature(iter_advance_by)]

#[cfg(feature = "build")]
mod build;

#[cfg(feature = "build")]
pub use build::*;

#[macro_export]
macro_rules! import {
    () => {
        include!(concat!(env!("OUT_DIR"), "/cttm.rs"));
    };
}
