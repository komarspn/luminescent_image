use std::fs::OpenOptions;

use clap::Parser;
use image::{GenericImageView, ImageReader, Pixel};
use stl_io::{Normal, Triangle, Vertex};

const IMAGE_MIN_HEIGHT: f32 = 0.2;
const IMAGE_MAX_HEIGHT: f32 = 8.2;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub input_file: String,

    #[arg(short, long)]
    pub output_file: String,

    #[arg(short = 's', long, default_value_t = 150.0)]
    pub output_stl_max_size: f32,
}

pub fn convert_image_to_3d_model(args: &Args) {
    let img = ImageReader::open(&args.input_file)
        .unwrap()
        .decode()
        .unwrap();
    let img = img.resize(300, 300, image::imageops::FilterType::Nearest);

    let mut coordinates: Vec<[f32; 3]> = img
        .pixels()
        .map(|(x, y, pixel)| [x as f32, y as f32, pixel.to_luma().channels()[0] as f32])
        .collect();

    let width = img.width();
    let height = img.height();

    coordinates = revert_z_coordinates(coordinates);
    coordinates = scale_coordinates(coordinates, width, height, args.output_stl_max_size);
    coordinates = move_up_coordinates(coordinates);

    let mut triangles = convert_to_triangles(&coordinates, width, height);
    triangles.extend(generate_base(width, height, args.output_stl_max_size));
    triangles.extend(generate_sides(&coordinates, width, height));

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&args.output_file)
        .unwrap();

    stl_io::write_stl(&mut file, triangles.iter()).unwrap();
}

fn revert_z_coordinates(mut coordinates: Vec<[f32; 3]>) -> Vec<[f32; 3]> {
    coordinates
        .iter_mut()
        .for_each(|point| point[2] = 255.0 - point[2]);
    coordinates
}

fn scale_coordinates(
    coordinates: Vec<[f32; 3]>,
    width: u32,
    height: u32,
    output_stl_max_size: f32,
) -> Vec<[f32; 3]> {
    let scale_x = output_stl_max_size / width as f32;
    let scale_y = output_stl_max_size / height as f32;
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

fn convert_to_triangles(scale_coordinates: &[[f32; 3]], width: u32, height: u32) -> Vec<Triangle> {
    let mut result = Vec::new();
    for y in 0..height - 1 {
        for x in 0..width - 1 {
            let p1 = Vertex::new(scale_coordinates[(y * width + x) as usize]);
            let p2 = Vertex::new(scale_coordinates[(y * width + x + 1) as usize]);
            let p3 = Vertex::new(scale_coordinates[((y + 1) * width + x) as usize]);
            let p4 = Vertex::new(scale_coordinates[((y + 1) * width + x + 1) as usize]);

            let normal = Normal::new([0.0, 0.0, 1.0]);

            result.push(Triangle {
                normal,
                vertices: [p1, p2, p3],
            });
            result.push(Triangle {
                normal,
                vertices: [p2, p4, p3],
            });
        }
    }
    result
}

fn generate_base(width: u32, height: u32, output_stl_max_size: f32) -> Vec<Triangle> {
    let width = width as f32;
    let height = height as f32;
    let scale_xy = (output_stl_max_size / width).min(output_stl_max_size / height);

    let normal = Normal::new([0.0, 0.0, -1.0]);
    let p1 = Vertex::new([0.0, 0.0, 0.0]);
    let p2 = Vertex::new([0.0, height * scale_xy, 0.0]);
    let p3 = Vertex::new([width * scale_xy, 0.0, 0.0]);
    let p4 = Vertex::new([width * scale_xy, height * scale_xy, 0.0]);

    vec![
        Triangle {
            normal,
            vertices: [p1, p2, p3],
        },
        Triangle {
            normal,
            vertices: [p2, p4, p3],
        },
    ]
}

fn generate_sides(coordinates: &[[f32; 3]], width: u32, height: u32) -> Vec<Triangle> {
    let mut result = Vec::new();

    let normal = Normal::new([0.0, -1.0, 0.0]);
    for x in 0..width - 1 {
        let p1_coordinate = coordinates[x as usize];
        let p2_coordinate = coordinates[(x + 1) as usize];

        let p1 = Vertex::new(p1_coordinate);
        let p2 = Vertex::new(p2_coordinate);
        let p3 = Vertex::new([p1_coordinate[0], p1_coordinate[1], 0.0]);
        let p4 = Vertex::new([p2_coordinate[0], p2_coordinate[1], 0.0]);

        result.push(Triangle {
            normal,
            vertices: [p1, p2, p3],
        });
        result.push(Triangle {
            normal,
            vertices: [p2, p4, p3],
        });
    }

    let normal = Normal::new([0.0, 1.0, 0.0]);
    for x in 0..width - 1 {
        let p1_coordinate = coordinates[((height - 1) * width + x) as usize];
        let p2_coordinate = coordinates[((height - 1) * width + x + 1) as usize];
        let p1 = Vertex::new(p1_coordinate);
        let p2 = Vertex::new(p2_coordinate);
        let p3 = Vertex::new([p1_coordinate[0], p1_coordinate[1], 0.0]);
        let p4 = Vertex::new([p2_coordinate[0], p2_coordinate[1], 0.0]);

        result.push(Triangle {
            normal,
            vertices: [p1, p2, p3],
        });
        result.push(Triangle {
            normal,
            vertices: [p2, p4, p3],
        });
    }

    let normal = Normal::new([-1.0, 0.0, 0.0]);
    for y in 0..height - 1 {
        let p1_coordinate = coordinates[(y * width) as usize];
        let p2_coordinate = coordinates[((y + 1) * width) as usize];
        let p1 = Vertex::new(p1_coordinate);
        let p2 = Vertex::new(p2_coordinate);
        let p3 = Vertex::new([p1_coordinate[0], p1_coordinate[1], 0.0]);
        let p4 = Vertex::new([p2_coordinate[0], p2_coordinate[1], 0.0]);

        result.push(Triangle {
            normal,
            vertices: [p1, p2, p3],
        });
        result.push(Triangle {
            normal,
            vertices: [p2, p4, p3],
        });
    }

    let normal = Normal::new([1.0, 0.0, 0.0]);
    for y in 0..height - 1 {
        let p1_coordinate = coordinates[(y * width + width - 1) as usize];
        let p2_coordinate = coordinates[((y + 1) * width + width - 1) as usize];
        let p1 = Vertex::new(p1_coordinate);
        let p2 = Vertex::new(p2_coordinate);
        let p3 = Vertex::new([p1_coordinate[0], p1_coordinate[1], 0.0]);
        let p4 = Vertex::new([p2_coordinate[0], p2_coordinate[1], 0.0]);

        result.push(Triangle {
            normal,
            vertices: [p1, p2, p3],
        });
        result.push(Triangle {
            normal,
            vertices: [p2, p4, p3],
        });
    }

    result
}
