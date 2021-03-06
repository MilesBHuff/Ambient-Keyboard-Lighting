// Copyright © from 2021 by Miles B Huff <MilesBHuff@Users.NoReply.Github.com> per the terms of the GNU LAGPL 3.

////////////////////////////////////////////////////////////////////////////////
use std::fs;
use std::io::ErrorKind::WouldBlock;
use std::thread;
use std::time::Duration;

//  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //
extern crate libc;
use libc::{setpriority, PRIO_PROCESS, getpid};

//  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //
extern crate scrap;
use scrap::{Capturer, Display};

//  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //
extern crate structopt;
use structopt::StructOpt;

//  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //
#[macro_use]
mod utils;

////////////////////////////////////////////////////////////////////////////////
#[derive(StructOpt, Debug)]
#[structopt(name = "ambient-kb")]
struct ArgStruct { // Remember to order the struct such that it will not use more memory than it needs.

    /// Runs this many times per second
    #[structopt(short, long, default_value = "20")] // 20 is smooth.  Any lower risks creating a strobing effect.  Everyone's eyes are different;  YMMV.
    fps: f32,

    /// The priority to run at
    #[structopt(short, long, default_value = "19")] // 19 is the highest niceness possible.
    niceness: i8,

    /// Only processes every n pixels
    #[structopt(short, long, default_value = "30")] // 30 works well at 1080p.  Causes flickering when not a multiple of both display axes.  Causes flickering when set too high.
    divisor: u8,

    /// Prints the color being assigned to the keyboard
    #[structopt(short, long)]
    verbose: bool,
}

////////////////////////////////////////////////////////////////////////////////
fn main() {
    let args = ArgStruct::from_args(); // Get input
    assert!(args.niceness >= -20 && args.niceness < 20);

    //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //
    // Colors

    let color_channels: u8 = 3;
    let color_channels_index: usize = (color_channels - 1) as usize; // Could be a u8 if it weren't used for array indexing.

    let mut color_totals   = [0u32; 3]; // Theoretical maximum of 528,768,000 for 1920x1080;  so a large integer (ie, u32) is needed.
    let mut color_averages = [0u8;  3]; // Theoretical maximum of 256 each;  so we only need 8 bits.

    debug_assert_eq!(color_totals.len()   as u8, color_channels);
    debug_assert_eq!(color_averages.len() as u8, color_channels);

    //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //
    // Display

    let display = Display::primary().expect("Failed to load primary display.");
    let mut capturer = Capturer::new(display).expect("Failed to capture screenshot.");

    struct Dim {
        w: u16,
        h: u16,
    }
    let dim = Dim {
        w: rounded_integer_division!(capturer.width(),  args.divisor as usize) as u16,
        h: rounded_integer_division!(capturer.height(), args.divisor as usize) as u16,
    };
    let pixels: u32 = (dim.w as u32) * (dim.h as u32); // Theoretical maximum of 2,073,600 for 1920x1080;  so a large integer (ie, u32) is needed.

    //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //
    // Strides

    struct Stride {
        h: usize,
        w: u16,
        x: u16,
        y: usize,
        s: usize,
    }
    let mut stride = Stride {
        h: 0,
        w: 4 * (args.divisor as u16),
        x: 0,
        y: 0,
        s: 0,
    };

    //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //
    // Misc

    let frequency = Duration::from_millis((1000.0 / args.fps).round() as u64); // Set update frequency
    unsafe {setpriority(PRIO_PROCESS, getpid() as u32, args.niceness as i32);} // Reduce priority

    ////////////////////////////////////////////////////////////////////////////////
    loop {
        //TODO: Set keyboard backlight brightness to display backlight brightness

        //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //
        // Take a screenshot
        match capturer.frame() {
            Ok(buffer) => {

                // Reset certain re-used variables
                color_totals   = [0; 3];
                color_averages = [0; 3];

                // Loop through the screenshot
                stride.h = rounded_integer_division!(buffer.len(), (dim.h as usize));
                for y in 0..dim.h {
                    stride.y = stride.h * y as usize;
                    for x in 0..dim.w {
                        stride.x = stride.w * x;

                        // Total the pixels
                        stride.s = stride.y + stride.x as usize;
                        for i in 0..color_channels as usize {
                            color_totals[color_channels_index - i] += buffer[stride.s + i] as u32;
                        }
                    }
                }

                // Average the totals
                for i in 0..color_channels as usize {
                    color_averages[i] = rounded_integer_division!(color_totals[i], pixels) as u8;
                }

                // Convert to hex and send to ACPI
                let hex: String = format!("{:02x}{:02x}{:02x}", color_averages[0], color_averages[1], color_averages[2]);
                fs::write("/sys/class/leds/system76_acpi::kbd_backlight/color", hex.to_string()).expect("Unable to set keyboard color.");

                // Debug text
                if args.verbose {
                    println!("{} {}",
                        format!("#{}", hex),
                        format!("[{:03}, {:03}, {:03}]", color_averages[0], color_averages[1], color_averages[2]),
                    );
                }

                // Pause before taking another screenshot
                thread::sleep(frequency);
            },

            //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //
            Err(error) => {
                if error.kind() != WouldBlock {
                    panic!("Error: {}", error);
                }
            },
        };
    }
}
