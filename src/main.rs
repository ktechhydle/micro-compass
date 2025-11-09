#![no_main]
#![no_std]

use core::sync::atomic::{AtomicBool, Ordering};
use cortex_m::asm;
use cortex_m_rt::entry;
use critical_section_lock_mut::LockMut;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use lsm303agr::{Lsm303agr, MagMode, MagOutputDataRate};
use micro_compass::{EAST, NORTH, SOUTH, WEST};
use microbit::{
    Board,
    display::blocking::Display,
    hal::{
        Timer, Twim, gpio, gpiote,
        pac::{self, interrupt},
    },
    pac::twim0::frequency::FREQUENCY_A,
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

static COMPASS_ACTIVE: AtomicBool = AtomicBool::new(false);
static BEEP_ENABLED: AtomicBool = AtomicBool::new(false);
static GPIOTE_PERIPHERAL: LockMut<gpiote::Gpiote> = LockMut::new();
const PERIOD: u32 = 1000 / 220;
const CYCLES: u32 = 50 / PERIOD;

//
#[interrupt]
fn GPIOTE() {
    let beep_enabled = BEEP_ENABLED.load(Ordering::Relaxed);
    let compass_enabled = COMPASS_ACTIVE.load(Ordering::Relaxed);

    GPIOTE_PERIPHERAL.with_lock(|gpiote| {
        // button a
        if gpiote.channel0().is_event_triggered() {
            COMPASS_ACTIVE.store(!compass_enabled, Ordering::Relaxed);

            rprintln!("Compass Enabled: {}", !compass_enabled);
        }

        // button b
        if gpiote.channel1().is_event_triggered() {
            BEEP_ENABLED.store(!beep_enabled, Ordering::Relaxed);

            rprintln!("Beeping Enabled: {}", !beep_enabled);
        }

        gpiote.channel0().reset_events();
        gpiote.channel1().reset_events();
    });
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut timer0 = Timer::new(board.TIMER0);

    // small delay to allow warmup
    timer0.delay_ms(150);

    // hardware
    let mut display = Display::new(board.display_pins);
    let mut speaker_pin = board.speaker_pin.into_push_pull_output(gpio::Level::Low);
    let button_a = board.buttons.button_a.into_floating_input();
    let button_b = board.buttons.button_b.into_floating_input();
    let mut sensor = Lsm303agr::new_with_i2c(Twim::new(
        board.TWIM0,
        board.i2c_internal.into(),
        FREQUENCY_A::K100,
    ))
    .into_mag_continuous()
    .ok()
    .unwrap();
    sensor.init().unwrap();
    sensor
        .set_mag_mode_and_odr(
            &mut timer0,
            MagMode::HighResolution,
            MagOutputDataRate::Hz50,
        )
        .unwrap();

    // interrupt for Button A & B
    // A = channel0
    // B = channel1
    let gpiote = gpiote::Gpiote::new(board.GPIOTE);
    let channel0 = gpiote.channel0();
    let channel1 = gpiote.channel1();
    channel0
        .input_pin(&button_a.degrade())
        .hi_to_lo()
        .enable_interrupt();
    channel0.reset_events();
    channel1
        .input_pin(&button_b.degrade())
        .hi_to_lo()
        .enable_interrupt();
    channel1.reset_events();

    // make the peripheral globally available
    GPIOTE_PERIPHERAL.init(gpiote);

    // handle GPIO interrupts.
    unsafe { pac::NVIC::unmask(pac::Interrupt::GPIOTE) };
    pac::NVIC::unpend(pac::Interrupt::GPIOTE);

    loop {
        if COMPASS_ACTIVE.load(Ordering::Relaxed) {
            display.clear();

            if sensor.mag_status().unwrap().xyz_new_data() {
                let (x, y, _) = sensor.magnetic_field().unwrap().xyz_nt();

                if x >= 0 && y <= 0 {
                    display.show(&mut timer0, NORTH, 100);

                    // beep only on north
                    if BEEP_ENABLED.load(Ordering::Relaxed) {
                        for _ in 0..CYCLES {
                            speaker_pin.set_high().unwrap();
                            timer0.delay_ms(PERIOD / 2);
                            speaker_pin.set_low().unwrap();
                            timer0.delay_ms(PERIOD / 2);
                        }
                    }
                } else if x >= 0 && y >= 0 {
                    display.show(&mut timer0, EAST, 100);
                } else if x <= 0 && y >= 0 {
                    display.show(&mut timer0, SOUTH, 100);
                } else if x <= 0 && y <= 0 {
                    display.show(&mut timer0, WEST, 100);
                }

                rprintln!("{}, {}", x, y);
            }
        } else {
            // wait for interrupt (saves power)
            asm::wfi();
        }
    }
}
