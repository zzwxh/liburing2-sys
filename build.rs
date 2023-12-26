fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let lto = std::env::var("CARGO_FEATURE_LTO").is_ok();

    let cflags = match lto {
        true => "CFLAGS=\"-g -O3 -Wall -Wextra -flto=thin\"",
        false => "CFLAGS=\"-g -O3 -Wall -Wextra\"",
    };

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let script = format!(
        r#"
cd {out_dir}
rm -rf build
cp -r {manifest}/liburing build
cd build
./configure --cc=clang --use-libc
cd src
make V=1 {cflags} liburing-ffi.a
cp liburing-ffi.a ../../liburing2-sys.a
"#
    );

    std::process::Command::new("sh")
        .arg("-c")
        .arg(script)
        .status()
        .unwrap();

    println!("cargo:rustc-link-lib=uring2-sys");
    println!("cargo:rustc-link-search={out_dir}");

    let include_dir = format!("{out_dir}/build/src/include");
    let ffi_c = format!("{out_dir}/build/src/ffi.c");
    let liburing_h = format!("{out_dir}/build/src/include/liburing.h");
    let io_uring_h = format!("{out_dir}/build/src/include/liburing/io_uring.h");
    let bindings_rs = format!("{out_dir}/bindings.rs");

    bindgen::Builder::default()
        .clang_arg("--include-directory")
        .clang_arg(include_dir)
        .header(ffi_c)
        .allowlist_file(liburing_h)
        .allowlist_file(io_uring_h)
        .layout_tests(false)
        .merge_extern_blocks(true)
        .sort_semantically(true)
        .default_non_copy_union_style(bindgen::NonCopyUnionStyle::ManuallyDrop)
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .use_core()
        .generate()
        .unwrap()
        .write_to_file(bindings_rs)
        .unwrap();
}
