use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::terminal::is_raw_mode_enabled;
use std::io;
use std::io::Write;
use std::time::Duration;

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

#[must_use]
/// A utility function to read a line of user input from the console if it exists without blocking.
pub fn non_blocking_read_line() -> Option<String> {
    assert!(
        !is_raw_mode_enabled().expect("should be able to check if raw mode is enabled"),
        "raw mode should not be enabled"
    );

    match poll(Duration::from_secs(0)) {
        Err(err) => {
            panic!("failed to poll user input: {err}")
        }
        Ok(false) => None,
        Ok(true) => {
            // Since we know we're not in raw mode, we know that key events won't be available
            // here until the whole line is sent at once when the enter key is pressed. So we
            // continue reading until we get a line break.
            let mut line = String::new();

            loop {
                if let Event::Key(key) = read().expect("failed to get user input") {
                    match key.code {
                        KeyCode::Enter => return Some(line),
                        KeyCode::Char(char) => line.push(char),
                        _ => {}
                    }
                }
            }
        }
    }
}
