#![no_std]
#![no_main]

use core::fmt::Write as _;

use panic_halt as _;
use cortex_m::prelude::*;
use cortex_m_rt::entry;
use embedded_hal::{
     digital::v2::OutputPin,
};

//board imports
use adafruit_feather_rp2040::hal as hal;
use adafruit_feather_rp2040::{
    hal::{
        pac,
        pac::interrupt,
        Clock,
        Sio,
        gpio::FunctionI2C,
        uart,
        I2C,
        pio::PIOExt,
        timer::Timer,
    },
    Pins, XOSC_CRYSTAL_FREQ,
};

use fugit::RateExtU32;
use ws2812_pio::Ws2812;
use smart_leds::{RGB8, SmartLedsWrite, brightness};

use lis3dh::{Lis3dh, SlaveAddr};
use lis3dh::accelerometer::{
    orientation::Tracker,
    orientation::Orientation,
    Accelerometer,
};

// import animations
mod animations;
use animations::{
    NeoFireWork,
    NeoSpiral,
    NeoHeart,
    NeoGhost,
};


#[entry]
fn main() -> !{
    // Grab singletron objects
    let core = pac::CorePeripherals::take().unwrap();
    let mut peripherals = pac::Peripherals::take().unwrap();
    // init the watchdog timer, to pass into the clock init
    let mut watchdog = hal::watchdog::Watchdog::new(peripherals.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        peripherals.XOSC,
        peripherals.CLOCKS,
        peripherals.PLL_SYS,
        peripherals.PLL_USB,
        &mut peripherals.RESETS,
        &mut watchdog,
    ).ok().unwrap();

    // Initialize the single cycle IO
    let sio = Sio::new(peripherals.SIO);
    // Initialize the new pins to default state
    let pins = Pins::new(
        peripherals.IO_BANK0,
        peripherals.PADS_BANK0,
        sio.gpio_bank0,
        &mut peripherals.RESETS,
    );


    //Setup the Propmaker Power Enable pin
    let mut pwr_pin = pins.d10.into_push_pull_output();
    pwr_pin.set_high().unwrap();

    let timer = Timer::new(peripherals.TIMER, &mut peripherals.RESETS);
    let (mut pio, sm0, _, _, _) = peripherals.PIO0.split(&mut peripherals.RESETS);
    let mut neopixels = Ws2812::new(
        pins.d5.into_mode(),
        &mut pio,
        sm0,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );

    // Define the modes here
    let mut NeoFireWork = NeoFireWork::new();
    let mut NeoSpiral = NeoSpiral::new(RGB8::new(255,0,255));
    let mut NeoHeart = NeoHeart::new(RGB8::new(255,0,0), RGB8::new(255,0,255));
    let mut NeoGhost = NeoGhost::new();



    // Initialize the i2c and accelerometer
    // Configure 2 pins as being i2c
    let scl = pins.scl.into_mode::<FunctionI2C>();
    let sda = pins.sda.into_mode::<FunctionI2C>();

    // create the i2c driver 
    let i2c1 = I2C::i2c1(
        peripherals.I2C1,
        sda,
        scl,
        RateExtU32::kHz(400),
        &mut peripherals.RESETS,
        &clocks.system_clock,
    );

    let mut lis3dh = Lis3dh::new_i2c(i2c1, SlaveAddr::Default).unwrap();
    lis3dh.set_range(lis3dh::Range::G8).unwrap();
    let mut tracker = Tracker::new(1.0);

    let mut delay_timer = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());


    delay_timer.delay_ms(1000 as u32);

    // initialize the device orientation
    let mut orientation: Orientation = Orientation::Unknown;
    let mut mode: u8 = 0;
    let mut prev_mode = 0;
    let mut nticks: u8 = 5; // loop delay ms
    loop{
        // get acceleration vector
        let accelerVal = lis3dh.accel_norm().unwrap();
        // update tracker's internal state
        orientation = tracker.update(accelerVal);

        mode = match orientation{
            Orientation::LandscapeUp =>0,
            Orientation::PortraitUp =>1,
            Orientation::LandscapeDown =>3,
            Orientation::PortraitDown => 2,
            //other
            Orientation::Unknown=>prev_mode,
            Orientation::FaceDown=> prev_mode,
            Orientation::FaceUp => prev_mode,
        };
        prev_mode = mode;

        if nticks > 4{

            nticks = 0;
            //iterate through the applicable modes
            NeoFireWork.next();
            NeoSpiral.next();
            NeoHeart.next();
            NeoGhost.next();

            // select the list based  on current mode
            let ds: [RGB8; animations::NUM_PX] = match mode{
                0 => NeoSpiral.to_list(),
                1 => NeoFireWork.to_list(),
                2 => NeoHeart.to_list(),
                3 => NeoGhost.to_list(),
                _=> [RGB8::new(0,0,0); animations::NUM_PX],
            };

            //write to the leds
            neopixels.write(ds.iter().cloned()).unwrap();
        }

        nticks +=1;
        delay_timer.delay_ms(5 as u32);
    }
}