use clap::{Parser, Subcommand, Args};
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
  #[command(about = "build C# from GitHub")]
  CS(CS),
  #[command(about = "build rust from GitHub")]
  RUST(RUST),
}

#[derive(Args, Debug)]
pub struct CS {
  #[arg(help = "github .git file url")]
  url: String,

  #[arg(short, long, help = "github .git file url")]
  subdirectory: Option<String>,

  #[arg(short, long, help = "output directory")]
  output: Option<String>,
}

impl CS {
  pub fn to_url(&self) -> String { 
    let buf = std::path::PathBuf::from(&self.url);
    buf.to_string_lossy().into_owned()
  }
  pub fn to_repogitory(&self) -> String { 
    let buf = std::path::PathBuf::from(&self.url);
    buf.file_stem().unwrap().to_string_lossy().into_owned()
  }
  pub fn to_build_path(&self) -> String { 
    let mut buf = std::path::PathBuf::from(r".\");
    buf = buf.join(std::path::PathBuf::from(&self.to_repogitory()));
    if let Some(n) = &self.subdirectory { 
      buf = buf.join(std::path::PathBuf::from(n)); 
    }
    buf.to_string_lossy().into_owned()
  }
  pub fn to_working_path(&self) -> String { 
    let mut buf = std::env::current_dir().unwrap();
    if let Some(n) = &self.output { 
      buf = buf.join(std::path::PathBuf::from(n));
    }  
    buf.to_string_lossy().into_owned()
  }
  pub fn run(&self) {
    
    match run_powershell(|path: String| {
      println!("{} {:?}", "arg".green(), self);
      let url = self.to_url();
      let _repogitory = self.to_repogitory();
      let sub = self.to_build_path();
      let out = self.to_working_path();
      let ps = indoc::formatdoc!(r#"
        cd "{path}"
        git clone {url}
        dotnet publish "{sub}" -c Release -o "{out}"
      "#);
      println!("{}", "generate ps1 script...".green());
      println!("{}", ps.blue());
      ps
    }){
      Ok(output) => { 
        println!("{}", output); 
        println!("{}", "finished.".green());  
      }
      Err(e) => { 
        println!("Error: {}", e); 
        println!("{}", "finished.".red());  
      }
    }
  
  }
}

#[derive(Args, Debug)]
pub struct RUST {
  #[arg(help = "github .git file url")]
  url: String,

  #[arg(short, long, help = "github .git file url")]
  subdirectory: Option<String>,

  #[arg(short, long, help = "output directory")]
  output: Option<String>,
}

impl RUST {
  pub fn to_url(&self) -> String { 
    let buf = std::path::PathBuf::from(&self.url);
    buf.to_string_lossy().into_owned()
  }
  pub fn to_repogitory(&self) -> String { 
    let buf = std::path::PathBuf::from(&self.url);
    buf.file_stem().unwrap().to_string_lossy().into_owned()
  }
  pub fn to_build_path(&self) -> String { 
    let mut buf = std::path::PathBuf::from(r".\");
    buf = buf.join(std::path::PathBuf::from(&self.to_repogitory()));
    if let Some(n) = &self.subdirectory { 
      buf = buf.join(std::path::PathBuf::from(n)); 
    }
    buf.to_string_lossy().into_owned()
  }
  pub fn to_working_path(&self) -> String { 
    let mut buf = std::env::current_dir().unwrap();
    if let Some(n) = &self.output { 
      buf = buf.join(std::path::PathBuf::from(n));
    }  
    buf.to_string_lossy().into_owned()
  }
  pub fn run(&self) {
    match run_powershell(|path: String| {
      println!("{} {:?}", "arg".green(), self);
      let url = self.to_url();
      let repogitory = self.to_repogitory();
      let sub = self.to_build_path();
      let out = self.to_working_path();
      let ps = indoc::formatdoc!(r#"
        cd "{path}"
        git clone {url}
        cd "{sub}"
        cargo build --release
      "#);
      println!("{}", "generate ps1 script...".green());
      println!("{}", ps.blue());
      ps
    }){
      Ok(output) => { 
        println!("{}", output); 
        println!("{}", "finished.".green());  
      }
      Err(e) => { 
        println!("Error: {}", e); 
        println!("{}", "finished.".red());  
      }
    }
  }
}

fn run_powershell<F>(func: F) -> anyhow::Result<std::process::ExitStatus> where F: FnOnce(String) -> String {
  use std::io::Write;
  
  let temp = tempfile::tempdir().unwrap();
  let path = temp.path().to_string_lossy().into_owned();
  let file_path = format!(r"{path}\_temp.ps1");

  println!("{} {}", "create temp dir".green(), path);
  println!("{} {}", "create temp file".green(), file_path);
  {
    let mut buffer = std::fs::File::create(&file_path).unwrap();
    write!(&mut buffer, "{}", func(path)).unwrap();  
  }

  println!("{}", "run powershell process...".green());
  let dst = std::process::Command::new("powershell")
    .args(&["-ExecutionPolicy", "Bypass", "-File", file_path.as_str()])
    .spawn().unwrap()
    .wait().unwrap();
  println!("{} {:?}", "cleaning up temp dir".green(), temp);
  // std::fs::Fileから解放、wait()で待つをしないとロック or リリースされてしまう

  Ok(dst)

  // let mut child = Command::new("powershell")
  // .args(&["-Command", "ls"])
  // .spawn()
  // .expect("failed")
  // .wait().unwrap();

}


fn main() {
  let args = Cli::parse();
  match args.command {
    Commands::CS(n) => { n.run(); }
    Commands::RUST(n) => { n.run(); }
  }
}
