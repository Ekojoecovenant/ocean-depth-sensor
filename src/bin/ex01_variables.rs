fn main() {
    // 1. Immutable variables (can't change it later)
    let depth_meters = 1200.5; // inferred f64
    println!("Current depth: {} meters", depth_meters);

    // depth_meters = 1500.0; // ERROR: cannot assign twice to immutable variable

    // 2. Mutable variable (can change because of 'mut')
    let mut temperature_c = 4.2;
    println!("Initial temperature: {} °C", temperature_c);

    temperature_c = temperature_c - 1.8; // We can change it!
    println!("After cooling: {} °C", temperature_c);

    // 3. Shadowing: reuse the same name with a NEW value (even different type!)
    let depth_meters = "Deep in the abyss... scary but beautiful"; // Now it's a string (&str)
    println!("Poetic depth description: {}", depth_meters);

    let mut sensor_ok = true;
    println!("Sensor status: OK? {}", sensor_ok);

    sensor_ok = false;
    println!("Sensor status after failure: OK? {}", sensor_ok);

    // Shadow temperature_c (simple string, no fancy formatting)
    let temperature_c = "Freezing cold now!";
    println!("Temperature update: {}", temperature_c);
}
