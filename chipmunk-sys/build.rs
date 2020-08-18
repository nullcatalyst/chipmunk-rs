#![allow(unused_imports, dead_code, unused_variables)]

#[cfg(feature = "bindgen")]
extern crate bindgen;

use std::path::{Path, PathBuf};
use std::{env, fs, io};

// corresponds to the headers that we have in chipmunk-sys/chipmunk-{version}
const CHIPMUNK_HEADERS_BUNDLED_VERSION: &str = "7.0.3";

// means the lastest stable version that can be downloaded from chipmunk's source
const LASTEST_CHIPMUNK_VERSION: &str = "7.0.3";

#[cfg(feature = "bindgen")]
fn compute_include_paths() -> Vec<String> {
    let mut include_paths: Vec<String> = vec![];

    if let Ok(include_path) = env::var("CHIPMUNK_INCLUDE_PATH") {
        include_paths.push(format!("{}", include_path));
    };

    #[cfg(feature = "vcpkg")]
    {
        // don't print the "cargo:xxx" directives, we're just trying to get the include paths here
        let vcpkg_library = vcpkg::Config::new()
            .cargo_metadata(false)
            .probe("chipmunk")
            .unwrap();
        for path in vcpkg_library.include_paths {
            include_paths.push(format!("{}", path.display()));
        }
    }

    include_paths
}

fn get_bundled_header_path() -> PathBuf {
    let mut include_path: PathBuf = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    include_path.push(format!("chipmunk-{}", CHIPMUNK_HEADERS_BUNDLED_VERSION));
    include_path.push("include");
    include_path
}

#[cfg(feature = "use-vcpkg")]
fn get_vcpkg_config() {
    vcpkg::find_package("chipmunk").unwrap();
}

fn link_chipmunk(target_os: &str) {
    #[cfg(feature = "use-vcpkg")]
    {
        // prints the appropriate linking parameters when using pkg-config
        // useless when using "bundled"
        get_vcpkg_config();
    }
}

fn find_cargo_target_dir() -> PathBuf {
    // Infer the top level cargo target dir from the OUT_DIR by searching
    // upwards until we get to $CARGO_TARGET_DIR/build/ (which is always one
    // level up from the deepest directory containing our package name)
    let pkg_name = env::var("CARGO_PKG_NAME").unwrap();
    let mut out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    loop {
        {
            let final_path_segment = out_dir.file_name().unwrap();
            if final_path_segment.to_string_lossy().contains(&pkg_name) {
                break;
            }
        }
        if !out_dir.pop() {
            panic!("Malformed build path: {}", out_dir.to_string_lossy());
        }
    }
    out_dir.pop();
    out_dir.pop();
    out_dir
}

fn copy_dynamic_libraries(chipmunk_compiled_path: &PathBuf, target_os: &str) {
    // Windows binaries do not embed library search paths, so successfully
    // linking the DLL isn't sufficient to find it at runtime -- it must be
    // either on PATH or in the current working directory when we run binaries
    // linked against it. In other words, to run the test suite we need to
    // copy chipmunk.dll out of its build tree and down to the top level cargo
    // binary output directory.
    if target_os.contains("windows") {
        let chipmunk_dll_name = "chipmunk.dll";
        let chipmunk_bin_path = chipmunk_compiled_path.join("bin");
        let target_path = find_cargo_target_dir();

        let src_dll_path = chipmunk_bin_path.join(chipmunk_dll_name);
        let dst_dll_path = target_path.join(chipmunk_dll_name);

        fs::copy(&src_dll_path, &dst_dll_path).expect(&format!(
            "Failed to copy chipmunk dynamic library from {} to {}",
            src_dll_path.to_string_lossy(),
            dst_dll_path.to_string_lossy()
        ));
    }
}

fn main() {
    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");
    let host = env::var("HOST").expect("Cargo build scripts always have HOST");
    let target_os = get_os_from_triple(target.as_str()).unwrap();

    #[cfg(not(feature = "bindgen"))]
    {
        copy_pregenerated_bindings();
        println!("cargo:include={}", get_bundled_header_path().display());
    }

    #[cfg(feature = "bindgen")]
    {
        let include_paths: Vec<String> = compute_include_paths();
        generate_bindings(target.as_str(), host.as_str(), include_paths.as_slice())
    }

    link_chipmunk(target_os);
}

#[cfg(not(feature = "bindgen"))]
fn copy_pregenerated_bindings() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let crate_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    fs::copy(
        crate_path.join("chipmunk_bindings.rs"),
        out_path.join("chipmunk_bindings.rs"),
    )
    .expect("Couldn't find pregenerated bindings!");
}

#[cfg(feature = "bindgen")]
// headers_path is a list of directories where the chipmunk headers are expected
// to be found by bindgen (should point to the include/ directories)
fn generate_bindings(target: &str, host: &str, headers_paths: &[String]) {
    let target_os = get_os_from_triple(target).unwrap();
    let mut bindings = bindgen::Builder::default()
        // enable no_std-friendly output by only using core definitions
        .use_core()
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .ctypes_prefix("libc");

    // Set correct target triple for bindgen when cross-compiling
    if target != host {
        bindings = bindings.clang_arg("-target");
        bindings = bindings.clang_arg(target.clone());
    }

    if headers_paths.len() == 0 {
        // if no paths are being provided, fall back to the headers included in this repo
        let include_path = get_bundled_header_path();
        println!("cargo:include={}", include_path.display());

        bindings = bindings.clang_arg(format!("-I{}", include_path.display()));
    } else {
        // if paths are included, use them for bindgen. Bindgen should use the first one.
        println!("cargo:include={}", headers_paths.join(":"));
        for headers_path in headers_paths {
            bindings = bindings.clang_arg(format!("-I{}", headers_path));
        }
    }

    let bindings = bindings
        .header("wrapper.h")
        .clang_arg("-DCP_USE_CGTYPES=0")
        .whitelist_type("cp.*")
        .whitelist_function("cp.*")
        .whitelist_var("CP_.*")
        .whitelist_function("CP_.*")
        .derive_debug(false)
        .generate()
        .expect("Unable to generate bindings!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("chipmunk_bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn get_os_from_triple(triple: &str) -> Option<&str> {
    triple.splitn(3, "-").nth(2)
}
