pub mod err;
pub mod args;
pub mod tests;
use anyhow::Context as _;

#[derive(argsproc::HelloMacroPy, clap::Parser, serde::Serialize, serde::Deserialize, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {

  #[arg(num_args(0..))] 
  pub commands: Option<Vec<String>>,

  #[arg(short, long, default_value_t = 1)]
  a: u32,

  #[arg(short, long)]
  b: Option<String>,

}

impl Cli {
  pub fn to_value(&self) -> anyhow::Result<serde_json::Value> {
    Ok(serde_json::to_value(&self).context("err")?)
  }
}

