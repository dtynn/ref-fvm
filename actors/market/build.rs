fn main() {
    use wasm_builder::WasmBuilder;

    #[cfg(not(feature = "wasm-prof"))]
    WasmBuilder::new()
        .with_current_project()
        .append_to_rust_flags("-Ctarget-feature=+crt-static")
        .append_to_rust_flags("-Cpanic=abort")
        .append_to_rust_flags("-Coverflow-checks=yes")
        .append_to_rust_flags("-Clto=yes")
        .append_to_rust_flags("-Copt-level=z")
        .build();

    #[cfg(feature = "wasm-prof")]
    std::env::set_var("WASM_BUILD_TYPE", "debug");

    #[cfg(feature = "wasm-prof")]
    WasmBuilder::new()
        .with_current_project()
        .append_to_rust_flags("-Ctarget-feature=+crt-static")
        .append_to_rust_flags("-Cpanic=abort")
        .append_to_rust_flags("-Coverflow-checks=yes")
        .append_to_rust_flags("--emit=llvm-ir")
        .append_to_rust_flags("-Zinstrument-coverage")
        .append_to_rust_flags("-Zno-profiler-runtime")
        .append_to_rust_flags("-Ccodegen-units=1")
        .append_to_rust_flags("-Copt-level=0")
        .append_to_rust_flags("-Clink-dead-code")
        .build();
}
