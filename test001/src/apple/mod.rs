pub mod conf;
pub mod queue;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
