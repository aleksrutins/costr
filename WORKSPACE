load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rules_foreign_cc",
    sha256 = "2a4d07cd64b0719b39a7c12218a3e507672b82a97b98c6a89d38565894cf7c51",
    strip_prefix = "rules_foreign_cc-0.9.0",
    url = "https://github.com/bazelbuild/rules_foreign_cc/archive/0.9.0.tar.gz",
)

load("@rules_foreign_cc//foreign_cc:repositories.bzl", "rules_foreign_cc_dependencies")

rules_foreign_cc_dependencies()

_ALL_CONTENT = """\
filegroup(
    name = "all_srcs",
    srcs = glob(["**"]),
    visibility = ["//visibility:public"],
)
"""

http_archive(
    name = "oatpp",
    build_file_content = _ALL_CONTENT,
    sha256 = "e1f80fa8fd7a74da6737e7fee1a4db68b4d7085a3f40e7d550752d6ff5714583",
    urls = [
        "https://github.com/oatpp/oatpp/archive/refs/tags/1.3.0.tar.gz"
    ]
)