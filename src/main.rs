use hidapi::HidApi;

fn main() {
    println!("Printing all available hid devices:");
    let api = HidApi::new().unwrap();
    for device in api.device_list() {
        println!(
            "vendor_id: {:04x}, product_id: {:04x}, manufacturer: {:?}, product: {:?}",
            device.vendor_id(),
            device.product_id(),
            device.manufacturer_string(),
            device.product_string()
        );
    }
}
