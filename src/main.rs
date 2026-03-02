fn main() {
    let depth_meters: f64 = 3872.45; // pretend measurement from pressure sensor
    let salinity_psu: f32 = 34.92; // practical salinity units
    let temperature_c: f32 = 2.14; // very cold deep water

    println!("🌊 Ocean Depth Sensor Reading 🌊");
    println!("  Depth     : {:.2} meters", depth_meters);
    println!("  Salinity  : {:.2} PSU", salinity_psu);
    println!("  Temp      : {:.2} °C", temperature_c);
    println!("  Status    : Nominal (for now… 😈)");
}
