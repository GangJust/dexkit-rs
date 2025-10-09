cmake_minimum_required(VERSION 3.4.1...3.27)
project("dexkit_wrapper")

set(CMAKE_CXX_STANDARD 20)

# find_package(cxx REQUIRED CONFIG)
# link_libraries(cxx::cxx)

add_subdirectory(../DexKit/Core Core)
target_include_directories(dexkit_static PUBLIC ../DexKit/Core/include)

add_library(${CMAKE_PROJECT_NAME} STATIC
        dexkit_wrapper.cpp)

target_link_libraries(${CMAKE_PROJECT_NAME}
        dexkit_static
)