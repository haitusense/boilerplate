mod core;
use clap::Parser;
use core::Cli;

fn main() {
  let args = Cli::parse();
  println!("args {:?}", args);

  println!("args To Value {:?}", args.to_value());
}