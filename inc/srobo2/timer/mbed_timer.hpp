#pragma once

#include <mbed.h>
#include <srobo2/ffi/base.hpp>

namespace srobo2::timer {
class MBedTimer {
  mbed::Timer timer;
  srobo2::ffi::CTime ctime;

  static float now(const void* timer) {
    auto t = static_cast<const mbed::Timer*>(timer);

    return t->read_us() / 1.0E6;
  }

  static void sleep(const void* timer, float duration) {
    auto t = static_cast<const mbed::Timer*>(timer);

    auto start = t->read_us();
    auto end = start + duration * 1.0E6;

    while (t->read_us() < end);
  }

 public:
  MBedTimer() {
    timer.reset();
    timer.start();

    srobo2::ffi::__ffi_ctime_set_context(&ctime, &timer);

    srobo2::ffi::__ffi_ctime_set_now(&ctime, &MBedTimer::now);
    srobo2::ffi::__ffi_ctime_set_sleep(&ctime, &MBedTimer::sleep);
  }

  srobo2::ffi::CTime* GetTime() { return &ctime; }
};
}  // namespace srobo2::timer