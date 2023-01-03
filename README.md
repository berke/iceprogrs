# iceprogrs 

SPI-based SRAM programmer for Lattice iCE40 family FPGAs.

## Description

This works like `iceprog` but does not require an FTDI chip as
provided on the usual development boards.  It uses GPIO lines and an
SPI device through the `spidev` and `gpiochip` Linux kernel drivers,
using the `spidev` and `gpiod-rs` crates.

## Usage

You need the following:
- An SPI port driven by `spidev`, of which
  - The `SCK` (clock) line is connected to the `SPI_SCK` pin of the iCE40
  - The `MOSI` (or data out) line is connected to the `SPI_SI` pin of the iCE40
- Two GPIO lines
  - One connected to the `SS_B` pin
  - One connected to the `CREST` pin

The SPI slave select line is NOT used because that signal needs to
stay low during the transfers.  In normal SPI usage, the slave select
line is released between packets (bytes).

However on many devices you can configure the pins you see fit, so you
could use a pin labeled `SS` as a GPIO pin.

## Example using a Beaglebone Black and the Lattice iCE40HX8K breakout board

### Modifications to the board

- Remove all jumpers
- Remove R1,R4,R5 and R6 using a hot air gun or heated tweezers
- Solder +5V and ground wires.  There is a large 5V trace under the USB
  connector on the opposide side of the PCB, scrape the solder mask
  to get to the copper.  Find a convenient ground point.
- Solder wires to the northmost pads of R1 (to get `SCK`), R4 (to get `SS_B`),
  R5 (to get `CDONE`) and R6 (to get `CREST`)
- Pull up `CDONE` to VCCIO (the voltage used for SPI, typically 3.3V)
- Connect `SDO` to pin 4 of J6 (Northwest)

### Pin configuration

Configure your pins as follows.  I recommend placing 100 to 150 ohm
series resistors for protection and signal integrity near the
Beaglebone.

```
config-pin P9.17 gpio
config-pin P9.18 spi
config-pin P9.21 spi
config-pin P9.22 spi_sclk
config-pin P9.23 gpio
```

Connect them as follows:

| BBone pin | Type   | iCE40 pin |
|-----------|--------|-----------|
| P9.17     | GPIO   | `SS_B`    |
| P9.18     | SPI    | `SDO`     |
| P9.22     | SPI    | `SCK`     |
| P9.23     | GPIO   | `CREST`   |
| P9.2      | Ground | `GND`     |

Run the command:

```
./iceprogrs --bitstream blinky.bin --spi-speed 5000000
```

Works at 5 MHz with 20 to 30 cm worth of improvised wires.

## Author

Berke DURAK <bd@exhrd.fr>
