# Altair 8800 / Teletype Emulator

[![Current Crates.io Version](https://img.shields.io/crates/v/teletype.svg)](https://crates.io/crates/teletype)
[![Downloads badge](https://img.shields.io/crates/d/teletype.svg)](https://crates.io/crates/teletype)

This is an Altair 8800 / 88-SIO / teletype emulator, written with my [8080 emulator](https://crates.io/crates/intel8080) library.
The 8080 processor speed is 2.1 Mhz.

To install:

```text
cargo install teletype
```

You can configure the ROM file and the amount of RAM you want in the config file:
~/.config/teletype/config.toml

Example for Microsoft Basic 3.2:

```text
[memory]
rom = "/Users/nicolasb/4kbas32.bin"
ram = 0xFFFF
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
- Auto-type a BASIC file from your disk. This "injects" the text of the file into the teletype and is very convenient to load a BASIC program, since BASIC 3.2 does not provide disk operation commands.
- Load or save a snapshot (altair.snapshot)

Pressing ESC a second time quits the menu.  

Bash has an issue that deactivates terminal echo when the program quits (I noticed the same behavior with other programs that use the console crate). Two solutions:
- type ```stty echo``` after quit
- use zsh

On [this page](https://altairclone.com/downloads/basic/BASIC%20Programs/4K%20BASIC/) you will find several basic programs. I personally love seawar4k.bas and lander4k.bas.
