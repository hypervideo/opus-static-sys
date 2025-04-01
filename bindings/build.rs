use std::process::Command;
use std::path::PathBuf;

#[derive(Debug)]
struct ProcessComments;

impl bindgen::callbacks::ParseCallbacks for ProcessComments {
  fn process_comment(&self, comment: &str) -> Option<String> {
    match doxygen_bindgen::transform(comment) {
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
          unsafe {
            std::env::set_var("MACOSX_DEPLOYMENT_TARGET", target);
          }
        }
      }
    }
  }

  let dest = PathBuf::from("../opus").canonicalize().unwrap();

  let bindings = bindgen::Builder::default()
    .use_core()
    .merge_extern_blocks(true)
    .header(dest.join("include/opus.h").display().to_string())
    .allowlist_file(dest.join("include/opus.h").display().to_string())
    .allowlist_file(dest.join("include/opus_types.h").display().to_string())
    .allowlist_file(dest.join("include/opus_defines.h").display().to_string())
    .allowlist_file(dest.join("include/opus_multistream.h").display().to_string())
    .allowlist_file(dest.join("include/opus_projection.h").display().to_string())
    .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    .parse_callbacks(Box::new(ProcessComments))
    .layout_tests(false)
    .generate()
    .expect("Unable to generate bindings");
  bindings
    .write_to_file(dest.join("bindings.rs"))
    .expect("Couldn't write bindings.rs");

  std::fs::copy(dest.join("bindings.rs"), "../src/lib.rs")
    .expect("Couldn't copy bindings.rs to src/lib.rs");
}
