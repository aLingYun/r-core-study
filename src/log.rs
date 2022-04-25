use crate::println;

#[allow(unused)]
pub enum Color {
    Red,
    Yellow,
    Blue,
    Green,
    Gray,
}

pub fn puts_log(color: Color, s: &str) {
    match color {
        Color::Red     => println!("\x1b[31m[ERROR] {}\x1b[0m", s),
        Color::Yellow  => println!("\x1b[93m[WRAN]  {}\x1b[0m", s),
        Color::Blue    => println!("\x1b[34m[INFO]  {}\x1b[0m", s),
        Color::Green   => println!("\x1b[32m[DEBUG] {}\x1b[0m", s),
        Color::Gray    => println!("\x1b[90m[TRACE] {}\x1b[0m", s),
    }
}