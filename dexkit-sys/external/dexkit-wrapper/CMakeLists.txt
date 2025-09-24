cmake_minimum_required(VERSION 3.4.1...3.27)
project("dexkit_wrapper")

set(CMAKE_CXX_STANDARD 20)

add_subdirectory(../DexKit/Core Core)

target_include_directories(dexkit_static PUBLIC ../DexKit/Core/include)

message(STATUS "Disable dexkit log")
if (DISABLE_DEXKIT_LOG)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -DNDEXKIT_LOG")
    set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -DNDEXKIT_LOG")
endif ()

add_library(${CMAKE_PROJECT_NAME} STATIC
        dexkit_wrapper.cpp)

target_link_libraries(${CMAKE_PROJECT_NAME}
        dexkit_static)

# cmake_rs Install targets
install(TARGETS ${CMAKE_PROJECT_NAME} dexkit_static
    DESTINATION libs
)