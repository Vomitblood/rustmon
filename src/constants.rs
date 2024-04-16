pub static TARGET_URL: &str =
    "https://github.com/Vomitblood/pokesprite/archive/refs/heads/master.zip";

pub static DATA_DIRECTORY: once_cell::sync::Lazy<std::path::PathBuf> =
    once_cell::sync::Lazy::new(|| {
        dirs::data_dir()
            .map(|dir| dir.join("rustmon"))
            .expect("Data directory not found")
    });

pub static CACHE_DIRECTORY: once_cell::sync::Lazy<std::path::PathBuf> =
    once_cell::sync::Lazy::new(|| {
        dirs::cache_dir()
            .map(|dir| dir.join("rustmon"))
            .expect("Cache directory not found")
    });
