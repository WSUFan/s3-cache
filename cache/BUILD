package(default_visibility = ["//visibility:public"])

load("@rules_rust//rust:defs.bzl", "rust_library", "rust_binary")
load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")

rust_library(
    name = "lib",
    srcs = glob(["src/**/*.rs"]),
    crate_name = "cache",
    deps = all_crate_deps(
        normal = True,
    ) + ["//protos:configuration"],
    proc_macro_deps = all_crate_deps(
        proc_macro = True,
    ),
)


rust_binary(
    name = "cache",
    srcs = [
        "src/main.rs",
    ],
    deps = all_crate_deps(
        normal = True,
    ) + [
        ":lib",
        "//protos:configuration"
    ],
    proc_macro_deps = all_crate_deps(
        proc_macro = True,
    )
)