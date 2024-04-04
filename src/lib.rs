#![no_std]
extern crate alloc;
extern crate libm;

mod offset_estimator;

pub use offset_estimator::estimate;
