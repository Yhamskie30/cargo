//! Tests for -Zextra-link-arg.

use cargo_test_support::{basic_bin_manifest, project};

#[cargo_test]
fn build_script_extra_bin_link_args() {
    let p = project()
        .file("Cargo.toml", &basic_bin_manifest("foo"))
        .file("src/main.rs", "fn main() {}")
        .file(
            "build.rs",
            r#"
                fn main() {
                    println!("cargo:rustc-bin-link-arg=--this-is-a-bogus-flag");
                }
            "#,
        )
        .build();

    p.cargo("build -Zextra-link-arg -v")
        .masquerade_as_nightly_cargo()
        .with_status(101)
        .with_stderr_contains(
            "[RUNNING] `rustc --crate-name foo [..]-C link-arg=--this-is-a-bogus-flag[..]",
        )
        .run();
}

#[cargo_test]
fn build_script_extra_link_args() {
    let p = project()
        .file("Cargo.toml", &basic_bin_manifest("foo"))
        .file("src/main.rs", "fn main() {}")
        .file(
            "build.rs",
            r#"
                fn main() {
                    println!("cargo:rustc-link-arg=--this-is-a-bogus-flag");
                }
            "#,
        )
        .build();

    p.cargo("build -Zextra-link-arg -v")
        .masquerade_as_nightly_cargo()
        .with_status(101)
        .with_stderr_contains(
            "[RUNNING] `rustc --crate-name foo [..]-C link-arg=--this-is-a-bogus-flag[..]",
        )
        .run();
}

#[cargo_test]
fn build_script_extra_link_args_warn_on_stable() {
    let p = project()
        .file("Cargo.toml", &basic_bin_manifest("foo"))
        .file("src/main.rs", "fn main() {}")
        .file(
            "build.rs",
            r#"
                fn main() {
                    println!("cargo:rustc-link-arg=--this-is-a-bogus-flag");
                }
            "#,
        )
        .build();

    p.cargo("build -vv")
        .with_status(0)
        .with_stderr_contains("warning: cargo:rustc-link-arg requires -Zextra-link-arg flag")
        .run();
}
