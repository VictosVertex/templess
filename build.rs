fn build_libraries(path: &str) {
    use cmake::Config;
    Config::new(path)
            .very_verbose(true)
            .define("CLINGO_BUILD_SHARED", "OFF")
            .define("CLINGO_BUILD_STATIC", "ON")
            .define("CLINGO_MANAGE_RPATH", "OFF")
            .define("CLINGO_BUILD_WITH_PYTHON", "OFF")
            .define("CLINGO_BUILD_WITH_LUA", "OFF")
            .define("CLINGO_INSTALL_LIB", "ON")
            .define("CLINGO_BUILD_APPS", "OFF")
            .define("CLASP_BUILD_APP", "OFF")
            .build();
}

fn link_libraries(manifest_dir: &str) {
    println!("cargo:rustc-link-search=native={}/lib", manifest_dir);
    println!("cargo:rustc-link-lib=static=clingo");
    println!("cargo:rustc-link-lib=static=reify");
    println!("cargo:rustc-link-lib=static=potassco");
    println!("cargo:rustc-link-lib=static=clasp");
    println!("cargo:rustc-link-lib=static=gringo");
    println!("cargo:rustc-link-lib=dylib=msvcrt");
}

fn generate_bindings(manifest_dir: &str) {
    let bindings = bindgen::Builder::default()
        .header("include/clingo.h")
        .no_copy("clingo_model")
        .no_copy("clingo_solve_handle")
        .no_copy("clingo_control")
        .no_copy("clingo_options")
        .allowlist_type("clingo_control.*")
        .allowlist_type("clingo_options.*")
        .allowlist_type("clingo_model.*")
        .allowlist_type("clingo_solve_handle.*")
        .allowlist_type("clingo_symbol.*")
        .allowlist_type("clingo_error.*")
        .allowlist_function("clingo_control_.*")
        .allowlist_function("clingo_solve_handle_.*")
        .allowlist_function("clingo_model_.*")
        .allowlist_function("clingo_configuration_.*")
        .allowlist_function("clingo_version")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(format!("{}/src/clingo/bindings.rs", manifest_dir))
        .expect("Couldn't write bindings!");
}

fn println_warning(message: &str) {
    println!("cargo::warning={}", message);
}

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_libs = std::env::var("TEMPLESS_BUILD_LIBS").is_ok();
    let build_bindings = std::env::var("TEMPLESS_BUILD_BINDINGS").is_ok();
    println_warning(&format!(
        "Building clingo with build_libs={} and build_bindings={}",
        build_libs, build_bindings
    ));

    if build_libs {
        println_warning("Building clingo libraries...");
        use std::path::Path;
        
        let path = std::env::var("TEMPLESS_BUILD_LIBS").unwrap();
        let path = Path::new(&path);

        if !path.exists() || !path.is_dir() {
            panic!("Clingo source directory '{}' does not exist or is not a directory", path.display());
        }

        build_libraries(path.to_str().expect("Invalid UTF-8 in path"));
    }

    if build_bindings {
        println_warning("Generating bindings for clingo...");

        generate_bindings(&manifest_dir);
    }

    println_warning("Linking clingo libraries...");
    link_libraries(&manifest_dir);
}