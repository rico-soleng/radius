use anyhow::Context;
use std::{fs::File, io::Read};

use crate::r_image;

pub fn open_file(path: String) -> anyhow::Result<r_image::Image> {
    let file = File::open(&path).context("File not found!")?;
    let mut buf: Vec<u8> = Vec::new();
    (&file).read_to_end(&mut buf)?;
    let guess = image::guess_format(&buf);

    if guess.is_ok() && path.split('.').last() != Some("CR2") {
        let format = guess?;
        let image = image::load_from_memory_with_format(&buf, format)?;

        if format == image::ImageFormat::OpenExr {
            Ok(r_image::Image {
                image: image.to_rgba32f(),
                open_domain: true,
            })
        } else {
            Ok(r_image::Image {
                image: image.to_rgba32f(),
                open_domain: false,
            })
        }
    } else {
        let raw_image = rawloader::decode_file(&path)?;
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
        Ok(r_image::Image {
            image,
            open_domain: true,
        })
    }
}
