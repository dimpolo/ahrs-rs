//! A collection of AHRS algorithms ported to Rust.

#![cfg_attr(not(feature = "std"), no_std)]
#![crate_name = "ahrs"]

pub use crate::{
    ahrs::{Ahrs, Error},
    madgwick::Madgwick,
    mahony::Mahony,
};

mod ahrs;
mod madgwick;
mod mahony;
