extern crate image;

use image::{GenericImageView,ImageBuffer,Rgba,Pixel};

fn main() {
    let size = 255;

    // Load sprite
    let sprite = image::open("./sprite.jpg").unwrap();
    let sprite_dims = sprite.dimensions();
    if sprite_dims != (size, size) {
        panic!("sprite must be {} x {}", size, size);
    }

    // Create buffer
    let mut buffer = ImageBuffer::<Rgba::<u8>, Vec<u8>>::new(size, size);

    // Blit sprite
    for x in 0..size {
        for y in 0..size {
            let sp = sprite.get_pixel(x, y);
            let mut bp = buffer.get_pixel_mut(x, y);

            let sc = sp.channels();
            let bc = bp.channels();

            if sc.len() != bc.len() {
                panic!("sprite has {} channels, buffer has {} channels, not equal",
                       sc.len(), bc.len());
            }

            let mut result: [u8; 4] = [0; 4];
            
            for i in 0..bc.len() {
                result[i] = sc[i] ^ bc[i];
            }

            *bp = Rgba(result);
        }
    }
    buffer.save("./buffer.jpg").unwrap();
}
