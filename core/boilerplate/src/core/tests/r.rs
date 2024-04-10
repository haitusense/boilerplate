#[cfg(test)]

const R_PATH: &str = r"C:\Program Files\R\R-4.3.2\bin\x64\";

fn call_r(code:&str) {
  use std::io::Write;

  let tempfile = tempfile::Builder::new().suffix(".r").tempfile().unwrap();
  let file_name = tempfile.path().to_string_lossy();
  println!("created : {:?}", file_name);

  let mut buf_writer = std::io::BufWriter::new(std::fs::File::create(&tempfile).unwrap());
  buf_writer.write(code.as_bytes()).unwrap();
  buf_writer.flush().unwrap();
  println!("written");
  let ps = format!(r#"&"{R_PATH}\Rscript.exe" {file_name}"#);
  println!("-------- ps start --------");
  match powershell_script::run(ps.as_str()) {
    Ok(output) => println!("{}", output),
    Err(e) => println!("Error: {}", e)
  };
  println!("-------- finished --------");
}

#[test]
fn test_r_version() {
  let code = indoc::formatdoc! {r#"
    R.version
    getwd()
  "#};
  call_r(code.as_str());
}

#[test]
fn test_r_install() {
  let code = indoc::formatdoc! {r#"
    install.packages(paste(getwd(), "/../../R", sep = ""), repos = NULL, type = "source")
  "#};
  call_r(code.as_str());
}

#[test]
fn test_r_uninstall() {
  let code = indoc::formatdoc! {r#"
    remove.packages("boilerplateR")
  "#};
  call_r(code.as_str());
}

#[test]
fn test_r_hello_world() {
  let code = indoc::formatdoc! {r#"
    boilerplateR::hello_world()
    # remove.packages("boilerplateR")
  "#};
  call_r(code.as_str());
}