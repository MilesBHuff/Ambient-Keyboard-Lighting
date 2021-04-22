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

////////////////////////////////////////////////////////////////////////////////
#[derive(StructOpt, Debug)]
#[structopt(name = "ambient-kb")]
struct ArgStruct {

    /// Prints the color being assigned to the keyboard
    #[structopt(short, long)]
    verbose: bool,

    /// Only processes every n pixels
    #[structopt(short, long, default_value = "30")] // 30 works well at 1080p.  Causes flickering when not a multiple of both display axes.  Causes flickering when set too high.
    divisor: usize,

    /// Runs this many times per second
    #[structopt(short, long, default_value = "18")] // 20 is smooth.  Any lower risks creating a strobing effect.  Everyone's eyes are different;  YMMV.
    fps: f32,

    /// The priority to run at
    #[structopt(short, long, default_value = "19")] // 19 is the highest niceness possible
    niceness: i32,
}

////////////////////////////////////////////////////////////////////////////////
fn main() {
    let args = ArgStruct::from_args(); // Get input

    //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //
    // Colors

    let color_channels: usize = 3;                        // Could be a u8 if it weren't used for array indexing.
    let color_channels_index: usize = color_channels - 1; // Could be a u8 if it weren't used for array indexing.

    let mut color_totals   = [0u32, 0u32, 0u32]; // Theoretical maximum of 528,768,000 for 1920x1080;  so a large integer (ie, u32) is needed.
    let mut color_averages = [0u8,  0u8,  0u8 ]; // Theoretical maximum of 256 each;  so we only need 8 bits.

    debug_assert_eq!(color_totals.len(),   color_channels);
    debug_assert_eq!(color_averages.len(), color_channels);

    //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //
    // Display

    let display = Display::primary().expect("Failed to load primary display.");
    let mut capturer = Capturer::new(display).expect("Failed to capture screenshot.");

    struct Dim {
        w: usize,
        h: usize,
    }
    let dim = Dim {
        w: capturer.width()  / args.divisor,
        h: capturer.height() / args.divisor,
    };
    let pixels = dim.w * dim.h; // Theoretical maximum of 2,073,600 for 1920x1080;  so a large integer (ie, u32) is needed.

    //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //
    // Strides

    struct Stride {
        h: usize,
        w: usize,
        x: usize,
        y: usize,
        s: usize,
    }
    let mut stride = Stride {
        h: 0,
        w: 4 * args.divisor,
        x: 0,
        y: 0,
        s: 0,
    };

    //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //
    // Misc

    let frequency = Duration::from_millis((1000.0 / args.fps).round() as u64); // Set update frequency
    unsafe {setpriority(PRIO_PROCESS, getpid() as u32, args.niceness);} // Reduce priority

    ////////////////////////////////////////////////////////////////////////////////
    loop {
        //TODO: Set keyboard backlight brightness to display backlight brightness

        //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //
        // Take a screenshot
        match capturer.frame() {
            Ok(buffer) => {

                // Reset certain re-used variables
                color_totals   = [0, 0, 0];
                color_averages = [0, 0, 0];

                // Loop through the screenshot
                stride.h = buffer.len() / dim.h;
                for y in 0..dim.h {
                    stride.y = stride.h * y;
                    for x in 0..dim.w {
                        stride.x = stride.w * x;

                        // Total the pixels
                        stride.s = stride.x + stride.y;
                        for i in 0..color_channels {
                            color_totals[color_channels_index - i] += buffer[stride.s + i] as u32;
                        }
                    }
                }

                // Average the totals
                for i in 0..color_channels {
                    color_averages[i] = (color_totals[i] as f32 / pixels as f32).round() as u8;
                }

                // Convert to hex and send to acpi
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
