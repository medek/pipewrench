mod debug_render;
mod texture;
mod uniforms;

pub use self::debug_render::*;
pub use self::texture::*;
pub use self::uniforms::*;

pub mod prelude {
    pub use nalgebra::{Mat4, Eye};
    pub use glium_sdl2::SDL2Facade;
    pub use glium::backend::Facade;
    pub use glium::{Program, Frame, Surface};
    pub use glium::index::{IndexBuffer, IndicesSource, NoIndices, PrimitiveType};
    pub use glium::vertex::VertexBuffer;
    pub use glium::draw_parameters::{DrawParameters, DepthClamp, Depth, DepthTest,
                                 Blend, BlendingFunction,
                                 PolygonMode};
    pub use super::debug_render::{DebugRender, DebugVertex, DebugRenderer};
}
