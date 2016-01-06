include(FindPackageHandleStandardArgs)

find_program(RustC_PATH rustc)

find_package_handle_standard_args(RustC REQUIRED_VARS RustC_PATH)
