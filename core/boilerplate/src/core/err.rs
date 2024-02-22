#![allow(dead_code)]

use anyhow::Context as _;
use thiserror::Error; // 式の評価が起きないので、contextよりwith_contextの方が速い
use colored::Colorize;

/******** 例外catch ********/


#[cfg(feature="r-lang")]
use extendr_api::Robj;

#[derive(Error, Debug)]
pub enum MyError<'a> {
  #[error("{}", format!("{} {}:{}",.0,.1,.2).red())]
  Panic(&'a str, &'a str, u32),

  #[error("{}", .0.yellow())]  
  Warning(String),
}

#[cfg(feature="python")]
macro_rules! warning {
  ($($arg:tt)*) => {
    pyo3::Python::with_gil(|py| { py.eval(format!("warnings.warn(f'''{}''', Warning, stacklevel = 2)", MyError::Warning(format!($($arg)*))).as_str(), None, None).unwrap(); });
  }
}
#[cfg(feature="r-lang")]
macro_rules! warning {
  ($($arg:tt)*) => {
    let _temp = format!("warning('{}')", MyError::Warning(format!($($arg)*)));
    extendr_api::prelude::r!("{{_temp}}");
  }
}
#[cfg(all(not(feature="python"), not(feature="r-lang")))]
macro_rules! warning {
  ($($arg:tt)*) => {
    println!("{:?}", MyError::Warning(format!($($arg)*)));
  }
}

pub fn panic(i: &str) -> anyhow::Result<&str> {
  fn call_panic() { panic!("{}", MyError::Panic("called panic in function", file!(), line!())); }
  fn call_bail() -> anyhow::Result<()> { anyhow::bail!("called bail in function"); }  
  match i {
    "panic" => { call_panic(); Ok(i) },
    "error" => anyhow::bail!("called bail"),
    "warning" => { warning!("called warning"); Ok(i) },
    "error_stack" => { call_bail().context("called context")?; Ok(i) }
    _ => Ok(i)
  }
}