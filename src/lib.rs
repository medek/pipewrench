#[cfg(feature = "render")]
#[macro_use]
extern crate glium;
#[cfg(feature = "render")]
extern crate glium_sdl2;
extern crate sdl2;
extern crate toml;
extern crate nalgebra;
extern crate num;
extern crate image;
#[macro_use]
mod macros;
mod result;
mod window;
mod input;
mod storage;
pub mod config;
#[cfg(feature = "collision")]
pub mod collision;
#[cfg(feature = "render")]
pub mod render;

pub use result::*;
pub use window::*;
pub use input::*;
pub use storage::*;

pub trait Thingie: //TODO: Rename this to something not stupid
    nalgebra::BaseNum
    + num::Float
    + std::fmt::Display
    + std::fmt::Debug
    + PartialOrd
    + PartialEq
    + std::ops::Add<Self, Output=Self>
    + std::ops::Sub<Self, Output=Self>
    + std::ops::Mul<Self, Output=Self>
    + std::ops::Div<Self, Output=Self>
    + num::NumCast
    + Copy {}

impl Thingie for f32 {}
impl Thingie for f64 {}
