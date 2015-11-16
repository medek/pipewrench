#[macro_use]
extern crate glium;
extern crate glium_sdl2;
extern crate sdl2;
extern crate toml;
extern crate nalgebra;
extern crate num;
#[macro_use]
mod macros;
mod result;
mod window;
mod input;
mod storage;
pub mod config;
pub mod collision;

pub use result::*;
pub use window::*;
pub use input::*;
pub use storage::*;

pub trait Thingie: //TODO: Rename this to something not stupid
    num::Float
    + std::fmt::Display
    + PartialOrd
    + std::ops::Add<Self, Output=Self>
    + std::ops::Sub<Self, Output=Self>
    + std::ops::Mul<Self, Output=Self>
    + std::ops::Div<Self, Output=Self>
    + num::NumCast
    + Copy {}

impl Thingie for f32 {}
impl Thingie for f64 {}
