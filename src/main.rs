use clap::Parser;

fn main() {
    let args = luminescent_image::Args::parse();
    let config: luminescent_image::Config =
        confy::load("luminescent_image", None).unwrap_or_default();
    luminescent_image::convert_image_to_3d_model(&args, &config);
}
