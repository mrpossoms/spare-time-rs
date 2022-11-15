// extern crate libc;
// extern crate signal_hook;

use termios::*;
use signal_hook::consts::signal::*;
use signal_hook::iterator::Signals;

use std::io::Error;

mod tg;


// extern fn sig_int_hndlr(signal: i32)
// {
//     tg::restore_settings(&mut OLD_SETTINGS);
//     std::process::exit(1);
// }

fn main() -> Result<(), std::io::Error> {
    let mut signals = Signals::new(&[SIGINT, SIGTERM, SIGQUIT, SIGWINCH])?;

    let mut cols = tg::term_width();
    let mut rows = tg::term_height();
    println!("width: {cols}, height: {rows}");

    let mut OLD_SETTINGS = tg::game_settings();

    'main: loop {
        for signal in signals.pending() {
            match signal {
                SIGWINCH => {
                    cols = tg::term_width();
                    rows = tg::term_height();
                    println!("width: {cols}, height: {rows}");
                    std::thread::sleep(std::time::Duration::new(1, 0));
                },
                SIGINT | SIGTERM | SIGQUIT => {
                    tg::restore_settings(&mut OLD_SETTINGS);
                    println!("Good bye");
                    break 'main;
                },
                _ => {}
            }

        }

        println!("{}", tg::get_key());
        tg::clear(1);
    }

    Ok(())
}
