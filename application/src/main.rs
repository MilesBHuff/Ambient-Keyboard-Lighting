////////////////////////////////////////////////////////////////////////////////
use std::assert_eq;
use std::fs;
use std::io::ErrorKind::WouldBlock;
use std::mem::drop;
use std::thread;
use std::time::Duration;

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

    /// Runs this many times per second
    #[structopt(short, long, default_value = "4")] // 4fps is 250ms, which is around the average adult human reaction time.
    fps: f32,

    /// Only processes every n pixels
    #[structopt(short, long, default_value = "4")] // 4 seems reasonable.
    divisor: usize,
}

////////////////////////////////////////////////////////////////////////////////
fn main() {
    //TODO: Set niceness to 20.

    // Get input
    let args = ArgStruct::from_args();
    let divisor = args.divisor;
    let fps     = args.fps;
    let verbose = args.verbose;
    drop(args);

    //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //

    // Colors
    let color_channels = 3usize;
    let mut color_totals   = [0u32, 0u32, 0u32]; // Theoretical maximum of 528,768,000 for 1920x1080;  so a large integer (ie, u32) is needed.
    let mut color_averages = [0u8,  0u8,  0u8];
    assert_eq!(color_totals.len(),   color_channels);
    assert_eq!(color_averages.len(), color_channels);

    // Display
    let display = Display::primary().expect("Failed to load primary display.");
    let mut capturer = Capturer::new(display).expect("Failed to capture screenshot.");
    struct Dim {
        w: usize,
        h: usize,
    } let dim = Dim {
        w: capturer.width()  / divisor,
        h: capturer.height() / divisor,
    };
    let pixels = dim.w * dim.h; // Theoretical maximum of 2,073,600 for 1920x1080;  so a large integer (ie, u32) is needed.

    // Core loop
    let frequency = Duration::from_millis((1000.0 / fps).round() as u64);
    // drop(fps);
    loop {
        thread::sleep(frequency);

        // Take a screenshot
        match capturer.frame() {
            Ok(buffer) => {

                // Loop through the screenshot
                let stride = buffer.len() / dim.h;
                for y in 0..dim.h {
                    let y_stride = stride * y;
                    for x in 0..dim.w {
                        let x_stride = x * 4 * divisor;
                        let xy_stride = x_stride + y_stride;
                        // drop(x_stride);

                        // Total the pixels
                        for i in 0..color_channels {
                            color_totals[(color_channels - 1) - i] += buffer[xy_stride + i] as u32;
                        }
                    }
                }

                // Average the totals
                for i in 0..color_channels {
                    color_averages[i] = (color_totals[i] as f32 / pixels as f32).round() as u8;
                }

                // Convert to hex and send to acpi
                let hex: String = format!("{:02x}{:02x}{:02x}", color_averages[0], color_averages[1], color_averages[2]);
                // Command::new("sys76-kb").arg("set").arg("-c").arg(format!("{}", hex)).spawn().expect("Error while executing `sys76-kb`.");
                fs::write("/sys/class/leds/system76_acpi::kbd_backlight/color", hex.to_string()).expect("Unable to set keyboard color.");

                // Debug text
                if verbose {
                    println!("{} {}",
                        format!("#{}", hex),
                        format!("[{:03}, {:03}, {:03}]", color_averages[0], color_averages[1], color_averages[2]),
                    );
                }
            },
            Err(error) => {
                if error.kind() != WouldBlock {
                    panic!("Error: {}", error);
                }
            },
        };

        // Reset re-used variables
        color_totals   = [0, 0, 0];
        color_averages = [0, 0, 0];

        //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //  //

        //TODO: Set keyboard backlight brightness to display backlight brightness
    }
}
