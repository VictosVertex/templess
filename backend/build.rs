#![allow(dead_code)]
//! Build script for the clingo Rust bindings.

/// Prints a warning message to Cargo.
///
/// This is used because cargo warnings are annoying.
///
/// # Parameters
/// - `message`: The warning message to print.
fn println_warning(message: &str) {
    println!("cargo::warning={message}");
}

/// Links the clingo libraries to the Rust project.
///
/// # Parameters
/// - `manifest_dir`: The directory of the Cargo manifest.
fn link_libraries(manifest_dir: &str) {
    println_warning("Linking clingo libraries...");
    println!("cargo:rustc-link-search=native={manifest_dir}/lib");
    println!("cargo:rustc-link-lib=static=clingo");
    println!("cargo:rustc-link-lib=static=reify");
    println!("cargo:rustc-link-lib=static=potassco");
    println!("cargo:rustc-link-lib=static=clasp");
    println!("cargo:rustc-link-lib=static=gringo");
}

/// Module for bundling and building clingo from source.
///
/// This module handles cloning the clingo repository, applying necessary patches,
/// building the libraries, copying them to the project, and generating Rust bindings.
///
/// It is only compiled if the `bundle-clingo` feature is enabled.
mod bundle {
    use super::*;
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::{env, fs};

    /// Applies a manual patch to clingo's CMakeLists.txt to include wide_integer in the build.
    ///
    /// This is done because the original CMakeLists.txt does not include wide_integer by default,
    /// which is necessary for compilation. The patch wasn't applied in clingo, probably because currently
    /// clingo 6 is in development which may handle this differently.
    ///
    /// # Parameters
    /// - `clingo_source_path`: The path to the cloned clingo source code.
    fn patch_clingo_manually(clingo_source_path: &Path) {
        println_warning("Manually patching clingo's CMakeLists.txt...");

        let target_file_path = clingo_source_path.join("libclingo").join("CMakeLists.txt");

        const LINE_TO_FIND: &str = "list(APPEND clingo_targets libgringo libreify ordered_map hopscotch_map sparse_map optional variant)";
        const REPLACEMENT_LINE: &str = "list(APPEND clingo_targets libgringo libreify ordered_map hopscotch_map sparse_map optional variant wide_integer)";

        let original_content = fs::read_to_string(&target_file_path)
            .expect("Failed to read CMakeLists.txt for patching.");

        if !original_content.contains(LINE_TO_FIND) {
            panic!("Could not find the target line to patch!");
        }

        let patched_content = original_content.replace(LINE_TO_FIND, REPLACEMENT_LINE);

        fs::write(&target_file_path, patched_content)
            .expect("Failed to write patched CMakeLists.txt.");

        println_warning("Patch applied successfully.");
    }

    /// Clones the clingo repository from GitHub.
    ///
    /// This function uses the `git` command line tool to clone the
    /// repository into the specified path.
    ///
    /// # Parameters
    /// - `repo_url`: The URL of the clingo GitHub repository.
    /// - `repo_tag`: The specific tag or branch to clone.
    /// - `path`: The local path where the repository should be cloned.
    fn clone_clingo(repo_url: &str, repo_tag: &str, path: &Path) {
        println_warning("Cloning clingo...");

        let status = Command::new("git")
            .args(["clone", "--branch", repo_tag, "--depth", "1", repo_url])
            .arg(path)
            .status()
            .expect("Failed to execute 'git clone'. Is git installed and in your PATH?");

        if !status.success() {
            panic!("'git clone' command failed with exit status: {status}.");
        }
    }

    /// Builds the clingo libraries using CMake.
    ///
    /// This function configures and builds clingo with specific options
    /// using the `cmake` crate.
    ///
    /// # Parameters
    /// - `path`: The path to the clingo source code.
    ///
    /// # Returns
    /// The path to the installation directory of the built libraries.
    fn build_libraries(path: &str) -> PathBuf {
        use cmake::Config;
        let mut config = Config::new(path);

        config
            .very_verbose(true)
            .define("CLINGO_BUILD_SHARED", "OFF")
            .define("CLINGO_BUILD_STATIC", "ON")
            .define("CLINGO_MANAGE_RPATH", "OFF")
            .define("CLINGO_BUILD_WITH_PYTHON", "OFF")
            .define("CLINGO_BUILD_WITH_LUA", "OFF")
            .define("CLINGO_INSTALL_LIB", "ON")
            .define("CLINGO_BUILD_APPS", "OFF")
            .define("CLASP_BUILD_APP", "OFF");

        let profile = match std::env::var("PROFILE") {
            Ok(p) => p,
            Err(_) => "debug".to_string(),
        };

        if profile == "release" {
            let flags = "/MD /O2 /Ob2 /DNDEBUG /GL-";
            config.define("CMAKE_CXX_FLAGS_RELEASE", flags);
            config.define("CMAKE_C_FLAGS_RELEASE", flags);
        }

        config.build()
    }

    /// Copies the compiled clingo libraries to the Rust project's `lib/` directory.
    ///
    /// This function ensures that the necessary libraries are available
    /// for linking during the Rust build process.
    ///
    /// # Parameters
    /// - `manifest_dir`: The directory of the Cargo manifest.
    /// - `install_path`: The path where the clingo libraries were installed after building.
    fn copy_libs_to_project(manifest_dir: &Path, install_path: &Path) {
        println_warning("Copying compiled libraries to project's lib/ folder...");
        let project_lib_dir = manifest_dir.join("lib");
        fs::create_dir_all(&project_lib_dir)
            .expect("Failed to create lib/ directory in project root.");

        let build_lib_dir = install_path.join("lib");
        let libs_to_copy = ["clingo", "reify", "potassco", "clasp", "gringo"];

        for lib_name in libs_to_copy {
            let source_file = build_lib_dir.join(format!("{lib_name}.lib"));
            let dest_file = project_lib_dir.join(format!("{lib_name}.lib"));

            if source_file.exists() {
                fs::copy(&source_file, &dest_file)
                    .unwrap_or_else(|_| panic!("Failed to copy {}", source_file.display()));
                println_warning(
                    format!(
                        "Copied {} to {}",
                        source_file.display(),
                        dest_file.display()
                    )
                    .as_str(),
                );
            } else {
                panic!(
                    "Could not find compiled library to copy: {}",
                    source_file.display()
                );
            }
        }
    }

    /// Generates Rust bindings for the clingo C API using bindgen.
    ///
    /// This function reads the clingo header files and generates
    /// the corresponding Rust bindings, which are then written to
    /// the `src/clingo/bindings.rs` file in the project.
    ///
    /// # Parameters
    /// - `manifest_dir`: The directory of the Cargo manifest.
    /// - `clingo_source_path`: The path to the clingo source code.
    fn generate_bindings(manifest_dir: &Path, clingo_source_path: &Path) {
        let header_path = clingo_source_path.join("libclingo").join("clingo.h");
        let bindings = bindgen::Builder::default()
            .header(header_path.to_str().unwrap())
            .raw_line("#![allow(missing_docs)]")
            .no_copy("clingo_model")
            .no_copy("clingo_solve_handle")
            .no_copy("clingo_control")
            .no_copy("clingo_options")
            .allowlist_type("clingo_control.*")
            .allowlist_type("clingo_options.*")
            .allowlist_type("clingo_model.*")
            .allowlist_type("clingo_solve_handle.*")
            .allowlist_type("clingo_solve_result_.*")
            .allowlist_type("clingo_symbol.*")
            .allowlist_type("clingo_error.*")
            .allowlist_function("clingo_control_.*")
            .allowlist_function("clingo_solve_handle_.*")
            .allowlist_function("clingo_model_.*")
            .allowlist_function("clingo_symbol_.*")
            .allowlist_function("clingo_configuration_.*")
            .allowlist_function("clingo_error_.*")
            .allowlist_function("clingo_version")
            .generate()
            .expect("Unable to generate bindings");

        let bindings_path = manifest_dir.join("src").join("clingo").join("bindings.rs");
        bindings
            .write_to_file(&bindings_path)
            .expect("Couldn't write bindings!");
    }

    /// Main function to run the bundling process.
    ///
    /// This function orchestrates the entire process of
    /// cloning, patching, building, copying libraries, and generating bindings.
    pub fn run() {
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

        let vendor_dir = manifest_dir.join("vendor");
        let repo_path = vendor_dir.join("clingo");
        if !repo_path.exists() {
            fs::create_dir_all(&vendor_dir).expect("Failed to create vendor directory");
            clone_clingo(
                "https://github.com/potassco/clingo.git",
                "v5.8.0",
                &repo_path,
            );

            patch_clingo_manually(&repo_path);
        }

        generate_bindings(&manifest_dir, &repo_path);

        let path = build_libraries(repo_path.to_str().unwrap());

        println_warning(format!("Clingo built and installed to: {}", path.display()).as_str());

        copy_libs_to_project(&manifest_dir, &path);
    }
}

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    #[cfg(feature = "bundle-clingo")]
    {
        bundle::run();
    }

    link_libraries(&manifest_dir);
}
