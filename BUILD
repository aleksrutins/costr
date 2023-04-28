load("@rules_foreign_cc//foreign_cc:defs.bzl", "cmake")

cmake(
    name = "oatpp",
    cache_entries = {
        "CMAKE_C_FLAGS": "-fPIC",
    },
    lib_source = "@oatpp//:all_srcs",
    out_static_libs = ["oatpp.a"],
)
