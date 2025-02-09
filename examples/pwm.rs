//! Testing PWM output

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_halt;

// use cortex_m::asm;
use cortex_m_rt::entry;
use stm32l4xx_hal::{delay, prelude::*, stm32};

#[entry]
fn main() -> ! {
    let c = cortex_m::Peripherals::take().unwrap();
    let p = stm32::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let mut pwr = p.PWR.constrain(&mut rcc.apb1r1);

    let clocks = rcc.cfgr.freeze(&mut flash.acr, &mut pwr);

    let mut gpioa = p.GPIOA.split(&mut rcc.ahb2);

    // TIM2
    let c1 = gpioa
        .pa0
        .into_af1_pushpull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);
    let c2 = gpioa
        .pa1
        .into_af1_pushpull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);
    let c3 = gpioa
        .pa2
        .into_af1_pushpull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);
    let c4 = gpioa
        .pa3
        .into_af1_pushpull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);

    let mut pwm = p
        .TIM2
        .pwm((c1, c2, c3, c4), 1.khz(), clocks, &mut rcc.apb1r1)
        .3;

    let max = pwm.get_max_duty();

    pwm.enable();

    let mut timer = delay::Delay::new(c.SYST, clocks);
    let second: u32 = 100;

    // NB: if the pins are LEDs, brightness is not
    //     linear in duty value.
    loop {
        pwm.set_duty(max);
        timer.delay_ms(second);
        // asm::bkpt();

        pwm.set_duty(max / 11 * 10);
        timer.delay_ms(second);
        // asm::bkpt();

        pwm.set_duty(3 * max / 4);
        timer.delay_ms(second);
        // asm::bkpt();

        pwm.set_duty(max / 2);
        timer.delay_ms(second);
        // asm::bkpt();

        pwm.set_duty(max / 4);
        timer.delay_ms(second);
        // asm::bkpt();
    }
}
