# SPDX-License-Identifier: AGPL-3.0-only
cmake_minimum_required(VERSION 3.22)
project(wutong-knowledge VERSION 0.1.0 LANGUAGES CXX)
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Architecture baseline targets Qt 6.12 LTS; 6.4 is the minimum we build against in CI (Ubuntu LTS).
find_package(Qt6 6.4 REQUIRED COMPONENTS Widgets)
qt_standard_project_setup()

list(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake")
include(RustBridge)
wutong_rust_bridge(TARGET knowledge_bridge PACKAGE wutong-knowledge-bridge)

qt_add_executable(wutong-knowledge src/main.cpp src/MainWindow.cpp src/MainWindow.h)
target_compile_definitions(wutong-knowledge PRIVATE WUTONG_APP_TITLE="Wutong Knowledge")
target_link_libraries(wutong-knowledge PRIVATE Qt6::Widgets knowledge_bridge)
if(MSVC)
  target_compile_options(wutong-knowledge PRIVATE /W4)
else()
  target_compile_options(wutong-knowledge PRIVATE -Wall -Wextra)
endif()
install(TARGETS wutong-knowledge)
