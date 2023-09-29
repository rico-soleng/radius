use glium::Surface;
use std::env;
use std::fs::File;
use std::io::Read;
use std::sync::mpsc;
use std::thread;

mod draw;
use draw::Draw;

mod r_image;
use r_image::{Image, ImagePlane};

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
        let file = File::open(&path).expect("File not found!");
        let buf: Vec<u8> = (&file).bytes().map(|b| b.unwrap()).collect();
        let guess = image::guess_format(&buf);

        if guess.is_ok() && path.split('.').last() != Some("CR2") {
            let format = guess.unwrap();
            let image =
                image::load_from_memory_with_format(&buf, format).expect("Failed to decode image!");

            if format == image::ImageFormat::OpenExr {
                tx.send((image.to_rgba32f(), true)).unwrap();
            } else {
                tx.send((image.to_rgba32f(), false)).unwrap();
            }
        } else {
            let raw_image = rawloader::decode_file(&path).expect("Failed");
            let data = &raw_image.data;
            let bl = raw_image.blacklevels.map(|x| (x as f32) / 512.0);

            let image = image::ImageBuffer::from_fn(
                (raw_image.width / 2) as u32,
                (raw_image.height / 2) as u32,
                |x, y| {
                    let tx = x * 2;
                    let ty = y * 2;

                    let p = match &data {
                        rawloader::RawImageData::Integer(p) => (
                            (p[((tx + 1) + ty * raw_image.width as u32) as usize] as f32) / 512.0,
                            (p[(tx + ty * raw_image.width as u32) as usize] as f32) / 512.0,
                            (p[(tx + (ty + 1) * raw_image.width as u32) as usize] as f32) / 512.0,
                        ),
                        rawloader::RawImageData::Float(p) => (
                            p[((tx + 1) + ty * raw_image.width as u32) as usize] as f32,
                            p[(tx + ty * raw_image.width as u32) as usize] as f32,
                            p[(tx + (ty + 1) * raw_image.width as u32) as usize] as f32,
                        ),
                    };
                    image::Rgba([p.0 - bl[0], p.1 - bl[1], p.2 - bl[2], 1.0])
                },
            );
            tx.send((image, true)).unwrap();
        }
        println!("Image!");
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

                if let Some(image_pl) = &image_plane {
                    image_pl.draw(&mut target, dimensions);
                }

                target.finish().unwrap();
            }
            winit::event::Event::MainEventsCleared => {
                window.request_redraw();
            }
            winit::event::Event::UserEvent(()) => {
                if let Ok(img) = rx.try_recv() {
                    let open_domain = img.1;
                    let image = img.0;

                    let img = Image { image, open_domain };
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
