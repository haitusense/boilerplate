use clap::Parser;

fn main() {
  let args = boilerplate::core::Cli::parse();
  println!("args {:?}", args);
  println!("args To Value {:?}", args.to_value());
}