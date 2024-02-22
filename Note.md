# boilerplate

## instllation

- ```pip install```はgitでまるまるクローンしてるので特に工夫は必要ない
- ```remotes::install_github```は```./R```以下しかクローンしてない様子

**makefile**

```makefile
CHECK_FILE = ../../rust
ifeq ("$(wildcard $(CHECK_FILE))", "")
  FUATURES = github
else
  FUATURES = local
endif

# 中略

cargo build --target=$(TARGET) --lib --release --manifest-path=./rust/Cargo.toml --target-dir $(TARGET_DIR) --features $(FUATURES)
```

**Cargo.toml**

```toml
[features]
github = ["crate_github"]
local = ["crate_local"]

[dependencies]
crate_github = { optional = true, git = "https::/github/crate_entity", package = "crate_entity"}
crate_local = { optional = true, path = "../crate_entity", package = "crate_entity"}
```

**lib.rs**

```rust
#[cfg(feature = "local")]
use crate_local as crate_entity;

#[cfg(feature = "github")]
use crate_github as crate_entity;
```

以上でローカルインストールとgithubインストールのきり替えが記述できるが、Rのインストール時のみcargo installでエラー  
["Can not update registry #2605"](https://github.com/rust-lang/cargo/issues/2605)と同等の現象だと思うけど、ユーザー側に対処させるのはコストが合わないので不使用


## error handling

R, Pythonの場合、処理は継続するがwarningが存在する。

RはR!でtryCatchでキャッチできない。スタックされたのは後で表示される

```python
import boilerplatePyo
for key in ["normal", "error", "error_stack", "warning", "panic"]:
  try:
    print(f"-------- {key} --------")
    print(boilerplatePyo.panic(key))
  except Exception as e:
    print(f"Caught an exception: {e}")

# -------- normal --------
# normal
# -------- error --------
# Caught an exception: called bail
# -------- error_stack --------
# Caught an exception: called context
#
# Caused by:
#     called bail in function
# -------- warning --------
# .\boilerplate\tests\test2.py:14: DeprecationWarning: called warning
#   print(boilerplatePyo.panic(key))
# warning
# -------- panic --------
# thread '<unnamed>' panicked at .\boilerplate\rust\src/lib.rs:84:21:
# called panic in function
# note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
# Traceback (most recent call last):
#   File ".\boilerplate\tests\test.py", line 41, in <module>
#     print(boilerplatePyo.panic(key))
#           ^^^^^^^^^^^^^^^^^^^^^
# pyo3_runtime.PanicException: called panic in function
```

```r
f <- function(src) {
  switch (src,
    "panic" = stop("Panic!"),
    "warning" = warning("warning"),
    print("normal")
  )
}
for (i in c("panic","warning","normal")) {
  tryCatch({ 
      print(paste("-------- ", key, " --------"))
      f(i)
    },
    error = function(e) { message(paste("ERROR! ", e, sep = " ")) },
    warning = function(e) { message(paste("WARNING! ", e, sep = " ")) },
    finally = { message("finish.") },
    silent = TRUE
  )
}

# result :
#   ERROR!  Error in f(i): Panic!

#   finish.
#   WARNING!  simpleWarning in f(i): warning

#   finish.
#   [1] "normal"
#   finish.
```

## kwargs ellipses 

```cs
var dst = mdic.GetValueOrDefault("key", "default value");
```
```rust
let dst = match kwargs {
  Some()=> match get_item("a") {
  },
    .extract::<u32>().unwrap()
```

Rは遅延評価書けたりするのが面倒

```
Option<&PyDict> -> &PyDict -> Option<&PyAny> -> T
Robj -(重複は SyntaxError: keyword argument repeated: c)-> HashMap<&str,Robj> -> Option<Robj> -> T
```

```
// https://zenn.dev/jij_inc/articles/serde-pyobject
// https://haripo.com/articles/2019/pyo3
/*
f <- function(src) {
  switch (src,
    "panic" = stop("Panic!"),
    "warning" = warning("warning"),
    print("normal")
  )
}
for (i in c("panic","warning","normal")) {
  tryCatch(
    { f(i) },
    error = function(e) { message(paste("ERROR! ", e, sep = " ")) },
    warning = function(e) { message(paste("WARNING! ", e, sep = " ")) },
    finally = { message("finish.") },
    silent = TRUE
  )
}

result :
  ERROR!  Error in f(i): Panic!

  finish.
  WARNING!  simpleWarning in f(i): warning

  finish.
  [1] "normal"
  finish.
*/
```

## value

() -> 変換必要

| rust          | python / pyo3     | R                | note |
|:--:           | :--:              | :--:             | :--  |
| i64, etc      | int / PyLong      | Integers         | py:上限なし R:32ビット| 
| f64, etc      | float / PyFloat   | Doubles          | py:倍精度 R:倍精度    |
| bool          | bool / PyBool     | Logicals         | それぞれ true / True / TRUEなのでstring介したcast注意 |
| String        | string / PyString | Strings          | |
| tuple         | tuple / PyTuple   | (List)           | |
| Vec<>         | list / PyList     | List             | |
| HashMap       | dict / PyDict     | (Pairlist)       | Pairlist::from_pairs(&v.into_iter().collect::<Vec<_>>()) |
| strcut        | (PyAny, PyDict)   | (Pairlist, Robj) | *1, serde_pyobject::to_pyobject, #[pyo3::prelude::pyclass]   |
| Option/Result | (PyAny, PyResult) |  |  |


```rust
  // *1
  let a = serde_json::to_value(v).unwrap();
  let b = a.as_object().unwrap().iter().filter_map(|n|{
    if let Some(m) = n.1.as_i64() { return Some((n.0, r!(m))); }
    if let Some(m) = n.1.as_f64() { return Some((n.0, r!(m))); }
    if let Some(m) = n.1.as_str() { return Some((n.0, r!(m))); }
    if let Some(m) = n.1.as_bool() { return Some((n.0, r!(m))); }
    None
  }).collect::<Vec<_>>();
  Pairlist::from_pairs(&b)
```

### 命名規則参考

RはGoogle, tidyverse等あるがアバウト  
C#はt_, s_のprefixルールあるが、評判悪し  

|                         | C++           | C#              | Rust(RFC 430) | javascript    | R              |
| :--                     | :--           | :--             | :--           | :--           | :--            |
| const/static variable   | CONSTANT_NAME | ConstantName    | CONSTANT_NAME | CONSTANT_NAME | CONSTANT_NAME  |
|                         | kConstantName |                 |               |               | kConstantName  |
| variable/field/property | my_var        | PublicVar       | local_var     | publicVar     | local_var      |
|                         | strcut_member | this.privateVar |               | privateVar    |                |
|                         | class_member_ | _privateVar     |               | privateVar_   |                |
| method                  | my_function() | DoSometing      | do_something  | doSometing    | do_something   |
|                         |               |                 |               |               | doSometing     |
|                         |               | DoSometingAsync |               |               |                |
| argument                |               | argName         | arg_name      | argName       |                |
|                         |               |                 |               | opt_arg       |                |
|                         |               |                 |               | var_args      |                |
| class/struct/trait      | ClassName     | ClassName       | StrcutName    | ClassName     | class_name     |
|                         |               |                 |               |               | ClassName      |
|                         |               | IInterfaceName  |               |               |                |
|                         |               | InterfaceName   |               |               |                |
| enum                    | EnumName      | EnumName        | EnumName      | EnumName      |                |
| enum member/variant     | ENUM_VAR      | EnumMember      | EnumMember    | EnumMember    |                |
| component               |               |                 |               | MyComponent   |                |
| library/creat           |               |                 | crate_name    |               | rLibraryName   |
| file name               | my_src.cpp    | FeatureCategory | crate_name    | file-name     | file_name.R    |
|                         |               |                 | file-name     | filePath      | file-name.R    |
|                         |               |                 |               |               | 00_file_name.R |




