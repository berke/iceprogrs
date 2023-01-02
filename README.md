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

## Author

Berke DURAK <bd@exhrd.fr>
