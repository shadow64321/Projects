cmake_minimum_required(VERSION 3.9)

project(WolfDen)

include_directories(${PROJECT_SOURCE_DIR})
#set(sources "main.cpp" "Functions.hpp")
set(sources "Main_spahgethti_version.cpp")
add_executable(${PROJECT_NAME} ${sources})
