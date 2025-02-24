use std::fs;

mod tests {
    use luminescent_image::{Args, convert_image_to_3d_model};

    use super::*;

    #[test]
    fn test_convert_image_to_3d_model() {
        fs::remove_file("test.stl").unwrap_or_default();
        let args = Args {
            input_file: "tests/golden_input.bmp".to_string(),
        };

        convert_image_to_3d_model(&args);

        assert!(compare_files("test.stl", "tests/golden_output.stl"));
    }

    fn compare_files(path1: &str, path2: &str) -> bool {
        let file1 = fs::read(path1).unwrap();
        let file2 = fs::read(path2).unwrap();
        file1 == file2
    }
}
