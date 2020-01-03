cmake_minimum_required(VERSION 3.12)
project(windowing)

find_package(SDL2 REQUIRED)

set(CMAKE_CXX_STANDARD 20)

add_library(windowing STATIC
        src/window.cpp
)

target_link_libraries(windowing PRIVATE ${SDL2_LIBRARIES})
target_include_directories(windowing PRIVATE ${SDL2_INCLUDE_DIRS})

install(TARGETS windowing DESTINATION .)
