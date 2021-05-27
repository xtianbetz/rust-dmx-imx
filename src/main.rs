use dmx::{self, DmxTransmitter};
use std::{thread, time};

fn main() {
    println!("X gonna give it to ya!");

    let mut dmx_port = dmx::open_serial("/dev/ttymxc0").unwrap();

    let red = &[0xff, 0xff, 0x00, 0x00];
    let green = &[0xff, 0x00, 0xff, 0x00];
    let blue = &[0xff, 0x00, 0x00, 0xff];

    let mut count = 0;
    let colors = &[red, green, blue];
    let mut current_color_idx = 0;

    loop {
        let current_color = colors[current_color_idx];
        dmx_port.send_dmx_packet(current_color).unwrap();
        count += 1;
        if count % 20 == 0 {
            current_color_idx += 1;
            if current_color_idx > 2 {
                current_color_idx = 0;
            }
        }

        // repeat about every 51 ms. for more accurate frame timings,
        // consider using the ticktock crate.
        thread::sleep(time::Duration::new(0, 50_000_000));
    }
}
