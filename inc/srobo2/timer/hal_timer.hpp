#pragma once

#include <stm32f4xx_hal.h>
#include <srobo2/ffi/base.hpp>

namespace srobo2::timer {
class HALCTime {
  srobo2::ffi::CTime time;

  static float Now(const void *context) { return HAL_GetTick() / 1000.0f; }

  static void Sleep(const void *context, float duration) {
    HAL_Delay(duration * 1000);
  }

  static HALCTime instance;

 public:
  static HALCTime *GetInstance() { return &instance; }

  void Init() {
    srobo2::ffi::__ffi_ctime_set_context(&time, this);
    srobo2::ffi::__ffi_ctime_set_now(&time, &Now);
    srobo2::ffi::__ffi_ctime_set_sleep(&time, &Sleep);
  }

  srobo2::ffi::CTime *GetTime() { return &time; }
};

HALCTime HALCTime::instance;
}  // namespace srobo2::timer