use glium::{uniform, Surface};

use crate::{draw::Draw, mesh::Vertex};

pub struct Image {
    pub image: image::ImageBuffer<image::Rgba<f32>, Vec<f32>>,
    pub open_domain: bool,
}

pub struct ImagePlane {
    pub texture: glium::Texture2d,
    pub open_domain: bool,
    pub program: glium::Program,
    pub plane_vertex_buffer: glium::VertexBuffer<Vertex>,
    pub plane_index_buffer: glium::IndexBuffer<u32>,
}

impl ImagePlane {
    pub fn new<
        T: glutin::surface::SurfaceTypeTrait + glutin::surface::ResizeableSurface + 'static,
    >(
        image: Image,
        display: glium::Display<T>,
    ) -> Self {
        let open_domain = image.open_domain;
        let img = image.image;

        let img_dimensions = img.dimensions();
        let img =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&img.into_raw(), img_dimensions);

        let texture = glium::texture::Texture2d::with_format(
            &display,
            img,
            glium::texture::UncompressedFloatFormat::F32F32F32F32,
            glium::texture::MipmapsOption::NoMipmap,
        )
        .unwrap();

        let plane_vertices = vec![
            Vertex {
                position: [-1., -1.],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [-1., 1.],
                tex_coords: [0.0, 0.5],
            },
            Vertex {
                position: [1., -1.],
                tex_coords: [0.5, 0.0],
            },
            Vertex {
                position: [1., 1.],
                tex_coords: [0.5, 0.5],
            },
        ];

        let plane_vertex_buffer = glium::VertexBuffer::new(&display, &plane_vertices).unwrap();
        let plane_index_buffer = glium::index::IndexBuffer::new(
            &display,
            glium::index::PrimitiveType::TrianglesList,
            &[0u32, 1u32, 3u32, 3u32, 2u32, 0u32],
        )
        .unwrap();

        let program = glium::Program::new(
            &display,
            glium::program::ProgramCreationInput::SourceCode {
                vertex_shader: include_str!("vertex.glsl"),
                tessellation_control_shader: None,
                tessellation_evaluation_shader: None,
                geometry_shader: None,
                fragment_shader: include_str!("fragment.glsl"),
                transform_feedback_varyings: None,
                outputs_srgb: true,
                uses_point_size: false,
            },
        )
        .unwrap();

        Self {
            texture,
            open_domain,
            program,
            plane_vertex_buffer,
            plane_index_buffer,
        }
    }
}

impl Draw for ImagePlane {
    fn draw(&self, target: &mut glium::Frame, dimensions: [f32; 2]) {
        target
                    .draw(
                        &self.plane_vertex_buffer,
                        &self.plane_index_buffer,
                        &self.program,
                        &uniform! {
                                    scale: [1.0_f32, 1.0_f32],
                                    rot: 0.0_f32,
                                    loc: [0.0_f32, 0.0_f32],
                                    dimensions: dimensions,
                                    tex: self.texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear),
                                    is_exr: self.open_domain,
                                    intensity: 1.0_f32,
                                    compression_factor: 1.0_f32,
                    },
                &Default::default(),
            )
            .unwrap();
    }
}
