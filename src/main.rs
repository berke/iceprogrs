mod progress;

use std::fs::File;
use std::io::prelude::*;
use spidev::{Spidev,SpidevOptions,SpiModeFlags};
use gpiod::{Chip,Options};
use pico_args::Arguments;

use progress::ProgressIndicator;

type Res<T> = Result<T,Box<dyn std::error::Error>>;

fn delay_ms(t:u64) {
    std::thread::sleep(std::time::Duration::from_millis(t));
}

fn main()->Res<()> {
    let mut args = Arguments::from_env();

    let bs_path : String = args.value_from_str("--bitstream")?;
    let spi_path : String = args.opt_value_from_str("--spi")?
	.unwrap_or_else(|| "/dev/spidev0.0".to_string());
    let spi_speed : u32 = args.opt_value_from_str("--spi-speed")?
	.unwrap_or(5_000_000);
    let ss_chip : String = args.opt_value_from_str("--ss-gpio")?
	.unwrap_or_else(|| "gpiochip0".to_string());
    let ss_pin : u32 = args.opt_value_from_str("--ss-pin")?
	.unwrap_or(5);
    let reset_chip : String = args.opt_value_from_str("--reset-gpio")?
	.unwrap_or_else(|| "gpiochip1".to_string());
    let reset_pin : u32 = args.opt_value_from_str("--reset-pin")?
	.unwrap_or(17);

    println!("Loading bitstream from {}",bs_path);
    let mut fd = File::open(bs_path)?;
    let mut dat = Vec::new();
    fd.read_to_end(&mut dat)?;
    let m = dat.len();
    println!("Bitstream size: {} B",m);
    
    println!("Opening SPI device {} at {} Hz",spi_path,spi_speed);
    let mut spi = Spidev::open(spi_path)?;
    let options = SpidevOptions::new()
         .bits_per_word(8)
         .max_speed_hz(spi_speed)
         .mode(SpiModeFlags::SPI_MODE_0)
         .build();
    spi.configure(&options)?;

    println!("SS_B is pin {} on {}",ss_pin,ss_chip);
    let ss_chip = Chip::new(ss_chip)?;
    let ss_opts = Options::output([ss_pin])
	.values([true])
	.consumer("SS_B");
    let ss = ss_chip.request_lines(ss_opts)?;

    println!("CREST is pin {} on {}",reset_pin,reset_chip);
    let reset_chip = Chip::new(reset_chip)?;
    let reset_opts = Options::output([reset_pin])
	.values([true])
	.consumer("CREST");
    let reset = reset_chip.request_lines(reset_opts)?;

    // Program
    println!("Programming FPGA");
    let mut prog = ProgressIndicator::new("Programming FPGA",m);
    reset.set_values(1_u8)?;
    ss.set_values(1_u8)?;

    delay_ms(500);
    ss.set_values(0_u8)?;
    reset.set_values(0_u8)?;
    delay_ms(500);
    reset.set_values(1_u8)?;
    delay_ms(500);

    let mut k = 0;
    for ch in dat.chunks(512) {
	prog.update(k);
	k += ch.len();
	spi.write(ch)?;
    }
    spi.write(&[0;7])?;

    delay_ms(100);
    println!("Done");
    Ok(())
}
