use super::super::result::PWResult;
use super::super::Thingie as Base;
use super::super::collision::Line;

use super::prelude::*;

pub trait DebugRender<S, F> where S: Base, F: Facade {
    /// program uniforms are matrix (mat4)
    /// vertex attributes are: position (vec2), color (vec4)
    fn render(&self, facade: &F, surface: &mut Frame, matrix: Mat4<S>, program: &Program, draw_parameters: &DrawParameters) -> PWResult<()>;
}

#[derive(Debug)]
pub struct DebugRenderer<'a, S> where S: Base {
    program: Program,
    draw_parameters: DrawParameters<'a>,
    matrix: Mat4<S>,
}

#[derive(Debug,Copy,Clone)]
pub struct DebugVertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

implement_vertex!(DebugVertex, position, color);

const VERT_100:&'static str = include_str!("../../shaders/100.vert");
const VERT_110:&'static str = include_str!("../../shaders/110.vert");
const VERT_140:&'static str = include_str!("../../shaders/140.vert");

const FRAG_100:&'static str = include_str!("../../shaders/100.frag");
const FRAG_110:&'static str = include_str!("../../shaders/110.frag");
const FRAG_140:&'static str = include_str!("../../shaders/140.frag");

impl<'a, S> DebugRenderer<'a, S> where S: Base {
    pub fn new(facade: &SDL2Facade) -> PWResult<DebugRenderer<'a, S>> {
        let prog = try!(program!(facade,
            140 => {
                vertex: VERT_140,
                fragment: FRAG_140
            },
            110 => {
                vertex: VERT_110,
                fragment: FRAG_110
            },
            100 => {
                vertex: VERT_100,
                fragment: FRAG_100
            }
        ));
        Ok(DebugRenderer {
            program: prog,
            draw_parameters: DrawParameters {
                blend: Blend {
                    color: BlendingFunction::AlwaysReplace,
                    alpha: BlendingFunction::Max,
                    ..Default::default()
                },
                depth: Depth {
                    test: DepthTest::Overwrite,
                    write: false,
                    range: (0.0, 1.0),
                    clamp: DepthClamp::NoClamp,
                },
                point_size: Some(3.0),
                ..Default::default()
            },
            matrix: Mat4::<S>::new_identity(4),
        })
    }

    pub fn set_matrix(&mut self, matrix: Mat4<S>) {
        self.matrix = matrix;
    }

    pub fn draw<F:Facade + Sized>(&self, display: &F, frame: &mut Frame, obj: &DebugRender<S, F>) -> PWResult<()> {
        obj.render(display, frame, self.matrix, &self.program, &self.draw_parameters)
    }
}

impl<F> DebugRender<f32,F> for Line<f32> where F: Facade {
    fn render(&self, facade: &F, surface: &mut Frame, matrix: Mat4<f32>, program: &Program, draw_parameters: &DrawParameters) -> PWResult<()> {
        let verts = try!(VertexBuffer::new(facade, &[
            DebugVertex { position: self.a.as_ref().clone(), color:  [1.0, 0.0, 0.0, 1.0]},
            DebugVertex { position: self.b.as_ref().clone(), color: [1.0, 0.0, 0.0, 1.0]}
        ]));

        let indices = try!(IndexBuffer::new(facade, PrimitiveType::LinesList, &[0u32, 1u32]));
        let uniforms = uniform! {
            matrix: matrix.as_ref().clone()
        };
        try!(surface.draw(&verts, &indices, program, &uniforms, draw_parameters));
        Ok(())
    }
}
