/// bip-rier operates in two modes: discovery and capture. In the discovery
/// mode, it lists the available HID devices. In capture mode, reads from one
/// of the devices. Upon each read, it executes a command passing the captured
/// text as first argument. The command comes from the user.
use clap::{Parser, Subcommand};
use hidapi::HidApi;
use serde::{Deserialize, Serialize};
use std::ffi::CString;
use std::process::Command;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    cmd: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Caputes HID device events
    Capture {
        /// Path to the HID device
        path: String,
        /// Command to be executed upon scan
        #[arg(long)]
        cmd: String,
    },
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
    let hid_api = HidApi::new().unwrap();

    match cli.cmd {
        None => list_hid_devices(hid_api),
        Some(Commands::Capture { path, cmd }) => capture_hid_events(hid_api, &cmd, path),
    }
}
