cmake_minimum_required(VERSION 3.12)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_FLAGS "-m64 -Wa,-mbig-obj")
set(CMAKE_BUILD_TYPE Debug CACHE STRING "")

project(boost_examples)

cmake_policy(SET CMP0074 NEW)

set(Boost_DEBUG ON)
set(Boost_ARCHITECTURE "x64")

# 设置BOOST_ROOT安装路径
# set(BOOST_ROOT $ENV{BOOST_ROOT})

if(MINGW)
	set(NET_LIB ws2_32 wsock32)
endif(MINGW)

find_package(Boost REQUIRED COMPONENTS program_options date_time)
include_directories(${Boost_INCLUDE_DIR})


add_executable(1_2_boost examples/1_2_boost.cpp)
add_executable(2_2_timer examples/2_2_timer.cpp)

add_executable(2_4_timer examples/2_4_timer.cpp)
target_link_libraries(2_4_timer Boost::date_time)

add_executable(10_5_program_options examples/10_5_program_options.cpp)
target_link_libraries(10_5_program_options Boost::program_options)

add_executable(12_3_sync_client examples/12_3_sync_client.cpp)
target_link_libraries(12_3_sync_client ${NET_LIB})

add_executable(12_3_sync_server examples/12_3_sync_server.cpp)
target_link_libraries(12_3_sync_server ${NET_LIB})

add_executable(12_3_async_server examples/12_3_async_server.cpp)
target_link_libraries(12_3_async_server ${NET_LIB})

add_executable(12_3_async_client examples/12_3_async_client.cpp)
target_link_libraries(12_3_async_client ${NET_LIB})

add_executable(test_websocket_server_sync examples/test_websocket_server_sync.cpp)
target_link_libraries(test_websocket_server_sync ${NET_LIB})

add_executable(test_websocket_client_sync examples/test_websocket_client_sync.cpp)
target_link_libraries(test_websocket_client_sync ${NET_LIB})

add_executable(test_websocket_server_async examples/test_websocket_server_async.cpp)
target_link_libraries(test_websocket_server_async ${NET_LIB})

add_executable(test_websocket_client_async examples/test_websocket_client_async.cpp)
target_link_libraries(test_websocket_client_async ${NET_LIB})

# resolved error: too many sections
# target_compile_options(test_websocket_client_async PUBLIC -Wa,-mbig-obj)
# target_link_options(test_websocket_client_async PUBLIC -Wa,-mbig-obj)
