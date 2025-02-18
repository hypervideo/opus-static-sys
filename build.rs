extern crate bindgen;

use std::process::Command;

// TODO: REPLACE ALL OF THIS WITH doxygen-bindgen = { version = "0.1" } ONCE EDITION 2024 IS RELEASED
use std::error::Error;
use yap::{IntoTokens, Tokens};
const SEPS: [char; 5] = [' ', '\t', '\r', '\n', '['];
fn format_ref(str: String) -> String {
  if str.contains("://") {
    format!("[{str}]({str})")
  } else {
    format!("[`{str}`]")
  }
}
fn take_word(toks: &mut impl Tokens<Item = char>) -> String {
  toks
    .take_while(|&c| !SEPS.into_iter().any(|s| c == s))
    .collect::<String>()
}
fn skip_whitespace(toks: &mut impl Tokens<Item = char>) {
  toks.skip_while(|c| c.is_ascii_whitespace());
}
fn emit_section_header(output: &mut Vec<String>, header: &str) {
  if !output.iter().any(|line| line.trim() == header) {
    output.push(header.to_owned());
    output.push("\n\n".to_owned());
  }
}
fn transform(str: &str) -> Result<String, Box<dyn Error>> {
  let mut res: Vec<String> = vec![];
  let mut toks = str.into_tokens();

  skip_whitespace(&mut toks);
  while let Some(tok) = toks.next() {
    if "@\\".chars().any(|c| c == tok) {
      let tag = take_word(&mut toks);
      skip_whitespace(&mut toks);
      match tag.as_str() {
        "param" => {
          emit_section_header(&mut res, "# Arguments");
          let (mut argument, mut attributes) = (take_word(&mut toks), "".to_owned());
          if argument.is_empty() {
            if toks.next() != Some('[') {
              return Err("Expected opening '[' inside attribute list".into());
            }
            attributes = toks.take_while(|&c| c != ']').collect::<String>();
            if toks.next() != Some(']') {
              return Err("Expected closing ']' inside attribute list".into());
            }
            attributes = format!(" \\[{}\\] ", attributes);
            skip_whitespace(&mut toks);
            argument = take_word(&mut toks);
          }
          res.push(format!("* `{}`{} -", argument, attributes));
        }
        "retval" => res.push(format!("* `{}`", take_word(&mut toks))),
        "c" | "p" => res.push(format!("`{}`", take_word(&mut toks))),
        "ref" => res.push(format_ref(take_word(&mut toks))),
        "see" | "sa" => {
          emit_section_header(&mut res, "# See also");
          res.push(format!("> {}", format_ref(take_word(&mut toks))));
        }
        "a" | "e" | "em" => res.push(format!("_{}_", take_word(&mut toks))),
        "b" => res.push(format!("**{}**", take_word(&mut toks))),
        "note" => res.push("> **Note** ".to_owned()),
        "since" => res.push("> **Since** ".to_owned()),
        "deprecated" => res.push("> **Deprecated** ".to_owned()),
        "remark" | "remarks" => res.push("> ".to_owned()),
        "li" => res.push("- ".to_owned()),
        "par" => res.push("# ".to_owned()),
        "returns" | "return" | "result" => emit_section_header(&mut res, "# Returns"),
        "{" => { /* group start, not implemented  */ }
        "}" => { /* group end, not implemented */ }
        "brief" | "short" => {}
        _ => res.push(format!("{tok}{tag} ")),
      }
    } else if tok == '\n' {
      skip_whitespace(&mut toks);
      res.push(format!("{tok}"));
    } else {
      res.push(format!("{tok}"));
    }
  }
  Ok(res.join(""))
}

#[derive(Debug)]
struct ProcessComments;

impl bindgen::callbacks::ParseCallbacks for ProcessComments {
  fn process_comment(&self, comment: &str) -> Option<String> {
    match transform(comment) {
      Ok(res) => Some(res),
      Err(err) => {
        println!("cargo:warning=Problem processing doxygen comment: {comment}\n{err}");
        None
      }
    }
  }
}

fn main() {
  if cfg!(target_os = "macos") {
    if let Ok(output) = Command::new("rustc").args(&["--print", "deployment-target"]).output() {
      if output.status.success() {
        if let Some(target) = std::str::from_utf8(&output.stdout)
          .unwrap()
          .strip_prefix("deployment_target=")
          .map(|v| v.trim())
          .map(ToString::to_string)
        {
          std::env::set_var("MACOSX_DEPLOYMENT_TARGET", target);
        }
      }
    }
  }

  let dest = cmake::Config::new("opus")
    .profile("Release")
    .define("OPUS_BUILD_TESTING", "OFF")
    .define("OPUS_BUILD_SHARED_LIBRARY", "OFF")
    .define("OPUS_BUILD_PROGRAMS", "OFF")
    .define("OPUS_ENABLE_FLOAT_API", "ON")
    .define("OPUS_INSTALL_PKG_CONFIG_MODULE", "ON")
    .define("OPUS_INSTALL_CMAKE_CONFIG_MODULE", "ON")
    .define("CMAKE_INTERPROCEDURAL_OPTIMIZATION", "TRUE")
    .build();
  println!("cargo:root={}", dest.display());
  println!("cargo:include={}/include", dest.display());
  println!("cargo:lib_path={}/lib", dest.display());
  println!("cargo:lib={}/lib/libopus.a", dest.display());
  println!("cargo:rustc-link-search=native={}/lib", dest.display());
  println!("cargo:rustc-link-lib=static=opus");

  let bindings = bindgen::Builder::default()
    .use_core()
    .header(dest.join("include/opus/opus.h").display().to_string())
    .allowlist_file(dest.join("include/opus/opus.h").display().to_string())
    .allowlist_file(dest.join("include/opus/opus_types.h").display().to_string())
    .allowlist_file(dest.join("include/opus/opus_defines.h").display().to_string())
    .allowlist_file(dest.join("include/opus/opus_multistream.h").display().to_string())
    .allowlist_file(dest.join("include/opus/opus_projection.h").display().to_string())
    .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    .parse_callbacks(Box::new(ProcessComments))
    .generate()
    .expect("Unable to generate bindings");
  bindings
    .write_to_file(dest.join("bindings.rs"))
    .expect("Couldn't write bindings.rs");
}
