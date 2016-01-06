cmake_minimum_required(VERSION 3.0)
project(ping)

list(APPEND CMAKE_MODULE_PATH ${PROJECT_SOURCE_DIR}/cmake)

find_package(RustC)

add_executable(ping src/ping.c)
if(CMAKE_COMPILER_IS_GNUCC)
    target_compile_options(ping PRIVATE -Wall)
endif()

if(RustC_FOUND)
  add_custom_target(ping_rs
    COMMAND ${RustC_PATH} ${PROJECT_SOURCE_DIR}/src/ping.rs
            -o ${PROJECT_BINARY_DIR}/ping_rs
    BYPRODUCTS ${PROJECT_BINARY_DIR}/ping_rs)
endif()
