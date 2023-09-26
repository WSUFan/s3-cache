workspace(
    name = "lru_s3_cache"
)
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
http_archive(
    name = "rules_rust",
    sha256 = "9d04e658878d23f4b00163a72da3db03ddb451273eb347df7d7c50838d698f49",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.26.0/rules_rust-v0.26.0.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(
    edition = "2021",
    versions = [
        "1.71.0"
    ],
)

load("@rules_rust//proto/protobuf:repositories.bzl", "rust_proto_protobuf_dependencies", "rust_proto_protobuf_register_toolchains")

rust_proto_protobuf_dependencies()

rust_proto_protobuf_register_toolchains()

load("@rules_rust//proto/protobuf:transitive_repositories.bzl", "rust_proto_protobuf_transitive_repositories")

rust_proto_protobuf_transitive_repositories()

load("@rules_rust//crate_universe:defs.bzl", "crates_repository")

crates_repository(
    name = "crate_index",
    cargo_lockfile = "//:Cargo.lock",
    lockfile = "//:Cargo.Bazel.lock",
    manifests = ["//:Cargo.toml"],
)

load("@crate_index//:defs.bzl", "crate_repositories")

crate_repositories()