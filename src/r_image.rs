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
    fn draw(&self, target: &mut glium::Frame, matrix: &nalgebra::Matrix2<f32>) {
        let dimensions = target.get_dimensions();

        let aspect_target = dimensions.0 as f32 / dimensions.1 as f32;
        let aspect_matrix = matrix.m11 / matrix.m22;
        let aspect_draw_area = aspect_target * aspect_matrix;
        let aspect_image = self.texture.width() as f32 / self.texture.height() as f32;

        // This matrix ensures that the aspect ratio of the image is correct and keeps the size within a (-1, 1) boundary.
        let m = if aspect_image >= aspect_draw_area {
            nalgebra::Matrix2::new(1.0, 0.0, 0.0, aspect_draw_area / aspect_image)
        } else {
            nalgebra::Matrix2::new(aspect_image / aspect_draw_area, 0.0, 0.0, 1.0)
        };

        // Move the image to the requested place on the target.
        let matrix = m * matrix;

        target
            .draw(
                &self.plane_vertex_buffer,
                &self.plane_index_buffer,
                &self.program,
                &uniform! {
                    mat: *matrix.as_ref(),
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
