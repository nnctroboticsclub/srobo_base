use core::time::Duration;

pub trait TimeImpl {
    fn now(&self) -> Duration; // Returns the elapsed time since the program started

    fn sleep(&self, duration: Duration); // Sleeps for the specified duration
}
