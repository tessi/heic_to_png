use image::{ImageBuffer, ImageFormat, Rgba};
use libheif_rs::{ColorSpace, HeifContext, RgbChroma};
use rustler::Binary;
use std::io::Cursor;
use std::io::{BufWriter, Write};

#[rustler::nif(schedule = "DirtyCpu")]
fn heic_to_png(image: Binary) -> Result<Vec<u8>, String> {
    let ctx = HeifContext::read_from_bytes(image.as_slice()).map_err(stringify)?;
    let handle = ctx.primary_image_handle().map_err(stringify)?;
    let alpha = handle.has_alpha_channel();
    let depth = handle.luma_bits_per_pixel(); // max 16

    let chroma = match (alpha, depth > 8) {
        (false, false) => RgbChroma::Rgb,
        (false, true) => RgbChroma::HdrRgbLe,
        (true, false) => RgbChroma::Rgba,
        (true, true) => RgbChroma::HdrRgbaLe,
    };
    let image = handle
        .decode(ColorSpace::Rgb(chroma), false)
        .map_err(stringify)?;
    let interleaved_plane = image.planes().interleaved.ok_or("no interleaved plane")?;
    let pixels = interleaved_plane.data;
    let width = interleaved_plane.width;
    let height = interleaved_plane.height;
    let stride = interleaved_plane.stride; // bytes per line
    let bytes_per_component = (depth + 7) / 8; // 1 or 2

    // println!("alpha: {}, bytes_per_component: {}, depth: {}, chroma: {:?}", alpha, bytes_per_component, depth, chroma);

    let bytes = if bytes_per_component <= 1 {
        // uses 1 byte per color
        if alpha {
            let image = ImageBuffer::from_fn(width, height, |x, y| {
                let i = y as usize * stride + x as usize * 4;
                image::Rgba([pixels[i], pixels[i + 1], pixels[i + 2], pixels[i + 3]])
            });
            serialize_image_buffer_u8(image)?
        } else {
            let image = ImageBuffer::from_fn(width, height, |x, y| {
                let i = y as usize * stride + x as usize * 3;
                image::Rgba([pixels[i], pixels[i + 1], pixels[i + 2], u8::MAX])
            });
            serialize_image_buffer_u8(image)?
        }
    } else {
        // uses 2 bytes per color
        let shift = 16 - depth;
        if alpha {
            let image = ImageBuffer::from_fn(width, height, |x, y| {
                let i = y as usize * stride + x as usize * 4 * 2;
                image::Rgba([
                    ((pixels[i + 1] as u16) << 8 | (pixels[i] as u16)) << shift,
                    ((pixels[i + 3] as u16) << 8 | (pixels[i + 2] as u16)) << shift,
                    ((pixels[i + 5] as u16) << 8 | (pixels[i + 4] as u16)) << shift,
                    ((pixels[i + 7] as u16) << 8 | (pixels[i + 6] as u16)) << shift,
                ])
            });
            serialize_image_buffer_u16(image)?
        } else {
            let image = ImageBuffer::from_fn(width, height, |x, y| {
                let i = y as usize * stride + x as usize * 3 * 2;
                image::Rgba([
                    ((pixels[i + 1] as u16) << 8 | (pixels[i] as u16)) << shift,
                    ((pixels[i + 3] as u16) << 8 | (pixels[i + 2] as u16)) << shift,
                    ((pixels[i + 5] as u16) << 8 | (pixels[i + 4] as u16)) << shift,
                    ((u8::MAX as u16) << 8 | (u8::MAX as u16)) << shift,
                ])
            });
            serialize_image_buffer_u16(image)?
        }
    };
    Ok(bytes)
}

fn serialize_image_buffer_u8(image: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<Vec<u8>, String> {
    let mut buffer = BufWriter::new(Cursor::new(vec![]));
    image
        .write_to(&mut buffer, ImageFormat::Png)
        .map_err(stringify)?;
    buffer.flush().map_err(stringify)?;
    let bytes = buffer.into_inner().map_err(stringify)?.into_inner();
    Ok(bytes)
}

fn serialize_image_buffer_u16(image: ImageBuffer<Rgba<u16>, Vec<u16>>) -> Result<Vec<u8>, String> {
    let mut buffer = BufWriter::new(Cursor::new(vec![]));
    image
        .write_to(&mut buffer, ImageFormat::Png)
        .map_err(stringify)?;
    buffer.flush().map_err(stringify)?;
    let bytes = buffer.into_inner().map_err(stringify)?.into_inner();
    Ok(bytes)
}

fn stringify<Displayable: std::fmt::Display>(e: Displayable) -> String {
    format!("{}", e)
}

rustler::init!("Elixir.HeicToPng.Native", [heic_to_png]);
