use clap::Parser;

fn main() {
    let args = luminescent_image::Args::parse();
    luminescent_image::convert_image_to_3d_model(&args);
}
