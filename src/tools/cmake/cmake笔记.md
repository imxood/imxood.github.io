## cmake 命令

cmake -B build                                                          配置 编译目录
cmake --build build                                                     运行 指定的编译路径
cmake --build build --target clean                                      清除编译目录
cmake -E remove_directory build

windows平台下指定架构:
    -A Win32 或 Win64 或 x64, 2019开始 无法使用 x86

cmake --trace-expand --loglevel=VERBOSE ...
    --trace-expand 可以使变量值显示
    --loglevel=VERBOSE 可以使message(VERBOSE ...)有输出

# cmake常用语法

    set(ENV{WCHEVT_SDK_BASE} ${CMAKE_CURRENT_LIST_DIR})                     设置环境变量

    include_directories(include)                                            设置头文件路径
    link_directories(/usr/lib)                                              设置库路径

    add_library(hello1 src/hello.cpp)                                       添加静态库
    add_library(hello2 SHARED src/hello.cpp)                                添加动态库
    add_executable(sayHello src/useHello.cpp)                               添加可执行程序
    target_link_libraries(sayHello hello)                                   链接库

    set(CMAKE_BUILD_TYPE Release)                                           设置编译类型, 默认就是Release

    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11")                    设置编译参数

    set(CMAKE_CXX_STANDARD 11)

    ADD_DEFINITIONS(-DmacroName -DmacroName=macroValue ......)              设置宏

    file(GLOB SRC_FILES src/*.c src/*.h)                                    生成一个匹配条件的文件列表

    set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${PROJECT_SOURCE_DIR}/exec/)         设置可执行程序的生成目录

    set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${PROJECT_SOURCE_DIR}/lib/)          设置动态库的生成目录

    set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${PROJECT_SOURCE_DIR}/lib/static/)   设置静态库的生成目录

    安装可执行程序或库:
    install(
        TARGETS hello1 hello2
        RUNTIME DESTINATION bin
        ARCHIVE DESTINATION lib　#安装静态库到/usr/local/lib目录
        LIBRARY DESTINATION lib　#安装动态库到/usr/local/lib目录
    )

    install(
        #安装头文件到/usr/local/include目录
        FILES ${PROJECT_SOURCE_DIR}/include/hello.h DESTINATION include
    )

    install(
        DIRECTORIES ${PROJECT_SOURCE_DIR}/include/utils DESTINATION include
    )

    执行一个command,生成${TEST_FILE}文件
    add_custom_command(OUTPUT  ${TEST_FILE}
        COMMAND echo "Generating log.txt file..."
        COMMAND ${CMAKE_COMMAND} -E copy ${CMAKE_CURRENT_LIST_FILE} ${TEST_FILE}
        COMMENT  "This is a test"
    )
    目标Test1,会等待${TEST_FILE}的生成,然后执行
    add_custom_target(Test1 ALL DEPENDS ${TEST_FILE})

    执行Test1 command
    add_custom_command(TARGET Test1
        PRE_BUILD
        COMMAND echo "executing a fake command"
        COMMENT "This command will be executed before building target Test1"
    )

    搜集所有在指定路径下的源文件的文件名
    aux_source_directory(< dir > < variable >)

    CMAKE_C_FLAGS 设置C编译选项
    CMAKE_CXX_FLAGS 设置C++编译选项


    -DCMAKE_INSTALL_RPATH=/usr 编译期与运行期的rpath都会多一条
    -DCMAKE_SKIP_RPATH=FALSE 编译期与运行期都不忽略RPATH


    # cmake 获取所有的 include_directories
    get_property(dirs DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR} PROPERTY INCLUDE_DIRECTORIES)
    foreach(dir ${dirs})
      message(STATUS "dir='${dir}'")
    endforeach()


    # cmake 获取target所有的属性
    # Get all propreties that cmake supports
    execute_process(COMMAND cmake --help-property-list OUTPUT_VARIABLE CMAKE_PROPERTY_LIST)

    # Convert command output into a CMake list
    STRING(REGEX REPLACE ";" "\\\\;" CMAKE_PROPERTY_LIST "${CMAKE_PROPERTY_LIST}")
    STRING(REGEX REPLACE "\n" ";" CMAKE_PROPERTY_LIST "${CMAKE_PROPERTY_LIST}")

    function(print_properties)
        message ("CMAKE_PROPERTY_LIST = ${CMAKE_PROPERTY_LIST}")
    endfunction(print_properties)

    function(print_target_properties tgt)

        if(NOT TARGET ${tgt})
          message("There is no target named '${tgt}'")
          return()
        endif()

        foreach (prop ${CMAKE_PROPERTY_LIST})
            string(REPLACE "<CONFIG>" "${CMAKE_BUILD_TYPE}" prop ${prop})
        # Fix https://stackoverflow.com/questions/32197663/how-can-i-remove-the-the-location-property-may-not-be-read-from-target-error-i
        if(prop STREQUAL "LOCATION" OR prop MATCHES "^LOCATION_" OR prop MATCHES "_LOCATION$")
            continue()
        endif()
            # message ("Checking ${prop}")
            get_property(propval TARGET ${tgt} PROPERTY ${prop} SET)
            if (propval)
                get_target_property(propval ${tgt} ${prop})
                message(STATUS "${tgt} ${prop} = ${propval}")
            endif()
        endforeach(prop)
    endfunction(print_target_properties)

## cmake常用的变量

    PROJECT_NAME       项目名称
    PROJECT_SOURCE_DIR 工程的根目录
    PROJECT_BINARY_DIR 运行cmake命令的目录
    CMAKE_INCLUDE_PATH 环境变量,非cmake变量
    CMAKE_LIBRARY_PATH 环境变量
    CMAKE_CURRENT_SOURCE_DIR 当前处理的CMakeLists.txt所在的路径
    CMAKE_SOURCE_DIR cmake配置文件的顶层目录
    CMAKE_BINARY_DIR
    CMAKE_PREFIX_PATH
    CMAKE_FILES_DIRECTORY
    CMAKE_INSTALL_PREFIX

    cmake_minimum_required(VERSION 2.8)

    find_package(PkgConfig)
    pkg_check_modules(audio_processing REQUIRED audio_processing)

    # pkg-config --libs --cflags audio_processing
    pkg_check_modules:
        <audio_processing>_FOUND          ... set to 1 if module(s) exist
        <audio_processing>_LIBRARIES      ... only the libraries (w/o the '-l')
        <audio_processing>_LIBRARY_DIRS   ... the paths of the libraries (w/o the '-L')
        <audio_processing>_LDFLAGS        ... all required linker flags
        <audio_processing>_LDFLAGS_OTHER  ... all other linker flags
        <audio_processing>_INCLUDE_DIRS   ... the '-I' preprocessor flags (w/o the '-I')
        <audio_processing>_CFLAGS         ... all required cflags
        <audio_processing>_CFLAGS_OTHER   ... the other compiler flags

        <audio_processing>_LIBDIR
        <audio_processing>_PREFIX

## 对于/usr/local/lib/pkgconfig下的pc文件

    example: /usr/local/lib/pkgconfig/audio_process.pc:
        libdir=/usr/local/lib
        includedir=/usr/local/include

        Name: audio_processing
        Description: Webrtc audio process module
        Version: 0.1
        Libs: -L${libdir} -laudio_processing
        Cflags: -DWEBRTC_AUDIO_PROCESSING_ONLY_BUILD -I${includedir}/webrtc -DWEBRTC_POSIX

    libdir:
        指定了库的路径

    includedir:
        指定了库的头文件路径

    Libs:
        -L: ${audio_process_LIBRARY_DIRS}
        -l: ${audio_process_LIBRARIES}

    Cflags中的-I:
        指定了${audio_process_INCLUDE_DIRS}

    -DWEBRTC_POSIX:
        表明需要一个WEBRTC_POSIX宏
