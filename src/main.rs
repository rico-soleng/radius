use glium::{implement_vertex, uniform, Surface, Texture2d};
use std::env;
use std::fs::File;
use std::io::Read;
use std::sync::mpsc;
use std::thread;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build();
    let (window, display) = glium::backend::glutin::simple_winit_window(&event_loop, "Radius");

    // Stop virtual keyboards from opening.
    window.set_ime_allowed(false);

    let mut frame = display.draw();
    frame.clear_color(0.0, 0.0, 0.0, 1.0);
    frame.finish().unwrap();

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

    let proxy = event_loop.create_proxy();

    let args: Vec<_> = env::args().collect();
    let path = args.get(1).expect("ERROR: No image given!").to_owned();

    let (tx, rx) = mpsc::channel();

    let _img_load_handle = thread::spawn(move || {
        let file = File::open(&path).expect("File not found!");
        let buf: Vec<u8> = file.bytes().map(|b| b.unwrap()).collect();
        let format = image::guess_format(&buf).expect("Unsupported format!");
        let image =
            image::load_from_memory_with_format(&buf, format).expect("Failed to decode image!");

        if format == image::ImageFormat::OpenExr {
            tx.send((image.to_rgba32f(), true)).unwrap();
        } else {
            tx.send((image.to_rgba32f(), false)).unwrap();
        }
        proxy.send_event(()).unwrap();
    });

    let mut is_exr = false;
    let mut texture: Option<Texture2d> = None;

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            winit::event::Event::RedrawRequested(_window_id) => {
                let dimensions = window.inner_size();
                let dimensions = [dimensions.width as f32, dimensions.height as f32];
                
                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);
                if let Some(tex) = &texture {
                    target
                    .draw(
                        &plane_vertex_buffer,
                        &plane_index_buffer,
                        &program,
                        &uniform! { 
                                    scale: [1.0_f32, 1.0_f32],
                                    rot: 0.0 as f32, 
                                    loc: [0.0_f32, 0.0_f32],
                                    dimensions: dimensions, 
                                    tex: tex.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                                    is_exr: is_exr,
                                    intensity: 1.0_f32,
                                    compression_factor: 1.0_f32,
                    },
                &Default::default(),
            )
            .unwrap();
                }
                
                target.finish().unwrap();
            }
            winit::event::Event::MainEventsCleared => {
                window.request_redraw();
            }
            winit::event::Event::UserEvent(()) => {
                if let Ok(img) = rx.try_recv() {
                    is_exr = img.1;
                    let img = img.0;

                    let img_dimensions = img.dimensions();
                    let img = glium::texture::RawImage2d::from_raw_rgba_reversed(
                        &img.into_raw(),
                        img_dimensions,
                    );

                    texture = Some(
                        glium::texture::Texture2d::with_format(
                            &display,
                            img,
                            glium::texture::UncompressedFloatFormat::F32F32F32F32,
                            glium::texture::MipmapsOption::NoMipmap,
                        )
                        .unwrap(),
                    );
                }
            }
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => control_flow.set_exit(),
                _ => (),
            },
            _ => (),
        };
        
    });
}
