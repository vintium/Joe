mod color;


use std::{thread, time};

use crate::color::{HSV, RGB};

use terminal_size::{Width, Height, terminal_size};

fn get_terminal_width() -> usize {
    let size = terminal_size();
    let terminal_width = match size {
        Some((Width(w), Height(_h))) => w,
        None => 80,
    };
    terminal_width as usize
}

fn main() {
    let message = String::from("Joe ");
    let message_width = message.chars().count();
    let mut color = HSV::from(0, 1.0, 1.0);
    let color_increment = 20;
    loop {
        // update terminal width for responsive size
        let terminal_width = get_terminal_width();

        // increment hue, wrapping around at 360°
        color.h = if (color.h + color_increment) > 360 {
            0 
        } else {
            color.h + color_increment
        };

        // convert color to rgb for ANSI escape
        let rgb_color = RGB::from_HSV(&color);
        let (r, g, b) = (rgb_color.r, rgb_color.g, rgb_color.b);

        // print a line with as many instances of the message can fit in the 
        // terminal width, using the current color.
        for _ in 0..(terminal_width / (message_width)) {
            print!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, message);
        }
        println!("");
        
        // wait a bit to slow it down
        thread::sleep(time::Duration::from_millis(100));
    }
}
