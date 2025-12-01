#pragma once

#include <chrono>

#include <mbed.h>
#include <srobo2/ffi/base.hpp>

namespace srobo2::timer {
class MBedTimer {
  mbed::Timer timer;
  srobo2::ffi::CTime ctime;

  static float now(const void* timer) {
    const auto t = static_cast<const mbed::Timer*>(timer);
    const auto elapsed = t->elapsed_time();

    return elapsed.count() * 1e-6f;
  }

  static void sleep(const void* timer, float duration) {
    using namespace std::chrono_literals;

    auto t = static_cast<const mbed::Timer*>(timer);

    auto start = t->elapsed_time();
    auto end = start + duration * 1s;

    while (t->elapsed_time() < end)
      ;
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