use glium::Surface;
use std::env;
use std::sync::mpsc;
use std::thread;

mod draw;
use draw::Draw;

mod r_image;
use r_image::ImagePlane;

mod imageloader;
mod mesh;

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Radius")
        .build(&event_loop);

    // Stop virtual keyboards from opening.
    window.set_ime_allowed(false);

    let mut frame = display.draw();
    frame.clear_color(0.0, 0.0, 0.0, 1.0);
    frame.finish().unwrap();

    let proxy = event_loop.create_proxy();

    let args: Vec<_> = env::args().collect();
    let path = args.get(1).expect("ERROR: No image given!").to_owned();

    let (tx, rx) = mpsc::channel();

    let _img_load_handle = thread::spawn(move || {
        let img = imageloader::open_file(path);
        tx.send(img).unwrap();
        proxy.send_event(()).unwrap();
    });

    let mut image_plane: Option<ImagePlane> = None;

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            winit::event::Event::RedrawRequested(_window_id) => {
                let dimensions = window.inner_size();
                let dimensions = [dimensions.width as f32, dimensions.height as f32];
                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);

                let matrix = nalgebra::Matrix2::identity();

                if let Some(image_pl) = &image_plane {
                    image_pl.draw(&mut target, dimensions, &matrix);
                }

                target.finish().unwrap();
            }
            winit::event::Event::MainEventsCleared => {
                window.request_redraw();
            }
            winit::event::Event::UserEvent(()) => {
                if let Ok(Ok(img)) = rx.try_recv() {
                    image_plane = Some(ImagePlane::new(img, display.clone()))
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
