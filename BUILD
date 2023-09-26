package(default_visibility = ["//visibility:public"])

load("@rules_rust//rust:defs.bzl", "rust_library", "rust_binary")
load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")
load("@rules_rust//proto/protobuf:defs.bzl", "rust_proto_library")

proto_library(
    name = "configuration_proto",
    srcs = ["src/protos/configuration.proto"]
)

rust_proto_library(
    name = "configuration",
    deps = [":configuration_proto"],
)

rust_library(
    name = "lib",
    srcs = glob(["src/**/*.rs"]),
    crate_name = "lru_s3_cache",
    deps = all_crate_deps(
        normal = True,
    ) + [
        ":configuration"
    ],
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