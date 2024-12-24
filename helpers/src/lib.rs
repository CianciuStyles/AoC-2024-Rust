pub mod direction;
pub mod position;
pub mod position_with_direction;

#[macro_export]
macro_rules! input_file {
    () => {
        std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("data")
                .join("input.txt"),
        )
        .unwrap()
    };
}

#[macro_export]
macro_rules! sample_file {
    () => {
        std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("data")
                .join("sample.txt"),
        )
        .unwrap()
    };
}
