# Ambient Keyboard Lighting

## About

This program calculates the average color of your display, and sets the keyboard to match it.

Currently, it only works for certain System76 computers, and requires [system76-acpi-dkms](https://github.com/pop-os/system76-acpi-dkms) to be installed.

It's also my first-ever Rust program.  Please let me know where I can improve!

## Usage

Treat it like any normal cargo app.  Note, however, that you have to run this as `root` or it won't work, since it needs write access to a file in `/sys`.

### Helptext

```
ambient-kb 1.0.0

USAGE:
    ambient-kb [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Prints the color being assigned to the keyboard

OPTIONS:
    -d, --divisor <divisor>    Only processes every n pixels [default: 4]
    -f, --fps <fps>            Runs this many times per second [default: 4]
```

## Copyright

Copyright (C) from 2021 by Miles B Huff, per the terms of the Affero General Public License (v3.0 or later).
