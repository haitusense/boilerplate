use extendr_api::prelude::*;
use extendr_api::Rany;
use anyhow::{Result, bail};
use anyhow::Context as _;
use thiserror::Error;
use colored::Colorize;
use serde_json::*;

use std::collections::HashMap;

// #[cfg(feature = "local")]
// use boilerplate_local as boilerplate;

// #[cfg(feature = "github")]
// use boilerplate_local as boilerplate; 

macro_rules! kwargs {
  (hash:$args:expr) => { Ok(match $args.as_list() {
    None => HashMap::new(),
    Some(n) => n.iter().fold(HashMap::new(), |mut acc, n| {
      // fwd : acc.entry(n.0).or_insert(n.1);
      // rev : acc.insert(n.0, n.1);
      if acc.contains_key(n.0) {
        acc.insert(n.0,n.1);
      } else {
        bail!("SyntaxError: keyword argument repeated");
      }
      acc
    })
  }) };
  (json:$args:expr) => { match $args.as_list() {
    None => json!(null),
    Some(n) => n.iter().fold(json!(null), |mut acc, n| {
      if None == acc.get(n.0) { acc[n.0] = json!(r!(n.1)); } 
      acc
    })
  }};
}



/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
  println!("{}", boilerplate::hello_world());
  println!("hello_r!!");
  boilerplate::hello_world()
}

#[extendr]
fn panic(src: &str) -> Robj { boilerplate::panic(src).into() }

#[extendr]
fn kwargs(args: Robj) -> Robj {
  boilerplate::cprintln!(blue: "args", format!("{:?}", args));

  let vec : Vec<(&str, Robj)> = args.as_list().unwrap().iter().collect();
  boilerplate::cprintln!(blue: "args", format!("{:?}", vec)); 
  
  let dst : Vec<String> = vec.iter().fold(Vec::new(), |mut acc, n|{
    match n.0 {
      "" => { },
      "NA" => { },
      _ => { acc.push(format!("--{}", n.0)) },
    };
    let m = R!("eval({{ &n.1 }})").unwrap();
    match m.rtype() {
      Rtype::Rstr => acc.push(m.as_str().unwrap().to_string()),
      Rtype::Logicals => acc.push(m.as_bool().unwrap().to_string()),
      Rtype::Integers => acc.push(m.as_integer().unwrap().to_string()),
      Rtype::Doubles => acc.push(m.as_real().unwrap().to_string()),
      Rtype::Strings => acc.push(m.as_str().unwrap().to_string()),
      _=> panic!("Syntax error")
    };
    acc
  });
  boilerplate::cprintln!(blue: "vec", format!("{:?}", dst));

  match boilerplate::core::Cli::from_clap_robj(args) {
    Ok(n) => boilerplate::cprintln!(green: "to clap", format!("{:?}", n)),
    Err(e) => boilerplate::cprintln!(red: "to clap", format!("Failed : {}", e)),
  };
  // println!("{:>12} {:?}", "dst_robj".blue().bold(), dst_robj.as_any());
  // println!("{:>12} {:?}", "hash_fwd".blue().bold(), hash_fwd.get(&"a").unwrap_or(& r!(NULL)).as_any());
  // println!("{:>12} {:?}", "hash_rev".blue().bold(), hash_rev.get(&"a").unwrap_or(& r!(NULL)).as_any());
  // println!("{:>12} {:?}", "dst_r".blue().bold(), dst_r);

  // println!("a = {}", args.to_real("a").unwrap_or(-1f64) );
  // let dst = R!("eval(parse(text={{ args }}['a']))").unwrap_or((-2f64).into());
  // println!("a = {:?}", dst);

  // println!("b = {}", args.to_char("b").unwrap_or("na") );

  // let names_and_values : Vec<_> = args.as_list().unwrap().iter().collect();
  // let dst1 = names_and_values.iter().find(|&s| s.0 == "x").unwrap();
  // let dst1 = &dst1.1;
  // let dst2 = names_and_values.iter().find(|&s| s.0 == "y").unwrap();
  // let dst2 = &dst2.1;
  // let x = R!("eval({{ dst1 }})[[ {{ dst2 }} ]]").unwrap();
  // println!("x = {:?}", x);

  Robj::from(0)
}




#[extendr]
fn ex_robj_types(val: Robj, arg: Robj) -> Robj {
  println!("{:>12} {:?}", "var".blue().bold(), val);
  println!("{:>12} {:?}", "type".blue().bold(), val.rtype());
  println!("{:>12} {:?}", "type".blue().bold(), val.as_any());
  println!("{:>12} {:?}", "argument".blue().bold(), arg);
  // match check_robj(val) {
  //   RobjType::Bool(n) => println!("{:>12} {}", "Bool".green().bold(), n),
  //   RobjType::Interger(n) => println!("{:>12} {}", "Interger".green().bold(), n),
  //   RobjType::Real(n) => println!("{:>12} {}", "Real".green().bold(), n),
  //   RobjType::Str(n) => println!("{:>12} {}", "Str".green().bold(), n),
  //   RobjType::Function(f) => {
  //     let dst = f.call(pairlist!(arg.clone())).unwrap();
  //     println!("{:>12} {:?}", "Func".green().bold(), f);
  //     println!("{:>12} {:?}", "args".green().bold(), arg);
  //     println!("{:>12} {:?}", "dst".green().bold(), dst);
  //   },
  //   _=> println!("unknown")
  // };
  Robj::from(1)
}




pub trait RobjArgs {
  fn to_real(&self, key: &str) -> anyhow::Result<f64>;
  fn to_char(&self, key: &str) -> anyhow::Result<&str>;
  fn to_bool(&self, key: &str) -> anyhow::Result<bool>;
  // fn to_robj(&self, key: &str) -> anyhow::Result<&Robj>;
}

impl RobjArgs for Robj {
  fn to_real(&self, key: &str) -> anyhow::Result<f64> {
    let names_and_values : Vec<(&str, Robj)> = self.as_list().context("failed args to list")?.iter().collect();
    let dst = names_and_values.iter().find(|&s| s.0 == key).context("not found key in args")?;
    Ok(dst.1.as_real().context("not found key in args")?)
  }
  fn to_char(&self, key: &str) -> anyhow::Result<&str> {
    let names_and_values : Vec<(&str, Robj)> = self.as_list().context("failed args to list")?.iter().collect();
    let dst = names_and_values.iter().find(|&s| s.0 == key).context("not found key in args")?;
    Ok(dst.1.as_str().context("not found key in args")?)
  }
  fn to_bool(&self, key: &str) -> anyhow::Result<bool> {
    let names_and_values : Vec<(&str, Robj)> = self.as_list().context("failed args to list")?.iter().collect();
    let dst = names_and_values.iter().find(|&s| s.0 == key).context("not found key in args")?;
    Ok(dst.1.as_bool().context("not found key in args")?)
  }
  // fn to_robj(&self, key: &str) -> anyhow::Result<&Robj> {
  //   let names_and_values : Vec<(&str, Robj)> = self.as_list().context("failed args to list")?.iter().collect();
  //   let dst = names_and_values.iter().find(|&s| s.0 == key).context("not found key in args")?;
  //   Ok(dst.1)
  // }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
  mod boilerplateR;
  fn hello_world;
  fn ex_robj_types;
  fn kwargs;
  fn panic;
}
