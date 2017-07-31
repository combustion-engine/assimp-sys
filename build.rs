extern crate cmake;

use cmake::Config;

macro_rules! options {
    ($config:ident, $($opt:tt = $value:tt,)*) => {
        $( $config.define(stringify!($opt), stringify!($value)); )*
    }
}

fn main() {
    let mut config = Config::new(".");

    options!(config,
        BUILD_SHARED_LIBS           = OFF,
        ASSIMP_OPT_BUILD_PACKAGES   = OFF,
        ASSIMP_ANDROID_JNIIOSYSTEM  = OFF,
        ASSIMP_NO_EXPORT            = OFF,
        ASSIMP_BUILD_ZLIB           = OFF,
        ASSIMP_BUILD_ASSIMP_TOOLS   = OFF,
        ASSIMP_BUILD_ASSIMP_VIEW    = OFF,
        ASSIMP_BUILD_SAMPLES        = OFF,
        ASSIMP_BUILD_TESTS          = OFF,
        ASSIMP_COVERALLS            = OFF,
        SYSTEM_IRRXML               = OFF,
        BUILD_DOCS                  = OFF,
        ASSIMP_DOUBLE_PRECISION     = OFF,
        ASSIMP_INSTALL_PDB          = OFF,
    );

    if cfg!(feature = "double_precision") {
        options!(config,
            ASSIMP_DOUBLE_PRECISION = ON,
        );
    }

    if cfg!(debug_assertions) {
        options!(config,
            ASSIMP_INSTALL_PDB      = ON,
        );
    }

    let dst = config.build_target("assimp").build();

    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=dylib=assimp");
}
