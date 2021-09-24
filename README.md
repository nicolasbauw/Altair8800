# Teletype Emulator

[![Current Crates.io Version](https://img.shields.io/crates/v/teletype.svg)](https://crates.io/crates/teletype)
[![Current docs Version](https://docs.rs/teletype/badge.svg)](https://docs.rs/teletype)
[![Downloads badge](https://img.shields.io/crates/d/teletype.svg)](https://crates.io/crates/teletype)

This is a teletype emulator, written for my [8080 emulator](https://crates.io/crates/intel8080).
It emulates a teletype interfaced on a 88-SIO board (MITS/Altair)

```text
cargo run -- bin/teletype_echo.bin
```

or
```
./teletype teletype_echo.bin
```

It has been tested with the echo test routine and the altair basic 3.2.