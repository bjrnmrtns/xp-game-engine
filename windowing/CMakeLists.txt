cmake_minimum_required(VERSION 3.12)
project(windowing)

find_package(SDL2 REQUIRED)

add_library(windowing STATIC
    src/windowing.cpp
)

target_link_libraries(windowing PRIVATE ${SDL2_LIBRARIES})
install(TARGETS windowing DESTINATION .)
