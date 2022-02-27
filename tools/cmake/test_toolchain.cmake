cmake_minimum_required (VERSION 2.6)

########################################################################################################

set(TOOLCHAIN_DIR $ENV{ARMGCC_DIR})

########################################################################################################

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# TOOLCHAIN_DIR AND NANO LIBRARY
string(REGEX REPLACE "\\\\" "/" TOOLCHAIN_DIR "${TOOLCHAIN_DIR}")

if(NOT TOOLCHAIN_DIR)
    message(FATAL_ERROR "***Please set ARMGCC_DIR in envionment variables***")
endif()

# TARGET_TRIPLET
set(TARGET_TRIPLET "arm-none-eabi")

set(TOOLCHAIN_BIN_DIR ${TOOLCHAIN_DIR}/bin)
set(TOOLCHAIN_INC_DIR ${TOOLCHAIN_DIR}/${TARGET_TRIPLET}/include)
set(TOOLCHAIN_LIB_DIR ${TOOLCHAIN_DIR}/${TARGET_TRIPLET}/lib)

set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_PROCESSOR arm)

set(CMAKE_TRY_COMPILE_TARGET_TYPE STATIC_LIBRARY)
set(CMAKE_C_COMPILER ${TOOLCHAIN_BIN_DIR}/${TARGET_TRIPLET}-gcc${TOOLCHAIN_EXT})
set(CMAKE_CXX_COMPILER ${TOOLCHAIN_BIN_DIR}/${TARGET_TRIPLET}-g++${TOOLCHAIN_EXT})
set(CMAKE_ASM_COMPILER ${TOOLCHAIN_BIN_DIR}/${TARGET_TRIPLET}-gcc${TOOLCHAIN_EXT})

set(CMAKE_OBJCOPY ${TOOLCHAIN_BIN_DIR}/${TARGET_TRIPLET}-objcopy CACHE INTERNAL "objcopy tool")
set(CMAKE_OBJDUMP ${TOOLCHAIN_BIN_DIR}/${TARGET_TRIPLET}-objdump CACHE INTERNAL "objdump tool")

# compiler options
set(CMAKE_C_FLAGS
    "-mcpu=cortex-m7 -mabi=aapcs -mthumb\
     -mfloat-abi=hard \
     -mfpu=fpv5-d16 \
    -std=gnu99 \
    --sysroot=${SYSROOT_DIR} \
    -Wno-unused-but-set-variable -Wall\
    -Wunused-parameter -Wundef \
    -fno-omit-frame-pointer -fno-stack-protector \
    -ffunction-sections -fdata-sections \
    -fno-builtin-printf -fno-builtin-sprintf \
    -gdwarf-2 -fno-common -ffreestanding -fno-strict-aliasing \
    -DCONFIG_ARM "
    CACHE INTERNAL
    "c compiler flags")


set(CMAKE_ASM_FLAGS "${CMAKE_ASM_FLAGS} -g")
set(CMAKE_ASM_FLAGS "${CMAKE_ASM_FLAGS} -mcpu=cortex-m7")
set(CMAKE_ASM_FLAGS "${CMAKE_ASM_FLAGS} -mfloat-abi=hard")
set(CMAKE_ASM_FLAGS "${CMAKE_ASM_FLAGS} -mfpu=fpv5-d16")
set(CMAKE_ASM_FLAGS "${CMAKE_ASM_FLAGS} -mthumb")
set(CMAKE_ASM_FLAGS "${CMAKE_ASM_FLAGS} -mapcs")
set(CMAKE_ASM_FLAGS "${CMAKE_ASM_FLAGS} -Wall")
set(CMAKE_ASM_FLAGS "${CMAKE_ASM_FLAGS} -fno-common")
set(CMAKE_ASM_FLAGS "${CMAKE_ASM_FLAGS} -ffunction-sections")
set(CMAKE_ASM_FLAGS "${CMAKE_ASM_FLAGS} -fdata-sections")
set(CMAKE_ASM_FLAGS "${CMAKE_ASM_FLAGS} -ffreestanding")
set(CMAKE_ASM_FLAGS "${CMAKE_ASM_FLAGS} -fno-builtin")
set(CMAKE_ASM_FLAGS "${CMAKE_ASM_FLAGS} -std=gnu99")

set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -g")
set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -mcpu=cortex-m7")
set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -mfloat-abi=hard")
set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -mfpu=fpv5-d16")
set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -mthumb")
set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -Wall")
set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -fno-common")
set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -ffunction-sections")
set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -fdata-sections")
set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -ffreestanding")
set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -fno-builtin")
set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -Wl,-Map=${CMAKE_CURRENT_BINARY_DIR}/app.map")

# set(CMAKE_C_FLAGS_DEBUG "${CMAKE_C_FLAGS_DEBUG} -O0 -g" CACHE INTERNAL "c compiler flags debug")
# set(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} -O0 -g" CACHE INTERNAL "cxx compiler flags debug")
# set(CMAKE_ASM_FLAGS_DEBUG "${CMAKE_ASM_FLAGS_DEBUG} -g" CACHE INTERNAL "asm compiler flags debug")
# set(CMAKE_EXE_LINKER_FLAGS_DEBUG "${CMAKE_EXE_LINKER_FLAGS_DEBUG}" CACHE INTERNAL "linker flags debug")

# set(CMAKE_C_FLAGS_RELEASE "${CMAKE_C_FLAGS_RELEASE} -O3 " CACHE INTERNAL "c compiler flags release")
# set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} -O3 " CACHE INTERNAL "cxx compiler flags release")
# set(CMAKE_ASM_FLAGS_RELEASE "${CMAKE_ASM_FLAGS_RELEASE}" CACHE INTERNAL "asm compiler flags release")
# set(CMAKE_EXE_LINKER_FLAGS_RELESE "${CMAKE_EXE_LINKER_FLAGS_RELESE}" CACHE INTERNAL "linker flags release")

enable_language(ASM C)


########################################################################################################


project(test_project)

########################################################################################################

include_directories(include)

file(GLOB SOURCES_FILES
        test.c
        pm.S
        startup.S
)

add_executable(app.elf ${SOURCES_FILES})


target_link_libraries(app.elf -Wl,--start-group,--end-group)

target_link_libraries(app.elf optimized m)

target_link_libraries(app.elf optimized c)

target_link_libraries(app.elf optimized gcc)

target_link_libraries(app.elf optimized nosys)

#Convert elf file to bin file
ADD_CUSTOM_COMMAND(TARGET app.elf
        POST_BUILD COMMAND
        ${CMAKE_OBJCOPY} -Obinary ${CMAKE_CURRENT_BINARY_DIR}/app.elf ${CMAKE_CURRENT_BINARY_DIR}/app.bin
)

configure_file(
        "${APP_DIR}/config.h.in"
        "${PROJECT_BINARY_DIR}/config.h"
)
