extern crate proc_macro;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_derive(HelloMacro)]
pub fn derive_self_name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let item = parse_macro_input!(input as ItemStruct);

  // let parsed = parse_macro_input!(input as syn::DeriveInput);
  println!("{:?}", item);

  let struct_name = item.ident;
  let gen = quote! {

    impl #struct_name {
      pub fn a<'a>() -> &'a str {
        stringify!(#struct_name)
      }
    }

  };
  gen.into()
}

#[proc_macro_derive(HelloMacroPy)]
pub fn derive_self_name2(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let item = parse_macro_input!(input as ItemStruct);
  let struct_name = item.ident;
  let gen = quote! {

    #[cfg(feature="python")]
    use pyo3::prelude::*;
    #[cfg(feature="python")]
    use pyo3::types::{PyDict, PyTuple};

    #[cfg(feature="r-lang")]
    use extendr_api::*;

    impl #struct_name {
      #[cfg(feature="python")]
      pub fn from_clap_py(args: &pyo3::types::PyTuple, kwargs:Option<&pyo3::types::PyDict>) -> anyhow::Result<Self> {
        let mut vec_args: Vec<String> = args.into_iter().map(|x| x.to_string() ).collect();
        let mut vec_kwargs: Vec<String> = match kwargs {
          None => Vec::new(),
          Some(n) => n.into_iter().fold(Vec::new(), |mut acc, i| {
            acc.push(format!("--{}", i.0));
            acc.push(i.1.to_string());
            acc
          })
        };

        let mut dst = vec![stringify!(#struct_name).to_string()];
        dst.append(&mut vec_args);
        dst.append(&mut vec_kwargs);
        Self::from_clap_vec(dst)
      }

      #[cfg(feature="r-lang")]
      pub fn from_clap_robj(args: Robj) -> anyhow::Result<Self> {
        use extendr_api::prelude::*;
        let vec : Vec<(&str, Robj)> = args.as_list().unwrap().iter().collect();
        let dst : Vec<String> = vec.iter().fold(
          vec![stringify!(#struct_name).to_string()],
          |mut acc, n|{
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
          }
        );
        // for n in vec {
        //   match n.0 {
        //     "" => { },
        //     "NA" => { },
        //     _ => { vec_args.push(format!("--{}", n.0)) },
        //   }
      
        //   let m = R!("eval({{ n.1 }})").unwrap();
        //   match m.rtype() {
        //     Rtype::Rstr => vec_args.push(m.as_str().unwrap().to_string()),
        //     Rtype::Logicals => vec_args.push(m.as_bool().unwrap().to_string()),
        //     Rtype::Integers => vec_args.push(m.as_integer().unwrap().to_string()),
        //     Rtype::Doubles => vec_args.push(m.as_real().unwrap().to_string()),
        //     Rtype::Strings => vec_args.push(m.as_str().unwrap().to_string()),
        //     _=> panic!("Syntax error")
        //   };
        // }
        // let mut dst = vec![stringify!(#struct_name).to_string()];
        // dst.append(&mut vec_args);
        Self::from_clap_vec(dst)
      }

      pub fn from_clap_vec<I, T>(itr: I) -> anyhow::Result<Self>
      where I: IntoIterator<Item = T>, T: Into<std::ffi::OsString> + Clone, {
        use anyhow::Context as _;
        use clap::{CommandFactory, FromArgMatches};
        let am = Self::command().try_get_matches_from(itr).context("err")?;
        let dst = Self::from_arg_matches(&am).context("err")?;
        Ok(dst)
      }
    }

  };
  gen.into()
}

