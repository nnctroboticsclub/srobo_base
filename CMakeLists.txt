set(SROBO_BASE_FFIInclude ${CMAKE_BINARY_DIR}/srobo_base_inc)

add_custom_command(
  OUTPUT "${SROBO_BASE_FFIInclude}/srobo2/ffi/base.hpp"
  COMMAND cbindgen --lang c++ --output ${SROBO_BASE_FFIInclude}/srobo2/ffi/base.hpp
  WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
)

add_custom_target(srobo_base_hxx_target DEPENDS "${SROBO_BASE_FFIInclude}/srobo2/ffi/base.hpp")

add_library(srobo_base_hxx INTERFACE)
add_dependencies(srobo_base_hxx srobo_base_hxx_target)
target_include_directories(srobo_base_hxx INTERFACE ./inc ${SROBO_BASE_FFIInclude})

if (DEFINED MBED_TARGET)
  find_package(StaticMbedOS REQUIRED)
  target_link_libraries(srobo_base_hxx INTERFACE StaticMbedOS)
endif()

unset(SROBO_BASE_FFIInclude CACHE)