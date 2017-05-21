cmake_minimum_required(VERSION 3.8)
project(commando)

set(CMAKE_CXX_STANDARD 11)

set(SOURCE_FILES
        src/app.rs
        src/cli.rs
        src/db.rs
        src/lib.rs
        src/main.rs
        src/preferences.rs
        src/project.rs
        src/utils.rs
        src/views.rs)

add_executable(commando ${SOURCE_FILES})