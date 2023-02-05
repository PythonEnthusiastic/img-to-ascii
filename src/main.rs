use image::GenericImageView;
use colored::*;
use image::Rgba;

fn print_ascii(val: u32, pixel: Rgba<u8>) {
    let val_index = val / 32;
    let ascii_char = [" ", ".", "`", "'", "\"", "^", "~", "*", ":", ";", "+", "(", ")", "<", ">", "$", "@"];
    let colored_char = ascii_char[val_index as usize].on_truecolor(pixel[0], pixel[1], pixel[2]);
    print!("{}{}", colored_char, colored_char);
}

// [" ", ".", "^", ":", ";", "<", ">", "@"]
// [" ", ".", "`", "'", "\"", "^", "~", "*", ":", ";", "+", "(", ")", "<", ">", "$", "@"]
// [" ", ".", "'", "^", ":", ";", "<", ">", "+", "(", ")", "$", "@"]
// $@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~<>i!lI;:,"^`'.

fn main() {
    let img = image::open("src/verycool.jpeg").unwrap();
    let (width, heigth) = img.dimensions();

    for y in 0..heigth {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let brigthness: u32 = (pixel[0] / 3 + pixel[1] / 3 + pixel[2] / 3) as u32;

            print_ascii(brigthness, pixel);
        }
        println!("");
    }
}