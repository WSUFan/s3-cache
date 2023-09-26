package(default_visibility = ["//visibility:public"])

load("@rules_rust//rust:defs.bzl", "rust_library", "rust_binary")
load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")

platform(
    name = "docker_image_platform",
    constraint_values = [
        "@platforms//cpu:x86_64",
        "@platforms//os:linux",
    ],
    exec_properties = {
        "OSFamily": "Linux",
        "container-image": "docker://docker.io/glyefan/rust-builder:1.2",
    },
)

rust_library(
    name = "lib",
    srcs = glob(["src/**/*.rs"]),
    crate_name = "lru_s3_cache",
    deps = all_crate_deps(
        normal = True,
    ),
    proc_macro_deps = all_crate_deps(
        proc_macro = True,
    ),
)


rust_binary(
    name = "lru_s3_cache",
    srcs = [
        "src/main.rs",
        "src/lib.rs"
    ],
    deps = all_crate_deps(
        normal = True,
    ) + [
        ":lib"
    ],
    proc_macro_deps = all_crate_deps(
        proc_macro = True,
    )
)