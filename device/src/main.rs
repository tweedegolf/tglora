#![no_main]
#![no_std]

use cortex_m::prelude::_embedded_hal_blocking_delay_DelayUs;
use lora_e5_bsp::RfSwitch;
use lorawan::device::error::DeviceError;
use lorawan::device::{Credentials, Device};
use lorawan::lorawan::{AppEui, AppKey, DevEui, MAX_PACKET_SIZE};
use lorawan::radio::{LoRaRadio, EU868};
use stm32wlxx_hal::dma::AllDma;
use stm32wlxx_hal::gpio::{pins, Exti, ExtiTrg, Input, PortA, Pull};
use stm32wlxx_hal::pac::interrupt;
use stm32wlxx_hal::rng::{Clk, Rng};
use stm32wlxx_hal::subghz::{Ocp, RfSwTx, SubGhz, Sx126x, SxConfig};
use stm32wlxx_hal::tim::Tim2;
use stm32wlxx_hal::{pac, rcc};

use device as _;

// TODO: Change to your own credentials
const APP_EUI: AppEui = AppEui::new(0x0000000000000000);
const DEV_EUI: DevEui = DevEui::new(0xFFFFFFFFFFFFFFFF);
const APP_KEY: AppKey = AppKey::new(0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA);

const SX_CONFIG: SxConfig = SxConfig::new()
    .set_pa_config(RfSwitch::PA_CONFIG)
    .set_pa_ocp(Ocp::Max140m)
    .set_tx_params(RfSwitch::TX_PARAMS);

#[cortex_m_rt::entry]
fn main() -> ! {
    // Retrieve peripherals
    let radio = cortex_m::interrupt::free(|cs| {
        let mut dp: pac::Peripherals = defmt::unwrap!(pac::Peripherals::take());

        let gpioa = PortA::split(dp.GPIOA, &mut dp.RCC);
        let dma = AllDma::split(dp.DMAMUX, dp.DMA1, dp.DMA2, &mut dp.RCC);

        let rfs = RfSwitch::new(gpioa.a4, gpioa.a5, cs);
        let subghz = SubGhz::new_with_dma(dp.SPI3, dma.d1.c1, dma.d2.c1, &mut dp.RCC);
        let radio = Sx126x::new(subghz, rfs, SX_CONFIG);
        let tim2 = Tim2::new(dp.TIM2, &mut dp.RCC);
        let rng = Rng::new(dp.RNG, Clk::MSI, &mut dp.RCC);

        // Button interrupt
        let _ = Input::new(gpioa.a0, Pull::Up, cs);
        pins::A0::setup_exti_c1(&mut dp.EXTI, &mut dp.SYSCFG, ExtiTrg::Falling);
        unsafe { pins::A0::unmask() };

        LoRaRadio::new(radio, tim2, rng)
    });

    let credentials: Credentials = Credentials::new(APP_EUI, DEV_EUI, APP_KEY);

    // Keep trying to join the network until successful
    let mut not_joined = Device::new_otaa(radio, credentials);
    let mut device = loop {
        match not_joined.join::<EU868>() {
            Ok(device) => break device.into_class_a(),
            Err(DeviceError::Join(device)) => {
                not_joined = device;
                not_joined
                    .as_mut_lora_radio()
                    .as_mut_tim()
                    .delay_us(10_000_000);
            }
            Err(e) => panic!("{:?}", e),
        }
    };

    // Create a buffer to store responses in
    let mut buf = [0; MAX_PACKET_SIZE];

    loop {
        // Wait for button press
        cortex_m::asm::wfi();

        // Transmit a message and print the response
        match device
            .transmit("Hello, world!".as_bytes(), &mut buf)
            .expect("failed to transmit")
        {
            Some((size, _)) => defmt::println!("response: {:?}", buf[0..size]),
            None => defmt::println!("no response"),
        }
    }
}

#[interrupt]
#[allow(non_snake_case)]
fn EXTI0() {
    pins::A0::clear_exti();
}
