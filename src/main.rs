use image::{GenericImageView, ImageReader, Pixel};

const IMAGE_MAX_SIZE: f32 = 100.0;
const IMAGE_MIN_HEIGHT: f32 = 0.2;
const IMAGE_MAX_HEIGHT: f32 = 8.2;

fn main() {
    let img = ImageReader::open("/mnt/data/Obrazy/wybrane/20210928_172006.jpg")
        .unwrap()
        .decode()
        .unwrap();
    let img = img.resize(100, 100, image::imageops::FilterType::Nearest);

    let mut coordinates: Vec<[f32; 3]> = img
        .pixels()
        .map(|(x, y, pixel)| [x as f32, y as f32, pixel.to_luma().channels()[0] as f32])
        .collect();

    let width = img.width() as f32;
    let height = img.height() as f32;

    coordinates = scale_coordinates(coordinates, width, height);
    coordinates = move_up_coordinates(coordinates);
}

fn scale_coordinates(coordinates: Vec<[f32; 3]>, width: f32, height: f32) -> Vec<[f32; 3]> {
    let scale_x = IMAGE_MAX_SIZE / width;
    let scale_y = IMAGE_MAX_SIZE / height;
    let scale_xy = scale_x.min(scale_y);
    let scale_z = (IMAGE_MAX_HEIGHT - IMAGE_MIN_HEIGHT) / 256.0;

    coordinates
        .into_iter()
        .map(|[x, y, z]| [x * scale_xy, y * scale_xy, z * scale_z])
        .collect()
}

fn move_up_coordinates(mut coordinates: Vec<[f32; 3]>) -> Vec<[f32; 3]> {
    coordinates
        .iter_mut()
        .for_each(|point| point[2] += IMAGE_MIN_HEIGHT);
    coordinates
}
