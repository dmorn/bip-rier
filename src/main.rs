use clap::Parser;
/// bip-rier operates in two modes: discovery and capture. In the discovery mode,
/// it lists the available HID devices. In capture mode, collects the events
/// produced by a target hid event. The hid device is expected to produce a
/// valid local path pointing to an existent file. rol instructs then edrawings
/// to open that file.
use hidapi::HidApi;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::ffi::CString;
use std::process::Command;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    path: Option<String>,

    // Command to be called upon scan
    #[arg(long)]
    cmd: String,
}

#[derive(Debug)]
enum Mode {
    Discovery,
    Capture(String),
}

struct Config {
    mode: Mode,
    hid_api: HidApi,
    cmd: String,
}

impl Config {
    fn new(path: Option<String>, cmd: String) -> Result<Config, Box<dyn Error>> {
        let hid_api = HidApi::new()?;
        let mode = match path {
            Some(path) => Mode::Capture(path),
            None => Mode::Discovery,
        };

        Ok(Config { mode, hid_api, cmd })
    }
}

fn run(config: Config) {
    match config.mode {
        Mode::Discovery => list_hid_devices(config.hid_api),
        Mode::Capture(path) => capture_hid_events(config.hid_api, &config.cmd, path),
    }
}

fn capture_hid_events(hid_api: HidApi, cmd: &str, path: String) {
    let path = CString::new(path).unwrap();
    let device = hid_api.open_path(&path).unwrap();

    let pretty_path = String::from_utf8_lossy(path.to_bytes()).to_string();
    println!("Waiting for HID events on device {:}", &pretty_path);

    loop {
        // 512 is an arbitrary choice.
        let mut buf = [0; 512];
        match device.read(&mut buf[..]) {
            Ok(res) => {
                // Some scanners provide a "code" as first bit of the scan.
                // Check wether this happens or not with our target barcodes.
                let v = Vec::from(&buf[..res]);
                match String::from_utf8(v) {
                    Ok(path) => open(cmd, path.trim_matches(char::from(0))),
                    Err(err) => println!("Error: {:}", err),
                }
            }
            Err(err) => println!("Error: {:}", err),
        }
    }
}

fn open(cmd: &str, path: &str) {
    println!("* OPEN: {:?}", path);
    match Command::new(cmd).arg(&path).spawn() {
        Ok(mut child) => {
            let ecode = child.wait().expect("failed to wait on child");
            println!("** DONE {:}: {:?}", path, ecode);
        }
        Err(err) => println!("Error: {:}", err),
    }
}

#[derive(Serialize, Deserialize)]
struct Device {
    path: String,
    manufacturer: String,
    product: String,
}

fn list_hid_devices(hid_api: HidApi) {
    for device in hid_api.device_list() {
        let d = Device {
            path: String::from_utf8_lossy(device.path().to_bytes()).to_string(),
            manufacturer: String::from(device.manufacturer_string().unwrap_or("N/A")),
            product: String::from(device.product_string().unwrap_or("N/A")),
        };
        let j = serde_json::to_string(&d).unwrap();
        println!("{}", j);
    }
}

fn main() {
    let cli = Cli::parse();
    let config = Config::new(cli.path, cli.cmd).unwrap();
    run(config);
}
