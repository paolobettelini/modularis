fn main() {
    // cargo-leptos produces dist/ independently of Cargo's Rust source graph.
    // In release server builds, make the generated bundle an explicit input so
    // `cargo leptos build --release` followed by `cargo run/build --release`
    // always recompiles rust_embed when JS/WASM/CSS changed.
    let is_server = std::env::var_os("CARGO_FEATURE_SERVER").is_some();
    let is_release = std::env::var("PROFILE").is_ok_and(|profile| profile == "release");

    if is_server && is_release {
        println!("cargo:rerun-if-changed=dist");
    }
}
