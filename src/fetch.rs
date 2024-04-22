use image::GenericImageView;
use std::io::Write;

pub fn fetch(extract_destination: &std::path::Path, verbose: bool) {
    // prep working directory
    match create_working_directory() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error creating working directory: {}", e);
        }
    };

    // prep output directory
    match create_output_directory(extract_destination) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error creating output directory: {}", e);
            cleanup().unwrap();
            std::process::exit(1);
        }
    };

    // download pokemon.json
    match fetch_pokemon_json() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error fetching pokemon_raw.json: {}", e);
            cleanup().unwrap();
            std::process::exit(1);
        }
    }

    // process pokemon_raw.json
    match process_pokemon_json(extract_destination) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error processing pokemon_raw.json: {}", e);
            cleanup().unwrap();
            std::process::exit(1);
        }
    };

    // download colorscripts archive
    match fetch_colorscripts_archive(crate::constants::TARGET_URL) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error fetching colorscripts archive: {}", e);
            cleanup().unwrap();
            std::process::exit(1);
        }
    };

    // extract colorscripts archive
    // now we have the raw images
    match extract_colorscripts_archive() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error extracting colorscripts archive: {}", e);
            cleanup().unwrap();
            std::process::exit(1);
        }
    };

    // crop images to content
    match crop_all_images_in_directory() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error cropping images: {}", e);
            cleanup().unwrap();
            std::process::exit(1);
        }
    };

    // convert images to unicode, both small and big
    match convert_images_to_ascii(extract_destination, verbose) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error converting images to ASCII: {}", e);
            cleanup().unwrap();
            std::process::exit(1);
        }
    };

    // cleanup
    match cleanup() {
        Ok(_) => (),
        Err(e) => eprintln!("Error cleaning up: {}", e),
    };
}

fn create_working_directory() -> std::io::Result<()> {
    println!(
        "Creating working directory at {:?}...",
        &*crate::constants::CACHE_DIRECTORY
    );
    // create intermediate directories also
    std::fs::create_dir(&*crate::constants::CACHE_DIRECTORY)?;
    println!("Created working directory");
    Ok(())
}

fn create_output_directory(output_directory_path: &std::path::Path) -> std::io::Result<()> {
    println!(
        "Creating output directory at {:?}...",
        output_directory_path
    );
    std::fs::create_dir_all(output_directory_path)?;
    println!("Created output directory");
    Ok(())
}

fn fetch_pokemon_json() -> Result<(), Box<dyn std::error::Error>> {
    println!("Fetching pokemon_raw.json...");

    // create a client with a timeout of 4 seconds
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_millis(4000))
        .build()?;

    let mut attempts: u8 = 0;
    const MAX_ATTEMPTS: u8 = 5;

    loop {
        match client
            .get("https://raw.githubusercontent.com/Vomitblood/pokesprite/master/data/pokemon.json")
            .send()
        {
            Ok(response) => {
                if response.status().is_success() {
                    let mut dest = std::fs::File::create(
                        &*crate::constants::CACHE_DIRECTORY
                            .to_path_buf()
                            .join("pokemon_raw.json"),
                    )?;

                    let response_body = response.bytes()?;
                    std::io::copy(&mut response_body.as_ref(), &mut dest)?;

                    println!("Downloaded pokemon_raw.json");

                    return Ok(());
                } else {
                    // handle unsuccessful response status codes
                    eprintln!("Error fetching pokemon_raw.json: {}", response.status());
                }
            }
            Err(e) => {
                attempts += 1;
                eprintln!("Attempt {} failed: {}", attempts, e);
                if attempts >= MAX_ATTEMPTS {
                    return Err(e.into());
                }

                // delay before retrying
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
    }
}

fn process_pokemon_json(
    output_directory_path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating pokemon.json...");

    let pokemon_raw_json_path = &*crate::constants::CACHE_DIRECTORY.join("pokemon_raw.json");

    let pokemon_collection = read_pokemon_file(pokemon_raw_json_path)?;

    let processed_pokemon = transform_pokemon_data(&pokemon_collection.entries);

    // serialize the processed data to JSON
    let serialized_pokemon = serde_json::to_string_pretty(&processed_pokemon)?;

    // write processed data to file
    std::fs::write(
        output_directory_path.join("pokemon.json"),
        serialized_pokemon,
    )?;

    println!("Generated pokemon.json");

    Ok(())
}

fn read_pokemon_file(
    file_path: &std::path::Path,
) -> Result<crate::structs::PokemonRawCollection, Box<dyn std::error::Error>> {
    // open the file in read only mode
    let file = std::fs::File::open(file_path)?;
    let reader = std::io::BufReader::new(file);

    // deserialize the into pokemoncollection
    let collection = serde_json::from_reader(reader)?;

    Ok(collection)
}

fn transform_pokemon_data(
    pokemon_collection: &std::collections::HashMap<String, crate::structs::PokemonRaw>,
) -> Vec<crate::structs::Pokemon> {
    let mut processed_pokemons: Vec<crate::structs::Pokemon> = pokemon_collection
        .iter()
        .map(|(_key, p)| {
            let mut forms = p
                .gen_8
                .forms
                .keys()
                .map(|key| match key.as_str() {
                    "$" => "regular".to_string(),
                    _ => key.clone(),
                })
                .collect::<Vec<String>>();

            // ensure `regular` is first then sort remaining forms alphabetically
            // ocd af
            forms.sort();
            if let Some(pos) = forms.iter().position(|x| x == "regular") {
                forms.remove(pos);
                forms.insert(0, "regular".to_string());
            }

            crate::structs::Pokemon {
                // remove leading zeros from the pokedex number
                pokedex: p.idx.trim_start_matches('0').to_string(),
                // use the slug as the name
                // this is because i am too lazy to decapitalize the name
                // also in case of name =/= slug
                name: p.slug.eng.clone(),
                forms,
            }
        })
        .collect();

    // sort the vector by pokedex number
    processed_pokemons.sort_by(|a, b| {
        a.pokedex
            .parse::<u32>()
            .unwrap_or(0)
            .cmp(&b.pokedex.parse::<u32>().unwrap_or(0))
    });

    processed_pokemons
}

fn fetch_colorscripts_archive(target_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Fetching colorscripts archive...");

    // create a client with a timeout of 4 seconds
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_millis(4000))
        .build()?;

    let mut attempts: u8 = 0;
    const MAX_ATTEMPTS: u8 = 5;

    loop {
        match client.get(target_url).send() {
            Ok(response) => {
                if response.status().is_success() {
                    let mut dest = std::fs::File::create(
                        &*crate::constants::CACHE_DIRECTORY
                            .to_path_buf()
                            .join("pokesprite.zip"),
                    )?;

                    let response_body = response.bytes()?;
                    std::io::copy(&mut response_body.as_ref(), &mut dest)?;

                    println!("Downloaded colorscripts archive");

                    return Ok(());
                } else {
                    // handle unsuccessful response status codes
                    eprintln!("Error fetching colorscripts archive: {}", response.status());
                }
            }
            Err(e) => {
                attempts += 1;
                eprintln!("Attempt {} failed: {}", attempts, e);
                if attempts >= MAX_ATTEMPTS {
                    return Err(e.into());
                }

                // delay before retrying
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
    }
}

fn extract_colorscripts_archive() -> zip::result::ZipResult<()> {
    println!("Extracting colorscripts archive...");

    let archive_file = std::fs::File::open(
        &*crate::constants::CACHE_DIRECTORY
            .to_path_buf()
            .join("pokesprite.zip"),
    )?;
    let mut archive = zip::read::ZipArchive::new(std::io::BufReader::new(archive_file))?;

    // iterate over every single file in the archive
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_path = std::path::Path::new(file.name());

        let parent_dir = file_path
            .parent()
            .and_then(std::path::Path::file_name)
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or("");

        // check if the file is a in the correct directory that is NOT a directory
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
                .unwrap();

            let outpath = &*crate::constants::CACHE_DIRECTORY
                .join("raw_images")
                .join(parent_dir)
                .join(file_name);

            if !file.name().ends_with('/') {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(p)?;
                    }
                }
                let mut outfile = std::fs::File::create(outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            };
        };
    }

    println!("Extracted colorscripts archive");

    Ok(())
}

fn crop_all_images_in_directory() -> std::io::Result<()> {
    println!("Cropping images...");

    // make sure the cropped_images directory exists
    std::fs::create_dir_all(
        &*crate::constants::CACHE_DIRECTORY
            .to_path_buf()
            .join("cropped_images"),
    )?;

    // do for both regular and shiny subdirectories
    for subdirectory in ["regular", "shiny"].iter() {
        let input_subdirectory_path = &*crate::constants::CACHE_DIRECTORY
            .to_path_buf()
            .join("raw_images")
            .join(subdirectory);
        let output_subdirectory_path = &*crate::constants::CACHE_DIRECTORY
            .to_path_buf()
            .join("cropped_images")
            .join(subdirectory);

        std::fs::create_dir_all(output_subdirectory_path)?;

        for entry in std::fs::read_dir(input_subdirectory_path)? {
            let entry = entry?;
            let path = entry.path();

            let output_path = output_subdirectory_path.join(path.file_name().unwrap());
            crop_to_content(&path, &output_path).unwrap();
        }
    }

    println!("Cropped images");

    Ok(())
}

fn crop_to_content(
    input_path: &std::path::Path,
    output_path: &std::path::Path,
) -> image::ImageResult<image::DynamicImage> {
    // load the image
    let img = image::open(input_path)?;

    let (width, height) = img.dimensions();
    let mut min_x = width;
    let mut min_y = height;
    let mut max_x = 0;
    let mut max_y = 0;

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            if pixel[3] != 0 {
                // if pixel is not transparent
                if x < min_x {
                    min_x = x;
                }
                if y < min_y {
                    min_y = y;
                }
                if x > max_x {
                    max_x = x;
                }
                if y > max_y {
                    max_y = y;
                }
            }
        }
    }

    let cropped_width = max_x - min_x + 1;
    let cropped_height = max_y - min_y + 1;

    let mut cropped_img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        image::ImageBuffer::new(cropped_width, cropped_height);

    for y in 0..cropped_height {
        for x in 0..cropped_width {
            let pixel = img.get_pixel(x + min_x, y + min_y);
            cropped_img.put_pixel(x, y, pixel);
        }
    }

    let cropped_img = image::DynamicImage::ImageRgba8(cropped_img);

    // write the cropped image
    cropped_img.save(output_path)?;

    Ok(cropped_img)
}

fn convert_images_to_ascii(
    output_directory_path: &std::path::Path,
    verbose: bool,
) -> std::io::Result<()> {
    println!("Extract destination: {:?}", output_directory_path);
    println!("Converting images to ASCII...");

    for size in ["small", "big"].iter() {
        for subdirectory in ["regular", "shiny"].iter() {
            let input_subdirectory_path = &*crate::constants::CACHE_DIRECTORY
                .join("cropped_images")
                .join(subdirectory);
            let output_subdirectory_path = output_directory_path
                .join("colorscripts")
                .join(size)
                .join(subdirectory);

            std::fs::create_dir_all(&output_subdirectory_path)?;

            for entry in std::fs::read_dir(input_subdirectory_path)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_file() {
                    let img = image::open(&path).unwrap();
                    let ascii_art = if *size == "small" {
                        convert_image_to_unicode_small(&img)
                    } else {
                        convert_image_to_unicode_big(&img)
                    };

                    // print for fun
                    if verbose {
                        println!("{}", ascii_art);
                    };

                    let output_path = output_subdirectory_path.join(path.file_stem().unwrap());
                    let mut file = std::fs::File::create(output_path)?;
                    file.write_all(ascii_art.as_bytes())?;
                }
            }
        }
    }

    println!("Converted images to ASCII");

    Ok(())
}

fn convert_image_to_unicode_small(img: &image::DynamicImage) -> String {
    let mut unicode_sprite = String::new();
    let (width, height) = img.dimensions();

    for y in (0..height).step_by(2) {
        for x in 0..width {
            let upper_pixel = img.get_pixel(x, y);
            let lower_pixel = if y + 1 < height {
                img.get_pixel(x, y + 1)
            } else {
                // fallback to upper pixel if there's no lower pixel.
                upper_pixel
            };

            if upper_pixel[3] == 0 && lower_pixel[3] == 0 {
                unicode_sprite.push(' ');
            } else if upper_pixel[3] == 0 {
                unicode_sprite.push_str(&get_color_escape_code(lower_pixel, false));
                unicode_sprite.push('▄');
            } else if lower_pixel[3] == 0 {
                unicode_sprite.push_str(&get_color_escape_code(upper_pixel, false));
                unicode_sprite.push('▀');
            } else {
                unicode_sprite.push_str(&get_color_escape_code(upper_pixel, false));
                unicode_sprite.push_str(&get_color_escape_code(lower_pixel, true));
                unicode_sprite.push('▀');
            }
            unicode_sprite.push_str("\x1b[0m"); // Reset ANSI code after each character
        }
        unicode_sprite.push('\n'); // New line for each row, plus reset might be added here too if colors extend beyond.
    }

    unicode_sprite
}

fn convert_image_to_unicode_big(img: &image::DynamicImage) -> String {
    let mut unicode_sprite = String::new();
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);

            if pixel[3] == 0 {
                unicode_sprite.push_str("  ");
            } else {
                unicode_sprite.push_str(&get_color_escape_code(pixel, false));
                unicode_sprite.push_str("██");
            }
        }
        unicode_sprite.push('\n');
    }

    unicode_sprite
}

fn get_color_escape_code(pixel: image::Rgba<u8>, background: bool) -> String {
    if pixel[3] == 0 {
        return format!("{}", crossterm::style::ResetColor);
    }

    let color = crossterm::style::Color::Rgb {
        r: pixel[0],
        g: pixel[1],
        b: pixel[2],
    };

    if background {
        format!("{}", crossterm::style::SetBackgroundColor(color))
    } else {
        format!("{}", crossterm::style::SetForegroundColor(color))
    }
}

fn cleanup() -> std::io::Result<()> {
    println!("Cleaning up...");

    std::fs::remove_dir_all(&*crate::constants::CACHE_DIRECTORY)?;

    println!("Cleaned up");

    Ok(())
}
