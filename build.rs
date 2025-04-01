use std::process::Command;

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
}
