pub mod core;
pub use core::err::*;
use colored::Colorize;

// use once_cell::sync::Lazy;
// use std::sync::RwLock;
// use std::backtrace::Backtrace;

#[macro_export]
macro_rules! cprintln {
  ($a:expr, $b:expr) => { println!("{:>12} {}", $a.bold(), $b) };
  (blue : $a:expr, $b:expr) => { println!("{:>12} {}", $a.blue().bold(), $b) };
  (green : $a:expr, $b:expr) => { println!("{:>12} {}", $a.green().bold(), $b) };
  (red : $a:expr, $b:expr) => { println!("{:>12} {}", $a.red().bold(), $b) };
  (yellow : $a:expr, $b:expr) => { println!("{:>12} {}", $a.yellow().bold(), $b) };
}


/******** core ********/

pub fn hello_world() -> &'static str {
  match (cfg!(feature = "r-lang"), cfg!(feature = "python")) {
    (true, false) => "hello_rust!!! in r",
    (false, true) => "hello_rust!!! in python",
    _ => "hello_rust!!!"
  }
}

pub fn cprint(c:&str, a:&str, b:&str) {
  match c {
    "blue" => cprintln!(blue: a, b),
    "green" => cprintln!(green: a, b),
    "red" => cprintln!(red: a, b),
    "yellow" => cprintln!(yellow: a, b),
    _=> cprintln!(a, b),
  };
}

pub fn sum_usize(a:usize, b:usize) -> anyhow::Result<usize> { Ok(a + b) }
