use io::{stdout, Write};
use std::io;
// accordion style
pub fn update_progress_bar(current_position: u64, total_size: u64) {
    let current_perc = current_position / (total_size / 100);
    let mut i = 0;
    let mut status_bar = "".to_string();
    while i <= current_perc {
        i = i + 1;
        if i % 5 == 0 {
            status_bar = format!("{}#", status_bar);
        }
    }

    i = current_perc;
    let mut empty_space = "".to_string();
    while i <= 100 - current_perc {
        i = i + 1;
        if i % 5 == 0 {
            empty_space = format!("{} ", empty_space);
        }
    }
    print!("\r                                                        ");
    stdout().flush().unwrap();
    print!(
        "\r    [{}{}] -> {}",
        status_bar,
        empty_space,
        format!("{}%", current_perc)
    );
    stdout().flush().unwrap();
}
