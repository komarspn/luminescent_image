use image::{GenericImageView, ImageReader, Pixel};

fn main() {
    let img = ImageReader::open("test.bmp").unwrap().decode().unwrap();

    let coordinates: Vec<[u32; 3]> = img
        .pixels()
        .map(|(x, y, pixel)| [x, y, pixel.to_luma().channels()[0] as u32])
        .collect();

    for coord in coordinates {
        println!("x: {}, y: {}, value: {}", coord[0], coord[1], coord[2]);
    }
}
