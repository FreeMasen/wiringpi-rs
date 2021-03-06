/* automatically generated by rust-bindgen */

pub const WPI_MODE_PINS: ::std::os::raw::c_uint = 0;
pub const WPI_MODE_GPIO: ::std::os::raw::c_uint = 1;
pub const WPI_MODE_GPIO_SYS: ::std::os::raw::c_uint = 2;
pub const WPI_MODE_PHYS: ::std::os::raw::c_uint = 3;
pub const WPI_MODE_PIFACE: ::std::os::raw::c_uint = 4;
pub const WPI_MODE_UNINITIALISED: ::std::os::raw::c_int = -1;
pub const INPUT: ::std::os::raw::c_uint = 0;
pub const OUTPUT: ::std::os::raw::c_uint = 1;
pub const PWM_OUTPUT: ::std::os::raw::c_uint = 2;
pub const GPIO_CLOCK: ::std::os::raw::c_uint = 3;
pub const SOFT_PWM_OUTPUT: ::std::os::raw::c_uint = 4;
pub const SOFT_TONE_OUTPUT: ::std::os::raw::c_uint = 5;
pub const PWM_TONE_OUTPUT: ::std::os::raw::c_uint = 6;
pub const LOW: ::std::os::raw::c_uint = 0;
pub const HIGH: ::std::os::raw::c_uint = 1;
pub const PUD_OFF: ::std::os::raw::c_uint = 0;
pub const PUD_DOWN: ::std::os::raw::c_uint = 1;
pub const PUD_UP: ::std::os::raw::c_uint = 2;
pub const PWM_MODE_MS: ::std::os::raw::c_uint = 0;
pub const PWM_MODE_BAL: ::std::os::raw::c_uint = 1;
pub const INT_EDGE_SETUP: ::std::os::raw::c_uint = 0;
pub const INT_EDGE_FALLING: ::std::os::raw::c_uint = 1;
pub const INT_EDGE_RISING: ::std::os::raw::c_uint = 2;
pub const INT_EDGE_BOTH: ::std::os::raw::c_uint = 3;
pub const PI_MODEL_A: ::std::os::raw::c_uint = 0;
pub const PI_MODEL_B: ::std::os::raw::c_uint = 1;
pub const PI_MODEL_AP: ::std::os::raw::c_uint = 2;
pub const PI_MODEL_BP: ::std::os::raw::c_uint = 3;
pub const PI_MODEL_2: ::std::os::raw::c_uint = 4;
pub const PI_ALPHA: ::std::os::raw::c_uint = 5;
pub const PI_MODEL_CM: ::std::os::raw::c_uint = 6;
pub const PI_MODEL_07: ::std::os::raw::c_uint = 7;
pub const PI_MODEL_3: ::std::os::raw::c_uint = 8;
pub const PI_MODEL_ZERO: ::std::os::raw::c_uint = 9;
pub const PI_VERSION_1: ::std::os::raw::c_uint = 0;
pub const PI_VERSION_1_1: ::std::os::raw::c_uint = 1;
pub const PI_VERSION_1_2: ::std::os::raw::c_uint = 2;
pub const PI_VERSION_2: ::std::os::raw::c_uint = 3;
pub const PI_MAKER_SONY: ::std::os::raw::c_uint = 0;
pub const PI_MAKER_EGOMAN: ::std::os::raw::c_uint = 1;
pub const PI_MAKER_MBEST: ::std::os::raw::c_uint = 2;
pub const PI_MAKER_UNKNOWN: ::std::os::raw::c_uint = 3;
extern "C" {
    #[link_name = "piModelNames"]
    pub static mut piModelNames: [*const ::std::os::raw::c_char; 16usize];
}
extern "C" {
    #[link_name = "piRevisionNames"]
    pub static mut piRevisionNames: [*const ::std::os::raw::c_char; 16usize];
}
extern "C" {
    #[link_name = "piMakerNames"]
    pub static mut piMakerNames: [*const ::std::os::raw::c_char; 16usize];
}
extern "C" {
    #[link_name = "piMemorySize"]
    pub static mut piMemorySize: [::std::os::raw::c_int; 8usize];
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct wiringPiNodeStruct {
    pub pinBase: ::std::os::raw::c_int,
    pub pinMax: ::std::os::raw::c_int,
    pub fd: ::std::os::raw::c_int,
    pub data0: ::std::os::raw::c_uint,
    pub data1: ::std::os::raw::c_uint,
    pub data2: ::std::os::raw::c_uint,
    pub data3: ::std::os::raw::c_uint,
    pub pinMode: ::std::option::Option<unsafe extern "C" fn(node:
                                                                *mut wiringPiNodeStruct,
                                                            pin:
                                                                ::std::os::raw::c_int,
                                                            mode:
                                                                ::std::os::raw::c_int)>,
    pub pullUpDnControl: ::std::option::Option<unsafe extern "C" fn(node:
                                                                        *mut wiringPiNodeStruct,
                                                                    pin:
                                                                        ::std::os::raw::c_int,
                                                                    mode:
                                                                        ::std::os::raw::c_int)>,
    pub digitalRead: ::std::option::Option<unsafe extern "C" fn(node:
                                                                    *mut wiringPiNodeStruct,
                                                                pin:
                                                                    ::std::os::raw::c_int)
                                               -> ::std::os::raw::c_int>,
    pub digitalWrite: ::std::option::Option<unsafe extern "C" fn(node:
                                                                     *mut wiringPiNodeStruct,
                                                                 pin:
                                                                     ::std::os::raw::c_int,
                                                                 value:
                                                                     ::std::os::raw::c_int)>,
    pub pwmWrite: ::std::option::Option<unsafe extern "C" fn(node:
                                                                 *mut wiringPiNodeStruct,
                                                             pin:
                                                                 ::std::os::raw::c_int,
                                                             value:
                                                                 ::std::os::raw::c_int)>,
    pub analogRead: ::std::option::Option<unsafe extern "C" fn(node:
                                                                   *mut wiringPiNodeStruct,
                                                               pin:
                                                                   ::std::os::raw::c_int)
                                              -> ::std::os::raw::c_int>,
    pub analogWrite: ::std::option::Option<unsafe extern "C" fn(node:
                                                                    *mut wiringPiNodeStruct,
                                                                pin:
                                                                    ::std::os::raw::c_int,
                                                                value:
                                                                    ::std::os::raw::c_int)>,
    pub next: *mut wiringPiNodeStruct,
}
#[test]
fn bindgen_test_layout_wiringPiNodeStruct() {
    assert_eq!(::std::mem::size_of::<wiringPiNodeStruct>() , 96usize , concat
               ! ( "Size of: " , stringify ! ( wiringPiNodeStruct ) ));
    assert_eq! (::std::mem::align_of::<wiringPiNodeStruct>() , 8usize , concat
                ! ( "Alignment of " , stringify ! ( wiringPiNodeStruct ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const wiringPiNodeStruct ) ) . pinBase as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( wiringPiNodeStruct ) ,
                "::" , stringify ! ( pinBase ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const wiringPiNodeStruct ) ) . pinMax as *
                const _ as usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( wiringPiNodeStruct ) ,
                "::" , stringify ! ( pinMax ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const wiringPiNodeStruct ) ) . fd as * const _
                as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( wiringPiNodeStruct ) ,
                "::" , stringify ! ( fd ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const wiringPiNodeStruct ) ) . data0 as * const
                _ as usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( wiringPiNodeStruct ) ,
                "::" , stringify ! ( data0 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const wiringPiNodeStruct ) ) . data1 as * const
                _ as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( wiringPiNodeStruct ) ,
                "::" , stringify ! ( data1 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const wiringPiNodeStruct ) ) . data2 as * const
                _ as usize } , 20usize , concat ! (
                "Alignment of field: " , stringify ! ( wiringPiNodeStruct ) ,
                "::" , stringify ! ( data2 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const wiringPiNodeStruct ) ) . data3 as * const
                _ as usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( wiringPiNodeStruct ) ,
                "::" , stringify ! ( data3 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const wiringPiNodeStruct ) ) . pinMode as *
                const _ as usize } , 32usize , concat ! (
                "Alignment of field: " , stringify ! ( wiringPiNodeStruct ) ,
                "::" , stringify ! ( pinMode ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const wiringPiNodeStruct ) ) . pullUpDnControl
                as * const _ as usize } , 40usize , concat ! (
                "Alignment of field: " , stringify ! ( wiringPiNodeStruct ) ,
                "::" , stringify ! ( pullUpDnControl ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const wiringPiNodeStruct ) ) . digitalRead as *
                const _ as usize } , 48usize , concat ! (
                "Alignment of field: " , stringify ! ( wiringPiNodeStruct ) ,
                "::" , stringify ! ( digitalRead ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const wiringPiNodeStruct ) ) . digitalWrite as
                * const _ as usize } , 56usize , concat ! (
                "Alignment of field: " , stringify ! ( wiringPiNodeStruct ) ,
                "::" , stringify ! ( digitalWrite ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const wiringPiNodeStruct ) ) . pwmWrite as *
                const _ as usize } , 64usize , concat ! (
                "Alignment of field: " , stringify ! ( wiringPiNodeStruct ) ,
                "::" , stringify ! ( pwmWrite ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const wiringPiNodeStruct ) ) . analogRead as *
                const _ as usize } , 72usize , concat ! (
                "Alignment of field: " , stringify ! ( wiringPiNodeStruct ) ,
                "::" , stringify ! ( analogRead ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const wiringPiNodeStruct ) ) . analogWrite as *
                const _ as usize } , 80usize , concat ! (
                "Alignment of field: " , stringify ! ( wiringPiNodeStruct ) ,
                "::" , stringify ! ( analogWrite ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const wiringPiNodeStruct ) ) . next as * const
                _ as usize } , 88usize , concat ! (
                "Alignment of field: " , stringify ! ( wiringPiNodeStruct ) ,
                "::" , stringify ! ( next ) ));
}
impl Clone for wiringPiNodeStruct {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    #[link_name = "wiringPiNodes"]
    pub static mut wiringPiNodes: *mut wiringPiNodeStruct;
}
extern "C" {
    pub fn wiringPiFailure(fatal: ::std::os::raw::c_int,
                           message: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wiringPiFindNode(pin: ::std::os::raw::c_int)
     -> *mut wiringPiNodeStruct;
}
extern "C" {
    pub fn wiringPiNewNode(pinBase: ::std::os::raw::c_int,
                           numPins: ::std::os::raw::c_int)
     -> *mut wiringPiNodeStruct;
}
extern "C" {
    pub fn wiringPiSetup() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wiringPiSetupSys() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wiringPiSetupGpio() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wiringPiSetupPhys() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pinModeAlt(pin: ::std::os::raw::c_int,
                      mode: ::std::os::raw::c_int);
}
extern "C" {
    pub fn pinMode(pin: ::std::os::raw::c_int, mode: ::std::os::raw::c_int);
}
extern "C" {
    pub fn pullUpDnControl(pin: ::std::os::raw::c_int,
                           pud: ::std::os::raw::c_int);
}
extern "C" {
    pub fn digitalRead(pin: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn digitalWrite(pin: ::std::os::raw::c_int,
                        value: ::std::os::raw::c_int);
}
extern "C" {
    pub fn pwmWrite(pin: ::std::os::raw::c_int, value: ::std::os::raw::c_int);
}
extern "C" {
    pub fn analogRead(pin: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn analogWrite(pin: ::std::os::raw::c_int,
                       value: ::std::os::raw::c_int);
}
extern "C" {
    pub fn wiringPiSetupPiFace() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wiringPiSetupPiFaceForGpioProg() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn piBoardRev() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn piBoardId(model: *mut ::std::os::raw::c_int,
                     rev: *mut ::std::os::raw::c_int,
                     mem: *mut ::std::os::raw::c_int,
                     maker: *mut ::std::os::raw::c_int,
                     overVolted: *mut ::std::os::raw::c_int);
}
extern "C" {
    pub fn wpiPinToGpio(wpiPin: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn physPinToGpio(physPin: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setPadDrive(group: ::std::os::raw::c_int,
                       value: ::std::os::raw::c_int);
}
extern "C" {
    pub fn getAlt(pin: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pwmToneWrite(pin: ::std::os::raw::c_int,
                        freq: ::std::os::raw::c_int);
}
extern "C" {
    pub fn digitalWriteByte(value: ::std::os::raw::c_int);
}
extern "C" {
    pub fn digitalReadByte() -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn pwmSetMode(mode: ::std::os::raw::c_int);
}
extern "C" {
    pub fn pwmSetRange(range: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn pwmSetClock(divisor: ::std::os::raw::c_int);
}
extern "C" {
    pub fn gpioClockSet(pin: ::std::os::raw::c_int,
                        freq: ::std::os::raw::c_int);
}
extern "C" {
    pub fn waitForInterrupt(pin: ::std::os::raw::c_int,
                            mS: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wiringPiISR(pin: ::std::os::raw::c_int,
                       mode: ::std::os::raw::c_int,
                       function:
                           ::std::option::Option<unsafe extern "C" fn()>)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn piThreadCreate(fn_:
                              ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                             *mut ::std::os::raw::c_void)
                                                        ->
                                                            *mut ::std::os::raw::c_void>)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn piLock(key: ::std::os::raw::c_int);
}
extern "C" {
    pub fn piUnlock(key: ::std::os::raw::c_int);
}
extern "C" {
    pub fn piHiPri(pri: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn delay(howLong: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn delayMicroseconds(howLong: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn millis() -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn micros() -> ::std::os::raw::c_uint;
}
