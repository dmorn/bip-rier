# root CMakeLists.txt
cmake_minimum_required(VERSION 3.4.3 FATAL_ERROR)

project(trampoline)

set(HIDAPI_WITH_LIBUSB FALSE) # surely will be used only on Linux
set(BUILD_SHARED_LIBS FALSE) # HIDAPI as static library on all platforms
add_subdirectory(hidapi)

add_executable(trampoline main.c)

# NOTE: no `find_package` is required, since HIDAPI targets are already a part of the project tree
target_link_libraries(trampoline PRIVATE hidapi::hidapi)
