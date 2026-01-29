use tej_core::SpeedTestResult;

pub fn print_json(result: &SpeedTestResult) {
    let json = serde_json::to_string_pretty(result).expect("Failed to serialize result");
    println!("{json}");
}
