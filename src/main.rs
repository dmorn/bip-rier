/// bip-rier operates in two modes: discovery and capture. In the discovery mode,
/// it lists the available HID devices. In capture mode, collects the events
/// produced by a target hid event. The hid device is expected to produce a
/// valid local path pointing to an existent file. rol instructs then edrawings
/// to open that file.
use hidapi::{HidApi, HidDevice};
use std::env;
use std::error::Error;
use std::ffi::CString;

#[derive(Debug)]
enum Mode {
    Discovery,
    Capture(String),
}

struct Config {
    mode: Mode,
    hid_api: HidApi,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, Box<dyn Error>> {
        let hid_api = HidApi::new()?;
        let mut mode = Mode::Discovery;
        if args.len() > 1 {
            let path: String = args[1].clone();
            mode = Mode::Capture(path);
        }

        Ok(Config { mode, hid_api })
    }
}

fn run(config: Config) {
    match config.mode {
        Mode::Discovery => list_hid_devices(config.hid_api),
        Mode::Capture(path) => capture_hid_events(config.hid_api, path),
    }
}

#[cfg(not(target_os = "windows"))]
fn print_is_exclusive(device: HidDevice) {
    let is_open = device.is_open_exclusive().unwrap();
    println!("Is open exclusive? {:?}", is_open);
}

#[cfg(target_os = "windows")]
fn print_is_exclusive(_dev: HidDevice) {
    println!("Could not determine exlusive access on Windows");
}

fn capture_hid_events(hid_api: HidApi, path: String) {
    let path = CString::new(path).unwrap();
    let device = hid_api.open_path(&path).unwrap();
    print_is_exclusive(device);
}

fn list_hid_devices(hid_api: HidApi) {
    for device in hid_api.device_list() {
        println!(
            "- [path: {:?}, manufacturer: {:?}, product: {:?}]",
            device.path(),
            device.manufacturer_string().unwrap_or("N/A"),
            device.product_string().unwrap_or("N/A")
        );
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap();
    run(config);
}
