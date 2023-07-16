# Ambient Keyboard Lighting
[![build result](https://build.opensuse.org/projects/home:MilesBHuff:Official/packages/ambient-kb-git/badge.svg?type=default)](https://build.opensuse.org/package/show/home:MilesBHuff:Official/ambient-kb-git)

## About

This program calculates the average color of your display, and sets the keyboard to match it.

Currently, it only works for certain System76 computers, and requires certain System76-specific interfaces to be exposed in sysfs.

It's also my first-ever Rust program.  Please let me know where I can improve!

Note that this application has optional support for systemd and AppArmor.

## How do I get this on my computer?

### Supported distros

- **Arch Linux:** Use `packaging/arch-linux/PKGBUILD` to compile and install this package like anything else you'd find in the AUR.
- **OpenSUSE:** Use [OBS](https://build.opensuse.org/package/show/home:MilesBHuff:Official/ambient-kb-git) or the files in `packaging/opensuse/`.

### Manually

#### Building

Make sure you have `rust` and `cargo` installed.

```
cd 'application'
cargo build --release
```

#### Installation

Copy `ambient-kb` to `/usr/local/bin`.

If your system uses systemd, copy `configuration/systemd/ambient-kb.service` to `/etc/systemd/system/`.

If you have AppArmor installed, copy `configuration/apparmor/usr.bin.ambient-kb` to `/etc/apparmor.d/`.

## Usage

Make sure you either (a) have the Pop!_OS kernel *or* (b) have [`system76-acpi-dkms`](https://github.com/pop-os/system76-acpi-dkms) installed.  This application depends on interfaces that these expose in sysfs.
You also *have* to be running an X11 session;  this application depends on X to grab the screen, which it needs to do in order to calculate the color of your keyboard.

Execute the binary you installed, as root:  `ambient-kb`.
(You *have* to run this as root or it won't work, since it needs write access to a "file" in `/sys`.)

### Helptext

```
ambient-kb X.Y.Z

USAGE:
    ambient-kb [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Prints the color being assigned to the keyboard

OPTIONS:
    -d, --divisor <divisor>      Only processes every n pixels [default: 30]
    -f, --fps <fps>              Runs this many times per second [default: 20]
    -n, --niceness <niceness>    The priority to run at [default: 19]
```

### systemd

This application comes with a systemd service that allows you to automatically start this application at boot.

Unfortunately, the way this is currently implemented is... not great, so it may not work for you;  and it will ONLY work for single-user systems at present:  so if you have anything else under `/home/`, don't enable this service.  That said, if you *do* have a single-user system and want to try it out, just run the following as root:  `systemctl enable ambient-kb`.

### AppArmor

The AppArmor profile you installed should automatically take effect if you have AppArmor installed and set to "enforcing".

This profile will prevent this application from accessing any file other than the handful I have explicitly enumerated;  thus dramatically mitigating the impact of any potential security vulnerabilities.  This is important for any application you're planning to run 24/7 as root.

## Copyright

Copyright (C) from 2021 by Miles B Huff, per the terms of the Lesser Affero General Public License (v3.0 or later).
