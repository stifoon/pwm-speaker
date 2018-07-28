#![no_main]
#![no_std]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32f103xx_hal as hal;
extern crate pwm_speaker;

use rt::ExceptionFrame;
use hal::prelude::*;
use hal::delay::Delay;

entry!(main);

fn main() -> ! {
    let dp = hal::stm32f103xx::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(cp.SYST, clocks);
    let c1 = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
    let mut pwm = dp.TIM2.pwm(c1, &mut afio.mapr, 440.hz(), clocks, &mut rcc.apb1);
    pwm.enable();
    let mut speaker = pwm_speaker::Speaker::new(pwm, clocks);
    loop {
        use pwm_speaker::songs::*;
        speaker.play_score(&BATEAU_SUR_LEAU, 80 / 4, &mut delay);
        delay.delay_ms(1000u32);
        speaker.play_score(&FRERE_JACQUES, 140 / 4, &mut delay);
        delay.delay_ms(1000u32);
        speaker.play_score(&LAVENTURIER, 160 / 4, &mut delay);
        delay.delay_ms(1000u32);
        speaker.play_score(&MARIO_THEME_INTRO, 185 / 4, &mut delay);
        delay.delay_ms(1000u32);
        speaker.play_score(&THIRD_KIND, 120 / 4, &mut delay);
        delay.delay_ms(1000u32);
    }
}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
