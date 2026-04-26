use shake::device::Device;
use shake::error::ShakeResult;

fn main() -> ShakeResult<()> {
    let devices = Device::enumerate()?;

    for info in &devices {
        println!("Device #{}:", info.id);
        println!("  Name: {}", info.name);
        println!("  Capacity: {}", info.max_effects);
        println!("  Path: {}", info.path.display());
    }

    Ok(())
}
