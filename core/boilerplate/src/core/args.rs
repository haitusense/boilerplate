


// #[cfg(feature="r-lang")]
// pub fn kwargs_to_json(args: Robj) -> serde_json::Value {
//   let vec : Vec<(&str, Robj)> = args.as_list().unwrap().iter().collect();
//   let mut json = json!(null);
//   for n in vec {
//     let m = R!("eval({{ n.1 }})").unwrap();
//     if None == json.get(n.0) {
//       match m.rtype() {
//         Rtype::Rstr => json[n.0] = json!(m.as_str().unwrap()),
//         Rtype::Logicals => json[n.0] = json!(m.as_bool().unwrap()),
//         Rtype::Integers => json[n.0] = json!(m.as_integer().unwrap()),
//         Rtype::Doubles => json[n.0] = json!(m.as_real().unwrap()),
//         Rtype::Strings => json[n.0] = json!(m.as_str().unwrap()),
//         _=> {}
//       };
//     }
//   }
//   println!("{:?}", json);
//   json
// }


// macro_rules! var_args {
//   (fwd => $args:expr) => { match $args.as_list() {
//     Some(n) => n.iter().fold(HashMap::new(),|mut acc, n| {
//       

//       acc
//     }),
//     None => HashMap::new(),
//   }};
//   (rev => $args:expr) => { match $args.as_list() {
//     Some(n) => n.iter().fold(HashMap::new(),|mut acc, n| {
//       
//       acc
//     }),
//     None => HashMap::new(),
//   }};
//   (robj => $args:expr, $key:expr) => { match $args.as_list() {
//     Some(n) => match n.iter().find(|s| s.0 == $key) { // rpositionはエラー出る
//       Some(m) => m.1,
//       None => r!(NULL),
//     },
//     None => r!(NULL),
//   }};
//   (r => $args:expr, $key:expr) => { R!("eval(parse(text={{ &args }}['$key']))") };
// }

/*

pub enum RobjType<'a> {
  Bool(bool),
  Interger(i32),
  Real(f64),
  Str(&'a str),
  List(Robj),
  Function(Function),
  Unknown(Robj)
}

pub fn check_robj<'a>(src: Robj) -> RobjType<'a> {
  // https://extendr.github.io/extendr/extendr_api/robj/struct.Robj.html
  // https://extendr.github.io/extendr/extendr_api/robj/trait.Types.html

  if let Some(n) = src.as_bool() { return RobjType::Bool(n); }         // Option<bool>
  if let Some(n) = src.as_integer() { return RobjType::Interger(n); }  // Option<i32>
  if let Some(n) = src.as_real() { return RobjType::Real(n); }         // Option<f64>
  if let Some(n) = src.as_str() { return RobjType::Str(n); }           // Option<&'a str>
  // if let Some(n) = val.as_f64() { println!("f64 is {}", n) }
  // if let Some(n) = val.as_i64() { println!("i64 is {}", n) }  println!("err");

  if src.rtype() == Rtype::List {                                       // Rtype::List
    println!("list");
    // R!("print(head( {{ robj }} ))").unwrap();
    let names : Vec<_> = src.names().unwrap().collect();
    let dst = format!("{:?}", names);
    R!("print( {{ dst }} )").unwrap();
    return RobjType::Unknown(src);
  }

  if let Some(f) = src.as_function() { return RobjType::Function(f); }  // Option<Function>
    // let f = R!("function(x){ x + 1 }").unwrap();
    // call!("robj2", robj1).unwrap()
    // R!(robj2(robj1)).unwrap()

  return RobjType::Unknown(src);

}

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum VarArgsErr {
  #[error("failed args to list")]
	NotList,
  
  #[error("NoConnectionErr")]
	CommunicationError ,

  #[error("Timeout")]
  Timeout,
}
// let dst = names_and_values.iter().find(|&s| s.0 == key).context("not found key in args")?;
// Ok(dst.1.as_real().context("not found key in args")?)

pub fn check_var_args(args: Robj, rev: bool) -> Result<f64> {
  let names_and_values : Vec<_> = args.as_list().context(VarArgsErr::NotList)?.iter().collect();
  println!("{:>12} {:?}", "to vec".green().bold(), names_and_values);

  let first_a = names_and_values.iter().find(|&s| s.0 == "a").unwrap();
  let last_a = names_and_values.iter().rev().find(|&s| s.0 == "a").unwrap();
  println!("{:>12} {:?}", "first a".blue().bold(), first_a);
  println!("{:>12} {:?}", "last a".blue().bold(), last_a);

  let key = "a";
  let dst = names_and_values.iter().find(|&s| s.0 == key).context("not found key in args")?;
  Ok(dst.1.as_real().context("not found key in args")?)
}


pub fn var_args(args: Robj, rev: bool) -> Option<Robj> {

  let names_and_values : Vec<(&str, Robj)> = match args.as_list() {
    Some(n) => n,
    None => return None
  }.iter().collect();

  // let dst = names_and_values.iter().find(|&s| s.0 == key).context("not found key in args")?;
  let dst = match rev {
    true => names_and_values.iter().rev().find(|&s| s.0 == "a"),
    false => names_and_values.iter().find(|&s| s.0 == "a"),
  };

  match dst {
    Some(n) => Some(n.1),
    None => None
  }

}

  // RObj -> Option<&'a str>
  if let Some(n) = value.as_str() {
    send_string = n.to_string();
  }
  // Rtype::List
  if value.rtype() == Rtype::List {
    let robj = R!("jsonlite::toJSON({{ value }})").unwrap();
    send_string = robj.as_str().unwrap().to_string();
  }



*/




/* Rany<'a>の方がいいかな
pub enum Rtype {
  Null,        // NILSXP
  Symbol,      // SYMSXP
  Pairlist,    // LISTSXP
  Function,    // CLOSXP
  Environment, // ENVSXP
  Promise,     // PROMSXP
  Language,    // LANGSXP
  Special,     // SPECIALSXP
  Builtin,     // BUILTINSXP
  Rstr,        // CHARSXP
  Logicals,    // LGLSXP
  Integers,    // INTSXP
  Doubles,     // REALSXP
  Complexes,   // CPLXSXP
  Strings,     // STRSXP
  Dot,         // DOTSXP
  Any,         // ANYSXP
  List,        // VECSXP
  Expressions, // EXPRSXP
  Bytecode,    // BCODESXP
  ExternalPtr, // EXTPTRSXP
  WeakRef,     // WEAKREFSXP
  Raw,         // RAWSXP
  S4,          // S4SXP
  Unknown,
}
*/
