#[cfg(test)]



// module_path!()
// include!
// duct
// duct_sh
// dimensioned
// indoc
// human_panic

#[test]
fn another() {
  use std::io::Write;

  let tempfile = tempfile::Builder::new().suffix(".py").tempfile().unwrap();
  let file_name = tempfile.path().to_string_lossy(); // .and_then(std::ffi::OsStr::to_str).unwrap()
  println!("created : {:?}", file_name);

  let mut buf_writer = std::io::BufWriter::new(std::fs::File::create(&tempfile).unwrap());
  buf_writer.write("print(1 + 1)".as_bytes()).unwrap();
  buf_writer.flush().unwrap();
  println!("written : {:?}", file_name);

  println!("-------- ps start --------");
  let code = indoc::formatdoc!(r#"
    py -3.12 {file_name}
  "#);
  let ps = powershell_script::PsScriptBuilder::new()
    .no_profile(true)
    .non_interactive(true)
    .hidden(false)
    .print_commands(true)
    .build();
  // match powershell_script::run(ps.as_str()) {
  match ps.run(code.as_str()) {
    Ok(output) => println!("{}", output),
    Err(e) => println!("Error: {}", e)
  };
  println!("-------- finished --------");
}

