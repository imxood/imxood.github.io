
cmake_minimum_required(VERSION 3.5)

project(MXSpice LANGUAGES CXX)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

set(CMAKE_INCLUDE_CURRENT_DIR ON)

set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} ${CMAKE_CURRENT_LIST_DIR}/cmake)

find_package(PkgConfig)

# QtCreator supports the following variables for Android, which are identical to qmake Android variables.
# Check http://doc.qt.io/qt-5/deployment-android.html for more information.
# They need to be set before the find_package(Qt5 ...) call.

#if(ANDROID)
#    set(ANDROID_PACKAGE_SOURCE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/android")
#    if (ANDROID_ABI STREQUAL "armeabi-v7a")
#        set(ANDROID_EXTRA_LIBS
#            ${CMAKE_CURRENT_SOURCE_DIR}/path/to/libcrypto.so
#            ${CMAKE_CURRENT_SOURCE_DIR}/path/to/libssl.so)
#    endif()
#endif()

find_package(Qt5 COMPONENTS Widgets LinguistTools REQUIRED)

find_package(Boost REQUIRED COMPONENTS system)

if(UNIX)
    pkg_check_modules(NGSPICE REQUIRED ngspice)
    message(STATUS "NGSPICE_LIBRARIES: ${NGSPICE_LIBRARIES}")
    message(STATUS "NGSPICE_LIBRARY_DIRS: ${NGSPICE_LIBRARY_DIRS}")
    message(STATUS "NGSPICE_INCLUDE_DIRS: ${NGSPICE_INCLUDE_DIRS}")
    link_directories(${NGSPICE_LIBRARY_DIRS})
endif()

set(
    SOURCES
    main.cpp
    mainwindow.cpp
    mainwindow.h
    ngspice.cpp
    customerevent.cpp
    spicereporter.cpp
    mainwindow.ui
    ${TS_FILES}
)

# set(TS_FILES MXSpice_zh_CN.ts)

if(ANDROID)
  add_library(MXSpice SHARED ${SOURCES})
else()
  add_executable(MXSpice ${SOURCES})
endif()

#add_definitions("-DCPDEBUG=1")
set_property(SOURCE ngspice.cpp APPEND PROPERTY COMPILE_DEFINITIONS NGSPICE_LIBRARIES="${NGSPICE_LIBRARIES}")

include_directories(${NGSPICE_INCLUDE_DIRS} ${Boost_INCLUDE_DIRS})

link_directories(${NGSPICE_LIBRARY_DIRS})
#link_libraries(${NGSPICE_LIBRARIES})

target_link_libraries(MXSpice PRIVATE Qt5::Widgets ${Boost_LIBRARIES})

qt5_create_translation(QM_FILES ${CMAKE_SOURCE_DIR} ${TS_FILES})
