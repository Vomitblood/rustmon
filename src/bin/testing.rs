// fn main() {
//     let mut numbers = [1, 2, 3, 4, 5];

//     for i in numbers.iter_mut() {
//         *i += 1;
//         println!("{}", &*i + 1);
//     }
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
//     let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];

//     let max = *numbers.iter().max().unwrap();
//     let min = *numbers.iter().min().unwrap();

//     let sum: i32 = numbers.iter().sum();
//     let mean = sum as f64 / numbers.len() as f64;

//     assert_eq!(max, 56);
//     assert_eq!(min, -18);
//     assert_eq!(mean, 12.5);
// }

use std::io::Write;

fn main() {
    let mut string_input = String::new();
    print!("Enter a string: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut string_input).unwrap();
    let mut trimmed_string = string_input.trim().to_string().push_str("hello");
    println!("You entered: {}", trimmed_string);
}
