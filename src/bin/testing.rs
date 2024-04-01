use std::fs;

fn main() {
    let contents = fs::read_to_string("testing/testing.fuck").unwrap();
    let mut found: bool = false;

    for line in contents.lines() {
        match line {
            "1" | "2" => {
                found = true;
                break;
            }

            // if not found
            _ => found = false,
        }
    }

    if found == true {
        println!("Gotcha bitch");
    } else {
        println!("Never mind")
    }
}

// use rand::{thread_rng, Rng};
// use std::io::{self, stdout, Write};
// use std::num::ParseIntError;

// #[derive(Debug)]
// enum Error {
//     InvalidNumber(ParseIntError),
//     OutOfRange,
// }

// fn main() {
//     loop {
//         let random_number: u8 = generate_random_number();
//         loop {
//             match get_user_number() {
//                 Ok(user_number) => {
//                     if user_number < random_number {
//                         println!("Too low!");
//                     } else if user_number > random_number {
//                         println!("Too high!");
//                     } else {
//                         println!("You guessed the correct number!");
//                         break;
//                     }
//                 }
//                 Err(e) => println!("{:?}", e),
//             }
//         }
//         let user_choice = get_user_choice();
//         if user_choice != "y" {
//             break;
//         }
//     }
// }

// fn generate_random_number() -> u8 {
//     let mut rng = thread_rng();
//     rng.gen_range(1..101)
// }

// fn get_user_number() -> Result<u8, Error> {
//     let string_input = read_user_input("Guess an integer between 1 and 100: ").unwrap();
//     match string_input.trim().parse::<u8>() {
//         Ok(value) if value >= 1 && value <= 100 => Ok(value),
//         Ok(_) => Err(Error::OutOfRange),
//         Err(e) => Err(Error::InvalidNumber(e)),
//     }
// }

// fn get_user_choice() -> String {
//     loop {
//         let choice = read_user_input("Would you like to play again? (y/n): ").unwrap();
//         if choice == "y" || choice == "n" {
//             return choice;
//         } else {
//             println!("Invalid input. Please enter 'y' or 'n'.");
//         }
//     }
// }

// fn read_user_input(prompt: &str) -> io::Result<String> {
//     let mut string_input = String::new();
//     print!("{}", prompt);
//     stdout().flush()?;
//     io::stdin().read_line(&mut string_input)?;
//     Ok(string_input.trim().to_string())
// }

// use rand::{thread_rng, Rng};
// use std::io::stdin;
// use std::io::{stdout, Write};

// fn main() {
//     loop {
//         // generate a random number
//         let random_number: u8 = generate_random_number();

//         // loop until they get it correct
//         loop {
//             // get the user input
//             match get_user_number() {
//                 Ok(user_number) => {
//                     if user_number < random_number {
//                         println!("Too low!");
//                     } else if user_number > random_number {
//                         println!("Too high!");
//                     } else {
//                         println!("You guessed the correct number!");
//                         break;
//                     }
//                 }
//                 Err(e) => println!("{}", e),
//             }
//         }
//         let user_choice = get_user_choice();
//         if user_choice != "y" {
//             break;
//         }
//     }
// }

// fn generate_random_number() -> u8 {
//     let mut rng = thread_rng();
//     return rng.gen_range(1..101);
// }

// fn get_user_number() -> Result<u8, String> {
//     let mut string_input = String::new();
//     print!("Guess an integer between 1 and 100: ");
//     stdout().flush().unwrap();
//     std::io::stdin().read_line(&mut string_input).unwrap();
//     let parsed_input = string_input.trim().parse::<u8>();
//     // check if the input is a valid number
//     // this is validation
//     match parsed_input {
//         Ok(value) => {
//             if value < 1 || value > 100 {
//                 return Err("Please enter an integer between 1 and 100.".to_string());
//             } else {
//                 return Ok(value);
//             }
//         }
//         Err(_) => {
//             return Err("Please enter an integer between 1 and 100.".to_string());
//         }
//     }
// }

// fn get_user_choice() -> String {
//     loop {
//         let mut string_input = String::new();
//         print!("Would you like to play again? (y/n): ");
//         stdout().flush().unwrap();
//         stdin().read_line(&mut string_input).unwrap();
//         let choice = string_input.trim().to_string();
//         if choice == "y" || choice == "n" {
//             return choice;
//         } else {
//             println!("Invalid input. Please enter 'y' or 'n'.");
//         }
//     }
// }

// use ctrlc;
// use rand::{thread_rng, Rng};
// use std::io::stdin;
// use std::io::{stdout, Write};
// use std::sync::atomic::{AtomicBool, Ordering};
// use std::sync::Arc;

// fn main() {
//     let running = Arc::new(AtomicBool::new(true));
//     let r = running.clone();
//     ctrlc::set_handler(move || {
//         r.store(false, Ordering::SeqCst);
//         println!("You cannot escape. Play the fucking game."); // Print message when Ctrl-C is pressed
//     })
//     .expect("Error setting Ctrl-C handler");

//     while running.load(Ordering::SeqCst) {
//         // generate a random number
//         let random_number: u8 = generate_random_number();

//         // loop until they get it correct
//         loop {
//             // get the user input
//             match get_user_number() {
//                 Ok(user_number) => {
//                     if user_number < random_number {
//                         println!("Too low!");
//                     } else if user_number > random_number {
//                         println!("Too high!");
//                     } else {
//                         println!("You guessed the correct number!");
//                         break;
//                     }
//                 }
//                 Err(e) => println!("{}", e),
//             }
//         }
//         let user_choice = get_user_choice();
//         if user_choice != "y" {
//             break;
//         }
//     }
// }

// fn generate_random_number() -> u8 {
//     let mut rng = thread_rng();
//     return rng.gen_range(1..101);
// }

// fn get_user_number() -> Result<u8, String> {
//     let mut string_input = String::new();
//     print!("Guess an integer between 1 and 100: ");
//     stdout().flush().unwrap();
//     std::io::stdin().read_line(&mut string_input).unwrap();
//     let parsed_input = string_input.trim().parse::<u8>();
//     // check if the input is a valid number
//     // this is validation
//     match parsed_input {
//         Ok(value) => {
//             if value < 1 || value > 100 {
//                 return Err("Please enter an integer between 1 and 100.".to_string());
//             } else {
//                 return Ok(value);
//             }
//         }
//         Err(_) => {
//             return Err("Please enter an integer between 1 and 100.".to_string());
//         }
//     }
// }

// fn get_user_choice() -> String {
//     loop {
//         let mut string_input = String::new();
//         print!("Would you like to play again? (y/n): ");
//         stdout().flush().unwrap();
//         stdin().read_line(&mut string_input).unwrap();
//         let choice = string_input.trim().to_string();
//         if choice == "y" || choice == "n" {
//             return choice;
//         } else {
//             println!("Invalid input. Please enter 'y' or 'n'.");
//         }
//     }
// }

// use std::io::Write;

// fn main() {
//     let mut string_input = String::new();
//     print!("Enter a string: ");
//     std::io::stdout().flush().unwrap();
//     std::io::stdin().read_line(&mut string_input).unwrap();
//     let mut trimmed_string = string_input.trim().to_string().push_str("hello");
//     println!("You entered: {}", trimmed_string);
// }

// fn main() {
//     let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];

//     let max = *numbers.iter().max().unwrap();
//     let min = *numbers.iter().min().unwrap();

//     let sum: i32 = numbers.iter().sum();
//     let mean = sum as f64 / numbers.len() as f64;

//     assert_eq!(max, 56);
//     assert_eq!(min, -18);
//     assert_eq!(mean, 12.5);
// }

// fn main() {
//     let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
//     let mut max: i32;
//     let mut min: i32;
//     let mean: f64;

//     max = numbers[0];
//     for n in numbers {
//         if n > max {
//             max = n;
//         };
//     }

//     min = numbers[0];
//     for n in numbers {
//         if n < min {
//             min = n;
//         };
//     }

//     let mut sum = 0;
//     for n in numbers {
//         sum += n;
//     }

//     mean = sum as f64 / numbers.len() as f64;

//     assert_eq!(max, 56);
//     assert_eq!(min, -18);
//     assert_eq!(mean, 12.5);
// }

// fn main() {
//     let mut numbers = [1, 2, 3, 4, 5];

//     for i in numbers.iter_mut() {
//         *i += 1;
//         println!("{}", &*i + 1);
//     }
// }
