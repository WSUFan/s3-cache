package(default_visibility = ["//visibility:public"])
load("@rules_rust//rust:defs.bzl", "rust_library")
load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")
load("@rules_rust//cargo:defs.bzl", "cargo_build_script")

cargo_build_script(
    name = "gen_protos",
    srcs = ["build.rs"],
    build_script_env = { 
        "BAZEL_PROTOC": "$(execpath @com_google_protobuf//:protoc)",
    },  
    data = glob(["*.proto"]) + [ 
        "@com_google_protobuf//:protoc",
    ],
    deps = all_crate_deps(build = True),
)

rust_library(
    name = "configuration",
    srcs = [
        "lib.rs",
    ],
    crate_name = "protos",
    deps = all_crate_deps(
        normal = True,
    ) + [":gen_protos"],
)


