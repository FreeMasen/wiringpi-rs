mod bindings;

pub mod WiringPi {
    pub use bindings::HIGH;
    pub use bindings::LOW;
    use bindings::wiringPiNodeStruct;

    pub fn digital_write(pin: i32, value: u32) {
        unsafe { wiringPiNodeStruct.digitalWrite(pin, value) }
    }
}
