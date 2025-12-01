add_custom_command(
  OUTPUT "${CMAKE_CURRENT_SOURCE_DIR}/inc/srobo2/ffi/base.hpp"
  COMMAND cbindgen --lang c++ --output inc/srobo2/ffi/base.hpp
  WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
)

add_custom_target(srobo_base_hxx_target DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/inc/srobo2/ffi/base.hpp")


add_library(srobo_base_hxx INTERFACE)
add_dependencies(srobo_base_hxx srobo_base_hxx_target)
target_include_directories(srobo_base_hxx INTERFACE ./inc)

if (DEFINED MBED_TARGET)
  find_package(StaticMbedOS REQUIRED)
  target_link_libraries(srobo_base_hxx INTERFACE StaticMbedOS)
endif()
