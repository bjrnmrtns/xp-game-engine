cmake_minimum_required(VERSION 3.12)
project(sdlwindow)

find_package(SDL2 REQUIRED)

set(CMAKE_CXX_STANDARD 20)

add_library(sdlwindow STATIC
        src/sdlwindow.cpp
)

target_link_libraries(sdlwindow PRIVATE ${SDL2_LIBRARIES})
target_include_directories(sdlwindow PRIVATE ${SDL2_INCLUDE_DIRS})

install(TARGETS sdlwindow DESTINATION .)
