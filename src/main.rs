#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate f3;
extern crate stm32f303x_memory_map;
extern crate cortex_m;

use f3::peripheral;
use cortex_m::peripheral as cm;
use stm32f303x_memory_map as stm32;

enum GpioBank {
    A,
    B,
    C,
    D,
    E,
}

enum GpioDirection {
    In,
    Out,
}

struct Gpio {
    pin: u8,
    bank: GpioBank,
}

fn get_gpio_peripheral(gpio: &Gpio) -> &mut peripheral::gpio::Gpio {
    match gpio.bank {
        GpioBank::A => unsafe { peripheral::gpioa_mut() },
        GpioBank::B => unsafe { peripheral::gpiob_mut() },
        GpioBank::C => unsafe { peripheral::gpioc_mut() },
        GpioBank::D => unsafe { peripheral::gpiod_mut() },
        GpioBank::E => unsafe { peripheral::gpioe_mut() },
    }
}

impl Gpio {
    fn new(pin: u8, bank: GpioBank, dir: GpioDirection) -> Gpio {
        let gpio = Gpio {
            pin: pin,
            bank: bank,
        };

        gpio.set_clock(true);
        gpio.set_direction(dir);
        gpio
    }

    pub fn set_clock(&self, enable: bool) {
        let rcc = unsafe { peripheral::rcc_mut() };
        match self.bank {
            GpioBank::A => rcc.ahbenr.modify(|_, w| w.iopaen(enable)),
            GpioBank::B => rcc.ahbenr.modify(|_, w| w.iopben(enable)),
            GpioBank::C => rcc.ahbenr.modify(|_, w| w.iopcen(enable)),
            GpioBank::D => rcc.ahbenr.modify(|_, w| w.iopden(enable)),
            GpioBank::E => rcc.ahbenr.modify(|_, w| w.iopeen(enable)),
        }
    }

    pub fn set_direction(&self, dir: GpioDirection) {
        let gpio = get_gpio_peripheral(self);
        let mode = match dir {
            GpioDirection::In => 0b00,
            GpioDirection::Out => 0b01,
        };

        match self.pin {
            0 => gpio.moder.modify(|_, w| w.moder0(mode)),
            1 => gpio.moder.modify(|_, w| w.moder1(mode)),
            2 => gpio.moder.modify(|_, w| w.moder2(mode)),
            3 => gpio.moder.modify(|_, w| w.moder3(mode)),
            4 => gpio.moder.modify(|_, w| w.moder4(mode)),
            5 => gpio.moder.modify(|_, w| w.moder5(mode)),
            6 => gpio.moder.modify(|_, w| w.moder6(mode)),
            7 => gpio.moder.modify(|_, w| w.moder7(mode)),
            8 => gpio.moder.modify(|_, w| w.moder8(mode)),
            9 => gpio.moder.modify(|_, w| w.moder9(mode)),
            10 => gpio.moder.modify(|_, w| w.moder10(mode)),
            11 => gpio.moder.modify(|_, w| w.moder11(mode)),
            12 => gpio.moder.modify(|_, w| w.moder12(mode)),
            13 => gpio.moder.modify(|_, w| w.moder13(mode)),
            14 => gpio.moder.modify(|_, w| w.moder14(mode)),
            15 => gpio.moder.modify(|_, w| w.moder15(mode)),
            _ => gpio.moder.modify(|_, w| w.moder0(mode)),
        }
    }

    pub fn write(&self, is_on: bool) {
        let gpio = get_gpio_peripheral(self);

        match self.pin {
            0 => gpio.bsrr.write(|w| w.br0(!is_on).bs0(is_on)),
            1 => gpio.bsrr.write(|w| w.br1(!is_on).bs1(is_on)),
            2 => gpio.bsrr.write(|w| w.br2(!is_on).bs2(is_on)),
            3 => gpio.bsrr.write(|w| w.br3(!is_on).bs3(is_on)),
            4 => gpio.bsrr.write(|w| w.br4(!is_on).bs4(is_on)),
            5 => gpio.bsrr.write(|w| w.br5(!is_on).bs5(is_on)),
            6 => gpio.bsrr.write(|w| w.br6(!is_on).bs6(is_on)),
            7 => gpio.bsrr.write(|w| w.br7(!is_on).bs7(is_on)),
            8 => gpio.bsrr.write(|w| w.br8(!is_on).bs8(is_on)),
            9 => gpio.bsrr.write(|w| w.br9(!is_on).bs9(is_on)),
            10 => gpio.bsrr.write(|w| w.br10(!is_on).bs10(is_on)),
            11 => gpio.bsrr.write(|w| w.br11(!is_on).bs11(is_on)),
            12 => gpio.bsrr.write(|w| w.br12(!is_on).bs12(is_on)),
            13 => gpio.bsrr.write(|w| w.br13(!is_on).bs13(is_on)),
            14 => gpio.bsrr.write(|w| w.br14(!is_on).bs14(is_on)),
            15 => gpio.bsrr.write(|w| w.br15(!is_on).bs15(is_on)),
            _ => gpio.bsrr.write(|w| w.br15(!is_on).bs15(is_on)),

        }
    }

    pub fn read(&self) -> bool {
        let gpio = get_gpio_peripheral(self);

        match self.pin {
            0 => gpio.idr.read().idr0(),
            1 => gpio.idr.read().idr1(),
            2 => gpio.idr.read().idr2(),
            3 => gpio.idr.read().idr3(),
            4 => gpio.idr.read().idr4(),
            5 => gpio.idr.read().idr5(),
            6 => gpio.idr.read().idr6(),
            7 => gpio.idr.read().idr7(),
            8 => gpio.idr.read().idr8(),
            9 => gpio.idr.read().idr9(),
            10 => gpio.idr.read().idr10(),
            11 => gpio.idr.read().idr11(),
            12 => gpio.idr.read().idr12(),
            13 => gpio.idr.read().idr13(),
            14 => gpio.idr.read().idr14(),
            15 => gpio.idr.read().idr15(),
            _ => false,
        }
    }
}

#[export_name = "_exti15_10"]
pub extern "C" fn my_exti15_10_interrupt_handler() {
    let exti: &mut stm32::Exti = unsafe { &mut *stm32::EXTI.get() };
    let val = exti.pr1.read().pr13().is_pending();
    exti.pr1.modify(|_, w| w.pr13().pending());

    unsafe {
        bkpt!();

    }
}

#[doc(hidden)]
#[export_name = "_init"]
pub unsafe fn init() {
    f3::itm::init();
}

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    unsafe {
        bkpt!();

    }

    let (gpioa, gpioc, rcc) =
        unsafe { (peripheral::gpioa_mut(), peripheral::gpioc_mut(), peripheral::rcc_mut()) };
    rcc.apb2enr.write(|w| w.syscfgen(true));
    let led = Gpio::new(5, GpioBank::A, GpioDirection::Out);
    let btn = Gpio::new(13, GpioBank::C, GpioDirection::In);

    let nvic = unsafe { &mut *cm::NVIC.get() };
    let exti: &mut stm32::Exti = unsafe { &mut *stm32::EXTI.get() };
    let syscfg: &mut stm32::Syscfg = unsafe { &mut *stm32::SYSCFG.get() };

    syscfg.exticr4.write(|w| w.exti13().port_c());
    exti.ftsr1.write(|w| w.tr13().enabled());
    exti.imr1.write(|w| w.mr13().enabled());

    nvic.enable(stm32f303x_memory_map::interrupt::Interrupt::Exti1510);
    nvic.set_priority(stm32f303x_memory_map::interrupt::Interrupt::Exti1510, 3);

    cortex_m::interrupt::enable();
    loop {
        led.write(!btn.read());
    }
}
