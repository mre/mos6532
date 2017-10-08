# MOS 6532

A pure Rust implementation of the MOS 6532 processor (also known as `PIA` or `RIOT`).

It is a Peripheral Interface Adaptor which has three functions: a programmable timer, 128 bytes of RAM, and two 8 bit parallel I/O ports. 

It was used in the Atari 2600 as a coprocessor to read pedal input.

(Adapted from the [Stella Programming Manual]( https://alienbill.com/2600/101/docs/stella.html#pia1.0))


In the Atari 2600, the PIA uses the same clock as the microprocessor so that one PIA cycle occurs for each machine cycle. The PIA can be set for one of four different "intervals", where each interval is some multiple of the clock (and therefore machine cycles).

A value from 1 to 255 is loaded into the PIA which will be decremented by one at each interval . The timer can now be read by the microprocessor to determine elapsed time for timing various software operations and keep them synchronized with the hardware (TIA chip). ([Source](http://atarihq.com/danb/files/stella.pdf)) 

## Resources

* [Atari 2600 specification](http://problemkaputt.de/2k6specs.htm)
* [Stella Programmers guide](http://atarihq.com/danb/files/stella.pdf)
* [MOS 6532 datasheet](http://archive.6502.org/datasheets/mos_6532_riot.pdf) - Detailed but hard to read.
* [Picture of the Atari 2600 board, which used the 6532](http://www.hardwaresecrets.com/inside-the-atari-2600/3/) 
