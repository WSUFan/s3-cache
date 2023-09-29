package(default_visibility = ["//visibility:public"])

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