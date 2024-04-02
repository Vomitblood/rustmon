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

// fn get_directories() -> (std::path::PathBuf, std::path::PathBuf) {
//     let data_directory: std::path::PathBuf = match dirs::data_dir() {
//         Some(dir) => dir.join("rustmon"),
//         None => {
//             println!("Data directory not found");
//             std::process::exit(1);
//         }
//     };

//     let cache_directory: std::path::PathBuf = match dirs::cache_dir() {
//         Some(dir) => dir.join("rustmon"),
//         None => {
//             println!("Cache directory not found");
//             std::process::exit(1);
//         }
//     };

//     return (data_directory, cache_directory);
// }
