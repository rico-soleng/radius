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
    let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
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
    let (tx, rx) = mpsc::channel();
    
    if let Some(path) = args.get(1) {
        let path = path.to_owned();
        let filename = path.split("/").last();
        if let Some(filename) = filename {
            window.set_title(format!("Radius: {}", filename).as_str());
        }

        let _img_load_handle = thread::spawn(move || {
            let img = imageloader::open_file(path);
            tx.send(img).unwrap();
            proxy.send_event(()).unwrap();
        });
    } else {
        window.set_title("Radius | ERROR: No image given!");
    }

    let mut image_plane: Option<ImagePlane> = None;

    event_loop.run(move |event, control_flow| {        

        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::RedrawRequested => {
                    let mut target = display.draw();
                    target.clear_color(0.0, 0.0, 0.0, 1.0);

                    let matrix = nalgebra::Matrix2::identity();

                    if let Some(image_pl) = &image_plane {
                        image_pl.draw(&mut target, &matrix);
                    }

                    target.finish().unwrap();
                }
                winit::event::WindowEvent::CloseRequested => control_flow.exit(),
                winit::event::WindowEvent::Resized(new_size) => {
                    display.resize(new_size.into());
                    },
                _ => (),
            },
            winit::event::Event::UserEvent(()) => {
                if let Ok(img) = rx.try_recv() {
                    match img {
                        Ok(img) => {image_plane = Some(ImagePlane::new(img, display.clone()));},
                        Err(e) => {window.set_title(format!("Radius | Error: {:?}", e).as_str());},
                    }
                    
                }
            },
            winit::event::Event::AboutToWait => {
                window.request_redraw();
            }
            _ => (),
        };
    }).unwrap();
}
