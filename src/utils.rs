use std::io;
use std::io::Write;

/// A utility function to call the print! macro and flush the buffer to make sure the text is displayed.
pub fn print(str: &str) {
    print!("{str}");
    let _ = io::stdout().flush();
}

#[must_use]
/// A utility function to read a line of user input from the console.
pub fn read_line() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("should have been able to read line");
    String::from(line.trim())
}
