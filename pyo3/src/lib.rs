use anyhow::Context as _;
use pyo3::prelude::*;
use pyo3::{Python, types::{PyAny, PyTuple, PyDict}};
use serde_json::json;
use colored::Colorize;

#[pyfunction]
fn hello_world() {
  println!("{}", boilerplate::hello_world());
  println!("hello_pyo3!!");
}

#[pyfunction]
fn sum_usize(a: usize, b: usize) -> anyhow::Result<usize> { boilerplate::sum_usize(a, b) }

#[pyfunction]
fn panic(key: &str) -> anyhow::Result<&str> { boilerplate::panic(key) }
// fn panic(key: &str) -> PyResult<&str> {  boilerplate::panic(key).map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(format!("{e:?}"))) }

#[pyfunction]
fn cprint(c: &str, a: &str, b: &PyAny) { boilerplate::cprint(c, a, &b.to_string().as_str()); }

#[pyfunction]
#[pyo3(signature = (*args, **kwargs))]
fn kwargs(args: &PyTuple, kwargs: Option<&PyDict>) -> anyhow::Result<()> {

  let a = json!({"f" : "b"});
  boilerplate::cprintln!(blue: "json", format!("{:?}",a));
  for a in a.as_object().unwrap() {
    boilerplate::cprintln!(blue: "json",
      match a.1 {
        serde_json::Value::String(n) => format!("{}", n),
        _=> format!("{}", a.1)
      });
  }

  boilerplate::cprintln!(blue: "args", format!("{:?}",&args));
  boilerplate::cprintln!(blue: "kwargs", format!("{:?}",&kwargs));
  // boilerplate::cprintln!(blue: "len", kwargs.unwrap_or(buf).len());

  let v_args = serde_pyobject::from_pyobject::<serde_json::Value>(args)?;
  let v_kwargs = serde_pyobject::from_pyobject::<serde_json::Value>(kwargs.context("err")?)?;
  boilerplate::cprintln!(blue: "args", format!("{:?}", v_args));
  boilerplate::cprintln!(blue: "kwargs", format!("{:?}", v_kwargs));

  match boilerplate::core::Cli::from_clap_py(args, kwargs) {
    Ok(n) => boilerplate::cprintln!(green: "to clap", format!("{:?}", n)),
    Err(e) => boilerplate::cprintln!(red: "to clap", format!("Failed : {}", e)),
  };

    // boilerplate::cprintln!(blue: "a", kwargs.unwrap().get_item("a").unwrap().unwrap().extract::<u32>().unwrap());
    // boilerplate::cprintln!(blue: "b", kwargs.unwrap().get_item("b").unwrap().unwrap().extract::<&str>().unwrap());

  Ok(())

}

/// A Python module implemented in Rust.
#[allow(non_snake_case)]
#[pymodule]
fn boilerplatePyo(_py: Python, m: &PyModule) -> PyResult<()> {
  m.add_function(wrap_pyfunction!(hello_world, m)?)?;
  m.add_function(wrap_pyfunction!(sum_usize, m)?)?;
  m.add_function(wrap_pyfunction!(cprint, m)?)?;
  m.add_function(wrap_pyfunction!(panic, m)?)?;
  m.add_function(wrap_pyfunction!(kwargs, m)?)?;
  Ok(())
}
