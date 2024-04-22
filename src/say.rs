pub fn say(text: &str) {
    // first prioritise input text
    if !text.is_empty() {
        // if input text was provided
        let content = split_into_lines(text);
        print_speech_bubble(&content);
        crate::print::print(
            false,
            vec![&"regular".to_string()],
            true,
            vec![&"random".to_string()],
            vec![0],
            0.0,
            0,
        )
    } else {
        // if no input text was provided
        let buffer = read_from_stdin();
        let content = split_into_lines(buffer.as_str());
        print_speech_bubble(&content);
        crate::print::print(
            false,
            vec![&"regular".to_string()],
            true,
            vec![&"random".to_string()],
            vec![0],
            0.0,
            0,
        )
    }
}

fn read_from_stdin() -> String {
    use std::io::Read;
    let mut buffer = String::new();
    std::io::stdin()
        .read_to_string(&mut buffer)
        .expect("Failed to read from stdin");

    // trim newline character from end of buffer
    buffer.trim_end().to_string()
}

fn split_into_lines(input: &str) -> Vec<String> {
    input
        // splits the input string on newlines
        .lines()
        // converts each line to a string
        .map(|line| line.to_string())
        // collects all lines into a Vec<String>
        .collect()
}

fn print_speech_bubble(content: &[String]) {
    // determine the maximum length of the content lines, with a minimum of 24
    let mut max_length = content.iter().map(|line| line.len()).max().unwrap_or(0);
    max_length = max_length.max(24); // Ensure minimum width of 20

    // print the top border
    println!("+{}+", "-".repeat(max_length + 2));

    // print each line centered within the bubble
    for line in content.iter() {
        let padding = max_length - line.len();
        // padding to the left
        let left_padding = padding / 2;
        // padding to the right to balance out any odd number of padding spaces
        let right_padding = padding - left_padding;

        println!(
            "| {}{}{} |",
            " ".repeat(left_padding),
            line,
            " ".repeat(right_padding)
        );
    }

    // print the bottom border
    println!("+{}+", "-".repeat(max_length + 2));
    println!("                //");
    println!("               //");
}
