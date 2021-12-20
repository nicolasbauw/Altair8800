# Altair 8800 / Teletype Emulator

[![Current Crates.io Version](https://img.shields.io/crates/v/teletype.svg)](https://crates.io/crates/teletype)
[![Downloads badge](https://img.shields.io/crates/d/teletype.svg)](https://crates.io/crates/teletype)

This is an Altair 8800 / 88-SIO / teletype emulator, written with my [8080 emulator](https://crates.io/crates/intel8080) library.
The 8080 processor speed is 2.1 Mhz.

To install:
```text
cargo install teletype
```
or go to the releases section of this github page. (Windows or Apple universal binary)

You can run an Altair binary, for example BASIC 3.2 for which this program has been developed:
```
teletype 4kbas32.bin
```

```
MEMORY SIZE? 
TERMINAL WIDTH? 
WANT SIN? Y

62166 BYTES FREE

BASIC VERSION 3.2
[4K VERSION]

OK
```
The escape key opens a menu, from which you can:
- Quit the emulator without having to press CTRL-C
- Load a file from your disk. This "injects" the text of the file into the teletype and is very convenient to load a BASIC program, since BASIC 3.2 does not provide disk operation commands.

Pressing ESC a second time quits the menu.


On [this page](https://altairclone.com/downloads/basic/BASIC%20Programs/4K%20BASIC/) you will find several basic programs. I personally love seawar4k.bas and lander4k.bas.