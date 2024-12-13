use std::path::Path;
use image::{Luma, GrayImage, open};

fn add_to_luma(og_luma: &Luma<u8>, added_val: u8) -> Luma<u8> {
    let new_val = og_luma[0].saturating_add(added_val);
    Luma([new_val])
}

fn floyd_steinberg_dithering(img: &mut GrayImage) {
    let (width, height) = img.dimensions();

    for y in 0..height as i32 {
        for x in 0..width as i32 {
            let old_p = img.get_pixel(x as u32, y as u32).0[0];
            let new_p = if old_p > 127 { 255 } else { 0 }; // clamp to black and white only
            let c = Luma([new_p]);
            img.put_pixel(x as u32, y as u32, c);

            let quant_err = old_p as i16 - new_p as i16; // for error propagation

            // neighbors from definition
            let quant_err_u8 = |err: i16, factor: i16| -> u8 {
                (err * factor / 16).clamp(-255, 255).max(0) as u8
            };

            if x + 1 < width as i32 {
                // right neighbor
                img.put_pixel(
                    (x + 1) as u32,
                    y as u32,
                    add_to_luma(
                        img.get_pixel((x + 1) as u32, y as u32),
                        quant_err_u8(quant_err, 7),
                    ),
                );
            }
            if x - 1 >= 0 && y + 1 < height as i32 {
                // bottom-left neighbor
                img.put_pixel(
                    (x - 1) as u32,
                    (y + 1) as u32,
                    add_to_luma(
                        img.get_pixel((x - 1) as u32, (y + 1) as u32),
                        quant_err_u8(quant_err, 3),
                    ),
                );
            }
            if y + 1 < height as i32 {
                // bottom neighbor
                img.put_pixel(
                    x as u32,
                    (y + 1) as u32,
                    add_to_luma(
                        img.get_pixel(x as u32, (y + 1) as u32),
                        quant_err_u8(quant_err, 5),
                    ),
                );
            }
            if x + 1 < width as i32 && y + 1 < height as i32 {
                // bottom-right neighbor
                img.put_pixel(
                    (x + 1) as u32,
                    (y + 1) as u32,
                    add_to_luma(
                        img.get_pixel((x + 1) as u32, (y + 1) as u32),
                        quant_err_u8(quant_err, 1),
                    ),
                );
            }
        }
    }
}

fn main() {
    let mut imgs = Vec::new();

    for i in 0..6{
        let file_name = format!("test_{i}.png");
        let path = Path::new(&file_name);

        match open(path) {
            Ok(img) => {imgs.push(img.into_luma8());}
            Err(e) => {
                eprintln!("Failed to load {file_name}: {e}");
                continue;
            }
        }
    }

    /* OBSOLETE: Original test for one image only
    let mut img = image::open("test.jpg").unwrap().into_luma8();
    floyd_steinberg_dithering(&mut img);
    img.save("dithered_test.png").unwrap();
    */

    for (i, img) in imgs.iter_mut().enumerate(){
        floyd_steinberg_dithering(img);
        let output_file_name = format!("output_{i}.png");
        if let Err(e) = img.save(&output_file_name) {
            eprintln!("Failed to save {output_file_name}: {e}");
        }
        else{
            println!("Saved {output_file_name}")
        };
    }
}
