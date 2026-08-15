use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use chrono::Local;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    keybd_event, KEYEVENTF_EXTENDEDKEY, KEYEVENTF_KEYUP, VIRTUAL_KEY,
};

fn main() {
    let vk_right_alt = VIRTUAL_KEY(0xA5);

    println!("--- started ---");
    println!("triggering every 35 seconds. ctrl + c to stop.");
    println!();

    loop {
        for i in (1..=35).rev() {
            print!("\rNext restart in: {} seconds...    ", i);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_secs(1));
        }

        print!("\r                                     \r");
        io::stdout().flush().unwrap();

        unsafe {
            keybd_event(0xA5, 0, KEYEVENTF_EXTENDEDKEY, 0);
        }

        thread::sleep(Duration::from_millis(50));

        unsafe {
            keybd_event(0xA5, 0, KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP, 0);
        }

        let timestamp = Local::now().format("%H:%M:%S");
        println!("[{}] target key triggered.", timestamp);
    }
}
