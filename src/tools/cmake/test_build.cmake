
# set(CMAKE_C_COMPILER "/home/mx/work/ose_fw_lastest/ose-dev-code-base/modules/ose_val_baremetal/BullseyeCoverage/bin/arm-zephyr-eabi-gcc")
# set(TOOLCHAIN_C_FLAGS "-mthumb;-mcpu=cortex-m7;-mfpu=fpv5-sp-d16;-mfloat-abi=hard")

# execute_process(
# 	COMMAND ${CMAKE_C_COMPILER} ${TOOLCHAIN_C_FLAGS} --print-libgcc-file-name
# 	OUTPUT_VARIABLE LIBGCC_FILE_NAME
# 	OUTPUT_STRIP_TRAILING_WHITESPACE
# )

message(STATUS "Test Build Cmake")



# 测试循环
set(projects A B C)

foreach(project ${projects})
	message("project: ${project}")
endforeach()



# 文件路径操作
set(name tools/cmake/test_build.cmake)

# 判断文件是否存在
if(EXISTS ${name})
	message("file ${name} exists")
else()
	message("file ${name} not exists")
endif()

# 文件是否是绝对路径
if(NOT IS_ABSOLUTE ${name})
	message("file ${name} is not absolute path")
endif()

# 获取文件名, 获取文件路径
get_filename_component(fname ${name} NAME)
get_filename_component(prefix  ${name} DIRECTORY)

# 删除字符串前后的空格
string(STRIP ${fname} fname)
string(STRIP ${prefix} prefix)

message("fname: ${fname}, prefix: ${prefix}")



# 添加一个可执行程序
add_executable(hello test_build.c)



# 在链接前执行
add_custom_command(
	TARGET hello PRE_LINK
	COMMAND echo "executing a command before linking target hello"
	COMMENT "This command will be executed before linking target hello"
)

# 在编译后执行
add_custom_command(
	TARGET hello POST_BUILD
	COMMAND echo "executing a command after building target hello"
	COMMENT "This command will be executed after building target hello"
)



# 定制运行目标(就像make中的目标一样)
# add_custom_target, 每次都会运行命令
# add_custom_command, 有两种形式,
# 	1. OUTPUT, 产生文件
# 	2. TARGET, 目标定制(就是hook功能, 执行前,执行后等触发)
# 它会根据目标是否有变化,才考虑是否运行命令, 如果没有依赖它的命令它就不会执行
#
# make Test1.txt, 执行命令生成Test1.txt, 第二次执行不会执行, "ninja: no work to do."
# make Test2
set(TEST_FILE "Test1.txt")

add_custom_command(OUTPUT ${TEST_FILE}
	COMMAND echo "Generating Test1.txt file..."
	COMMAND ${CMAKE_COMMAND} -E copy ${CMAKE_CURRENT_LIST_FILE} ${TEST_FILE}
	COMMENT  "This is a Test1"
)

## Test2每一次都会运行
add_custom_target(Test2
	ALL DEPENDS ${TEST_FILE}
	COMMAND echo "Run Test2..."
	COMMENT  "This is a Test2"
)

# 这个command会在Test2执行前执行,,,,, PRE_BUILD只对VS有效!
add_custom_command(TARGET Test2
	PRE_BUILD
	COMMAND echo "executing a fake command"
	COMMENT "This command will be executed before building target Test2"
)

add_custom_target(Test3
	COMMAND echo "hello, I am Test3"
	COMMENT "This is Test3"
)

add_custom_command(OUTPUT Test4.txt
	COMMAND touch Test4.txt
	COMMENT "This is Test4"
)



# externalproject_add




# add_custom_target 添加定制目标, 给目标添加属性, 显示属性, 执行目标

add_custom_target(test_target)

set_property(
	TARGET test_target
    APPEND PROPERTY COMPILE_DEFINITIONS
	"abc:${CMAKE_CURRENT_SOURCE_DIR}/testfile"
)

# Filesystem

# 把内容输出到文件
file(GENERATE
    OUTPUT "includes.txt"
    CONTENT "$<TARGET_PROPERTY:test_target,COMPILE_DEFINITIONS>\n"
)

# 计算给出的文件的sha256
file(SHA256 ${filename} sum_str)

# 递归删除
file(REMOVE_RECURSE tools)

# 创建目录, 包含父目录
file(MAKE_DIRECTORY tools)
file(COPY tools/ text.txt DESTINATION /tmp)


# build chain

set(CROSS_COMPILE "arm-zephyr-eabi")

find_program(TOOL_size ${CROSS_COMPILE}-size)
if(NOT TOOL_size)
    message(FATAL_ERROR " ${CROSS_COMPILE}-size tool not found!")
endif()

find_program(TOOL_objdump ${CROSS_COMPILE}-objdump)
if(NOT TOOL_objdump)
    message(FATAL_ERROR " ${CROSS_COMPILE}-objdump tool not found!")
endif()

find_program(TOOL_objcopy ${CROSS_COMPILE}-objcopy)
if(NOT TOOL_objcopy)
    message(FATAL_ERROR " ${CROSS_COMPILE}-objcopy tool not found!")
endif()
