
#[macro_use]
mod macros;

pub mod dummy;
pub mod unit;
pub mod constant;

pub mod impl_slice;

pub mod gen_reshape;
pub mod gen_slice;

pub mod iter;
pub mod utils;
pub mod traits;

pub mod fold;

#[cfg(test)]
mod tests;
