#[cfg(test)]

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use anyhow::Context as _;

#[test]
fn test_python_version() -> PyResult<()> {
  /*
    import sys
    import pprint
    print(sys.version)
    pprint.pprint(sys.path)
  */
  Python::with_gil(|py| {
    let hw: String = py.eval("'python' + 'world'", None, None)?.extract()?;
    println!("Hello {}", hw);

    let sys = py.import("sys")?;
    let version: String = sys.getattr("version")?.extract()?;
    let paths: &PyAny = sys.getattr("path")?;

    println!("-------- version --------");
    println!("{version}");
    println!("-------- paths --------");
    println!("{paths:?}");

    Ok(())
  })
}


#[test]
fn test_python_args() -> PyResult<()> {
  let code = indoc::formatdoc!{r#"
    def func(*args, **kwargs):
      print('type = ',type(args))   # <class 'tuple'>
      print('type = ',type(kwargs)) # <class 'dict'>
      print('args = ', args)        # args =  (1, 3)
      print('kwargs = ', kwargs)    # kwargs = {{'b': 5, 'c': 3}}
    
    class A:
      def method(self, *args, **kwargs):
        assert args == ("hello",)
        assert kwargs == {{"cruel": "world"}}
        return "called with args and kwargs"
    a = A()
  "#};

  Python::with_gil(|py| {
    let module = PyModule::from_code(py, &code, "", "")?;

    let args = ("hello",);
    let kwargs = PyDict::new(py);
    kwargs.set_item("cruel", "world")?;
    let _kwargs2: &PyDict = py.eval( "{'a': 1, 'b': 'test'}", None, None)?.downcast()?;

    let func = module.getattr("func")?;
    let _ = func.call(args, Some(kwargs))?;
    // func.call0(py)?;
    // func.call1(py, args)?;

    let instance = module.getattr("a")?;
    let result = instance.call_method("method", args, Some(kwargs))?;
    let dst = result.extract::<&str>()?;
    println!("dst : {}", dst);

    // let dst = obj.get_item("a").unwrap().extract::<u32>().unwrap();


    Ok(())
  })
}


#[test]
fn test_python_py2rust() -> anyhow::Result<()> {
  Python::with_gil(|py| {
    let args = vec!["hello".into_py(py), 1.into_py(py), true.into_py(py)];
    let tuple = PyTuple::new(py,args);
    let kwargs = PyDict::new(py);
    kwargs.set_item("a", "a").context("")?;
    kwargs.set_item("b", 4.0).context("")?;

    println!("{:?}", tuple);
    println!("{:?}", kwargs);

    // json not merging for keep the order
    let json_tuple = serde_pyobject::from_pyobject::<serde_json::Value>(tuple).context("err")?;
    let json_kwargs = serde_pyobject::from_pyobject::<serde_json::Value>(kwargs).context("err")?;
    println!("{:?}", json_tuple);
    println!("{:?}", json_kwargs);

    // convert vec for clap
    let mut vec_tuple: Vec<String> = tuple.into_iter().map(|x| x.to_string() ).collect();
    let mut vec_kwargs: Vec<String> = kwargs.into_iter()
    .fold(Vec::new(), |mut acc, n| {
      acc.push(format!("--{}", n.0));
      acc.push(n.1.to_string());
      acc
    });
    vec_tuple.append(&mut vec_kwargs);
    println!("{:?}", vec_tuple);
    Ok(())
  })
}
