#pragma once

#include <srobo2/ffi/base.hpp>
#include <mbed.h>

#include <memory>

namespace srobo2::com {

class UARTCStreamTx {
  srobo2::ffi::CStreamTx tx_;
  std::shared_ptr<mbed::UnbufferedSerial> stream;

  static void Write(const void* instance, const void* context,
                    const uint8_t* data, size_t len) {
    auto stream = static_cast<const UARTCStreamTx*>(instance);
    stream->stream->write(const_cast<void*>(static_cast<const void*>(data)),
                          len);
  }

 public:
  UARTCStreamTx(std::shared_ptr<mbed::UnbufferedSerial> stream)
      : stream(stream) {
    srobo2::ffi::__ffi_cstream_associate_tx(&tx_, this, &UARTCStreamTx::Write);
  }

  srobo2::ffi::CStreamTx* GetTx() { return &tx_; }
};

class UARTCStreamRx {
  srobo2::ffi::CStreamRx* rx_;
  std::shared_ptr<mbed::UnbufferedSerial> stream;

 public:
  UARTCStreamRx(std::shared_ptr<mbed::UnbufferedSerial> stream)
      : stream(stream) {
    rx_ = srobo2::ffi::__ffi_cstream_new_rx();
    stream->attach([this]() {
      char buf;
      auto len = this->stream->read(&buf, 1);

      if (len == 1) {
        srobo2::ffi::__ffi_cstream_feed_rx(rx_, (uint8_t*)&buf, 1);
      }
    });
  }

  srobo2::ffi::CStreamRx* GetRx() { return rx_; }
};

}  // namespace srobo2::com