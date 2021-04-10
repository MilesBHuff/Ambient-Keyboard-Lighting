extern crate scrap;
use scrap::{Capturer, Display};

use std::assert_eq;
// use std::fs;
use std::io::ErrorKind::WouldBlock;
use std::thread;
use std::time::Duration;
use std::process::Command;

fn main() {

    // Update frequency
    let fps = 4; // 250ms, around the average adult human reaction time.
    let spf = Duration::new(1, 0) / fps;

    // Colors
    let color_channels = 3;
    let mut color_totals = [0u32, 0u32, 0u32]; // Theoretical maximum of 528,768,000 for 1920x1080;  so a large integer (ie, u32) is needed.  Using floats to better-support division later-on.
    let mut color_averages = [0u8, 0u8, 0u8];
    assert_eq!(color_channels, color_totals.len());
    assert_eq!(color_channels, color_averages.len());

    // Display
    let display = Display::primary().expect("Failed to load primary display.");
    let mut capturer = Capturer::new(display).expect("Failed to capture screenshot.");
    struct Dim {
        w: usize,
        h: usize,
    } let dim = Dim {
        w: capturer.width(),
        h: capturer.height(),
    };
    let pixels = dim.w * dim.h; // Theoretical maximum of 2,073,600 for 1920x1080;  so a large integer (ie, u32) is needed.

    // Wait until there's a frame.
    loop {
        match capturer.frame() {
            Ok(buffer) => {

                // Reset re-used variables
                color_totals   = [0, 0, 0];
                color_averages = [0, 0, 0];

                // Total the pixels
                let stride = buffer.len() / dim.h;
                for x in 0..dim.w {
                    let x_stride = 4 * x;
                    for y in 0..dim.h {
                        let xy_stride = x_stride + (stride * y);

                        for i in 0..color_channels {
                            color_totals[i] += buffer[xy_stride + i] as u32;
                        }
                    }
                }

                // Average the pixels
                for i in 0..color_channels {
                    color_averages[i] = (color_totals[i] as f32 / pixels as f32).round() as u8;
                }

                // Convert to hex and send to sys76-kb
                let hex: String = format!("{:x}{:x}{:x}", color_averages[0], color_averages[1], color_averages[2]);
                Command::new("sys76-kb").arg("set").arg("-c").arg(format!("{}", hex)).spawn().expect("Error while executing `sys76-kb`.");
                // fs::write("/sys/class/leds/system76_acpi::kbd_backlight/color", format!("{}", hex)).expect("Unable to set keyboard color.");

                // // Debug text
                // println!("{} {}",
                //     format!("[{}, {}, {}]", color_averages[0], color_averages[1], color_averages[2]),
                //     format!("#{}", hex),
                // );
            },
            Err(error) => {
                if error.kind() != WouldBlock {
                    panic!("Error: {}", error);
                }
            },
        };

        // Ratelimit the loop
        thread::sleep(spf);
    }
}
