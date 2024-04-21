pub const TARGET_URL: &str =
    "https://github.com/Vomitblood/pokesprite/archive/refs/heads/master.zip";

pub const DATA_DIRECTORY: once_cell::sync::Lazy<std::path::PathBuf> =
    once_cell::sync::Lazy::new(|| {
        dirs::data_dir()
            .map(|dir| dir.join("rustmon"))
            .expect("Data directory not found")
    });

pub const CACHE_DIRECTORY: once_cell::sync::Lazy<std::path::PathBuf> =
    once_cell::sync::Lazy::new(|| {
        dirs::cache_dir()
            .map(|dir| dir.join("rustmon"))
            .expect("Cache directory not found")
    });

// pub const GENERATIONS: [(&str, (u16, u16)); 8] = [
//     ("1", (1, 151)),
//     ("2", (152, 251)),
//     ("3", (252, 386)),
//     ("4", (387, 493)),
//     ("5", (494, 649)),
//     ("6", (650, 721)),
//     ("7", (722, 809)),
//     ("8", (810, 905)),
// ];
