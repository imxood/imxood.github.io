
# 定义一个全局(<GLOBAL | DIRECTORY | TARGET | SOURCE | TEST | VARIABLE | CACHED_VARIABLE>)的属性名"ZEPHYR_LIBS"
define_property(
    GLOBAL PROPERTY ZEPHYR_LIBS
    BRIEF_DOCS "Global list of all Zephyr CMake libs that should be linked in"
    FULL_DOCS  "Global list of all Zephyr CMake libs that should be linked in. zephyr_library() appends libs to this list."
)
# 设置属性值
set_property(GLOBAL PROPERTY ZEPHYR_LIBS "zzz")
# 获取属性值
get_property(zephyr_libs GLOBAL PROPERTY ZEPHYR_LIBS)
message(STATUS "zephyr_libs: '${zephyr_libs}'")




###################################################

# function(tst_arguments)

# 	CMAKE_PARSE_ARGUMENTS(TEST "" "NAME;COMMAND;BASELINE" "ARGSLIST" ${ARGN})

# 	message("TEST_DEFAULT_ARGS is ${TEST_DEFAULT_ARGS} from ${ARGN}")
# 	message("TEST_NAME is ${TEST_NAME}")
# 	message("TEST_COMMAND is ${TEST_COMMAND}")
# 	message("TEST_ARGSLIST is ${TEST_ARGSLIST}")
# 	message("TEST_BASELINE is ${TEST_BASELINE}")

# endfunction()

# tst_arguments()

# TEST_ARGUMENT(
# 	NAME
# 		testiso
# 	COMMAND
# 		"RunMe"
# 	ARGSLIST
# 		${SRC}
# 	BASELINE
# 		"$ENV{HOME}"
# )


# function(check_module result name)
# 	set(options)
# 	set(oneValueArgs PATH EXPECTED_ARCH EXPECTED_VERSION VERSION_COMPARE)
# 	set(multiValueArgs)
# 	cmake_parse_arguments(check_module
# 							"${options}"
# 							"${oneValueArgs}"
# 							"${multiValueArgs}"
# 							${ARGN})

# 	set(res "false")
# 	set(module_path ${check_module_PATH})
# 	message(STATUS "check_module_PATH: '${check_module_PATH}'")

# endfunction()

# check_module()


# 几个方便的函数

function(add_sources)
	message(STATUS "ARGN: '${ARGN}'")
endfunction()


function(add_sources_if condition)
	if(${${condition}})
		add_sources(${ARGN})
	endif()
endfunction()


function(add_glob_sources)
	message(STATUS "ARGN: '${ARGN}'")
	file(GLOB_RECURSE SOURCE_LIST ${ARGN})
	message(STATUS "SOURCE_LIST: ${SOURCE_LIST}")
endfunction()

function(add_glob_sources_if condition)
	message(STATUS "ARGN: '${ARGN}'")
	if(${${condition}})
		file(GLOB_RECURSE SOURCE_LIST ${ARGN})
		target_sources(app PRIVATE ${SOURCE_LIST})
		message(STATUS "SOURCE_LIST: ${SOURCE_LIST}")
	endif()
endfunction()

function(add_dir_sources)
	message(STATUS "dir: ${ARGN}")
	if(IS_DIRECTORY ${ARGN})
		file(GLOB_RECURSE SOURCE_LIST "${ARGN}/*.c")
		add_sources(${SOURCE_LIST})
	endif()
endfunction()

function(add_dir_sources_if condition)
	if(${${condition}})
		add_dir_sources(${ARGN})
	endif()
endfunction()

set(CONFIG_TEST ON)

add_sources(test_build.cmake)

add_sources_if(CONFIG_TEST test_build.cmake CMakeLists.txt)



# 解析参数

function(MY_PARSE)

	message(STATUS "Parse params")

	# Arg, ADULT Asa
	set(options ADULT Asa)

	# Arg value, NAME zhangsan AGE 20
	set(oneValueArgs NAME AGE)

	# Arg value1 value2 value3 .. , SCORE 100 200 300 400 500
	set(multiValueArgs SCORE)

    cmake_parse_arguments(STUDENT "ADULT;Asa" "${oneValueArgs}" "${multiValueArgs}" ${ARGN} )

    # 通过 prefix_参数名: 例如 STUDENT_ADULT
	message("isadult  = ${STUDENT_ADULT}")
	message("Asa  = ${STUDENT_Asa}")
    message("name  = ${STUDENT_NAME}")
    message("age  = ${STUDENT_AGE}")
    message("score  = ${STUDENT_SCORE}")

endfunction()

MY_PARSE(ADULT Asa NAME zhangsan AGE 20 SCORE 100 200 300 400 500)

# find_program, 查找程序

find_program(DTC dtc)

message(STATUS "DTC: ${DTC}")

if(${DTC} STREQUAL DTC-NOTFOUND)
	message(STATUS "Unable to find dtc")
endif()

set(MIN_DTC_VERSION 1.4.6)
execute_process(
	COMMAND
	${DTC} --version
	OUTPUT_VARIABLE dtc_version_output
)

message(STATUS "dtc_version_output: ${dtc_version_output}")

string(REGEX MATCH "Version: DTC ([0-9]+\.[0-9]+.[0-9]+).*" out_var ${dtc_version_output})

if(${CMAKE_MATCH_1} VERSION_LESS ${MIN_DTC_VERSION})
	# assert(0 "The detected dtc version is unsupported.                                 \n\
	# 	The version was found to be ${CMAKE_MATCH_1}                                   \n\
	# 	But the minimum supported version is ${MIN_DTC_VERSION}                        \n\
	# 	See https://docs.zephyrproject.org/latest/getting_started/                     \n\
	# 	for how to use the SDK's dtc alongside a custom toolchain."
	# )
	message(WARNING "dtc ${CMAKE_MATCH_1} VERSION_LESS ${MIN_DTC_VERSION}")
endif()



### 宏参数 ######

macro(macro_test hello world)
	# 所有的参数
	MESSAGE(STATUS ARGV=${ARGV})
	# 非显式声明的所有参数
	MESSAGE(STATUS ARGN=${ARGN})

	MESSAGE(STATUS ARGV0=${ARGV0})
	MESSAGE(STATUS ARGV1=${ARGV1})
	MESSAGE(STATUS ARGV2=${ARGV2})
	MESSAGE(STATUS ARGV3=${ARGV3})
endmacro()

# 调用宏时传入4个参数
macro_test(TOM JERRY SUSAN BERN)


function(print_property_attributes type name propName)
	if ("${type}" STREQUAL "CACHE")
		set(propTypeArgs)
		list(APPEND propTypeArgs CACHE)
		list(APPEND propTypeArgs "${name}")
			# list used because "set(propTypeArgs CACHE "${name}")" is an error...
		if ("${propName}" STREQUAL "")
			set(propName VALUE)
		endif()
	elseif ("${type}" STREQUAL "VARIABLE")
		set(propTypeArgs VARIABLE)
		set(propName "${name}") # force propName to variable name for VARIABLE
	else()
		message("type '${type}' not implemented yet...")
		return()
	endif()

	message("propName='${propName}'") # the name of the property

	get_property(propIsSet ${propTypeArgs} PROPERTY "${propName}" SET)
	message("propIsSet='${propIsSet}'")

	if (propIsSet)
		get_property(propValue ${propTypeArgs} PROPERTY "${propName}")
		message("propValue='${propValue}'")

		get_property(propIsDefined ${propTypeArgs} PROPERTY "${propName}" DEFINED)
		message("propIsDefined='${propIsDefined}'")

		get_property(propBriefDocs ${propTypeArgs} PROPERTY "${propName}" BRIEF_DOCS)
		message("propBriefDocs='${propBriefDocs}'")

		get_property(propFullDocs ${propTypeArgs} PROPERTY "${propName}" FULL_DOCS)
		message("propFullDocs='${propFullDocs}'")

		if ("${type}" STREQUAL "CACHE")
			if ("${propName}" STREQUAL "VALUE")
				print_property_attributes(CACHE "${name}" ADVANCED)
				print_property_attributes(CACHE "${name}" HELPSTRING)
				print_property_attributes(CACHE "${name}" MODIFIED)
				print_property_attributes(CACHE "${name}" STRINGS)
				print_property_attributes(CACHE "${name}" TYPE)
			endif()
		endif()
	endif()
endfunction()


function(print_variable_property_values varname)
	set(name "${varname}")
	set(value "${${varname}}")

	message("name='${name}'")
	message("value='${value}'")
	# SET, 是否是被set_property; DEFINE, 是否是被define_property
	get_property(varPropIsSet VARIABLE PROPERTY "${name}" SET)

	if (varPropIsSet)
		message("type='VARIABLE'")
		print_property_attributes(VARIABLE "${name}" "")
	else()
		message("variable '${name}' is not set")
	endif()

	get_property(cachePropIsSet CACHE "${name}" PROPERTY VALUE SET)

	if (cachePropIsSet)
		message("type='CACHE'")
		print_property_attributes(CACHE "${name}" "")
	else()
		message("cache entry '${name}' is not set")
	endif()

endfunction()


message("cmake version ${CMAKE_VERSION}")

set(plain_variable "plain variable value")
set(cache_entry "cache entry value" CACHE STRING "cache doc string")

print_variable_property_values(plain_variable)
print_variable_property_values(cache_entry)


# 把文件内容按行读取到列表中
file(STRINGS test_build.cmake myfile)
