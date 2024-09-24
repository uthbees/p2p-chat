use std::io;
use std::io::Write;

/// A utility function to call the print! macro and flush the buffer to make sure the text is displayed.
pub fn print(str: &str) {
    print!("{str}");
    let _ = io::stdout().flush();
}
