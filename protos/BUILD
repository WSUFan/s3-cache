package(default_visibility = ["//visibility:public"])
load("@rules_rust//rust:defs.bzl", "rust_library")
load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")

rust_library(
    name = "configuration",
    srcs = [
        "lib.rs",
        "configuration.rs"
    ],
    crate_name = "protos",
    deps = all_crate_deps(
        normal = True,
    )
)