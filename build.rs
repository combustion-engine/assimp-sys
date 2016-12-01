extern crate cmake;
use cmake::Config;

#[cfg(not(feature = "double_precision"))]
fn config_precision(config: &mut Config) {
    config.define("ASSIMP_DOUBLE_PRECISION", "OFF");
}

#[cfg(feature = "double_precision")]
fn config_precision(config: &mut Config) {
    config.define("ASSIMP_DOUBLE_PRECISION", "ON");
}

fn main() {
    let mut config = Config::new(".");

    config.define("BUILD_SHARED_LIBS", "OFF");
    config.define("ASSIMP_BUILD_TESTS", "OFF");
    config.define("ASSIMP_BUILD_ASSIMP_TOOLS", "OFF");

    config_precision(&mut config);

    let dst = config.build();

    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=dylib=assimp");
}
