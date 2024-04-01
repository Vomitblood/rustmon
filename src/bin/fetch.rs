// this fetches the sprites and downloads to the user's local machine
// https://github.com/Vomitblood/pokesprite/archive/refs/heads/master.zip

// TODO: pass in url as argument and use it
// for now, we use this as default
const TARGET_URL: &str = "https://github.com/Vomitblood/pokesprite/archive/refs/heads/master.zip";
const WORKING_DIRECTORY: &str = "/tmp/rustmon/";

fn main() {
    // // create a working directory for the program to use
    // match create_working_directory() {
    //     Ok(_) => (),
    //     Err(e) => eprintln!("Error creating working directory: {}", e),
    // }

    // match download_colorscripts_archive(TARGET_URL) {
    //     Ok(_) => (),
    //     Err(e) => eprintln!("Error downloading file: {}", e),
    // }

    // TODO: extract here as default unless specified in flags
    extract_colorscripts_archive(std::path::Path::new(WORKING_DIRECTORY)).unwrap();
}

fn create_working_directory() -> std::io::Result<()> {
    println!("Creating working directory at {}...", WORKING_DIRECTORY);
    std::fs::create_dir(WORKING_DIRECTORY)?;
    return Ok(());
}

fn download_colorscripts_archive(target_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Fetching colorscripts archive...");

    let response = reqwest::blocking::get(target_url)?;

    let mut dest = std::fs::File::create(format!("{}colorscripts.zip", WORKING_DIRECTORY))?;

    let response_body = response.error_for_status()?.bytes()?;
    std::io::copy(&mut response_body.as_ref(), &mut dest)?;

    println!("Downloaded pokesprite.zip");

    return Ok(());
}

fn extract_colorscripts_archive(extract_location: &std::path::Path) -> zip::result::ZipResult<()> {
    // let archive_file = std::fs::File::open(&archive_path)?;
    let archive_file = std::fs::File::open(std::path::Path::new(
        format!("{}colorscripts.zip", WORKING_DIRECTORY).as_str(),
    ))?;
    let mut archive = zip::read::ZipArchive::new(std::io::BufReader::new(archive_file))?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_path = std::path::Path::new(file.name());
        let parent_dir = file_path
            .parent()
            .and_then(std::path::Path::file_name)
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or("");

        if (file
            .name()
            .starts_with("pokesprite-master/pokemon-gen8/regular/")
            && parent_dir == "regular")
            || (file
                .name()
                .starts_with("pokesprite-master/pokemon-gen8/shiny/")
                && parent_dir == "shiny")
        {
            let file_name = file_path
                .file_name()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or("");

            let outpath = extract_location.join(parent_dir).join(file_name);

            if !file.name().ends_with('/') {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(&p)?;
                    }
                }
                let mut outfile = std::fs::File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
        }
    }
    Ok(())
}
