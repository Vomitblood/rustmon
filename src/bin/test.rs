use image::GenericImageView;
use std::io::Write;

fn convert_images_to_ascii() -> std::io::Result<()> {
    let path = std::path::Path::new("/home/vomitblood/Pictures/staging/vomitbloodnft 1080p.png");

    let img = image::open(path).unwrap();
    let ascii_art = convert_image_to_unicode_small(&img);

    let output_path = std::path::Path::new("/home/vomitblood/Pictures/staging/vomitbloodnft.txt");
    let mut file = std::fs::File::create(output_path)?;
    file.write_all(ascii_art.as_bytes())?;

    println!("Converted images to ASCII");

    Ok(())
}

fn convert_image_to_unicode_small(img: &image::DynamicImage) -> String {
    let mut unicode_sprite = String::new();
    let (width, height) = img.dimensions();

    for y in (0..height).step_by(2) {
        for x in 0..width {
            let upper_pixel = img.get_pixel(x, y);
            let lower_pixel = if y + 1 < height {
                img.get_pixel(x, y + 1)
            } else {
                // fallback to upper pixel if there's no lower pixel.
                upper_pixel
            };

            if upper_pixel[3] == 0 && lower_pixel[3] == 0 {
                unicode_sprite.push(' ');
            } else if upper_pixel[3] == 0 {
                unicode_sprite.push_str(&get_color_escape_code(lower_pixel, false));
                unicode_sprite.push('▄');
            } else if lower_pixel[3] == 0 {
                unicode_sprite.push_str(&get_color_escape_code(upper_pixel, false));
                unicode_sprite.push('▀');
            } else {
                unicode_sprite.push_str(&get_color_escape_code(upper_pixel, false));
                unicode_sprite.push_str(&get_color_escape_code(lower_pixel, true));
                unicode_sprite.push('▀');
            }
            unicode_sprite.push_str("\x1b[0m"); // Reset ANSI code after each character
        }
        unicode_sprite.push('\n'); // New line for each row, plus reset might be added here too if colors extend beyond.
    }

    unicode_sprite
}

fn get_color_escape_code(pixel: image::Rgba<u8>, background: bool) -> String {
    if pixel[3] == 0 {
        return format!("{}", crossterm::style::ResetColor);
    }

    let color = crossterm::style::Color::Rgb {
        r: pixel[0],
        g: pixel[1],
        b: pixel[2],
    };

    if background {
        format!("{}", crossterm::style::SetBackgroundColor(color))
    } else {
        format!("{}", crossterm::style::SetForegroundColor(color))
    }
}

fn main() {
    convert_images_to_ascii().unwrap();
}
