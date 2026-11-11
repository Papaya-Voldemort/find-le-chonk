use console::Term;
use owo_colors::OwoColorize;
use std::io::Write;
use std::time::Duration;
use std::{io, thread};
use std::fmt::Display;
use owo_colors::DynColors::Rgb;

pub fn dot_with_wait(dot_count: u32, wait_between: u64) {
    print!("{}  ", "│".dimmed());
    io::stdout().flush().unwrap();

    for _ in 1..dot_count + 1 {
        print!(".");
        io::stdout().flush().unwrap();
        sleep(wait_between);
    }
}

pub fn sleep(time: u64) {
    thread::sleep(Duration::from_millis(time))
}

pub fn say(text: impl std::fmt::Display) {
    let bar = "│".dimmed();
    let text = text.to_string();

    let term_width = Term::stdout().size().1 as usize;
    let wrap_width = term_width.saturating_sub(5).max(20);

    for line in textwrap::wrap(&text, wrap_width) {
        println!("{bar}  {line}");
    }
}

pub fn blank_rail() {
    let bar = "│".dimmed();
    println!("{bar}");
}

pub fn cat() -> impl Display {
    "Le Chonk".color(Rgb(250, 82, 15))
}