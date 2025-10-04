pub fn log(message: &str, enabled: bool) {
    if enabled {
        println!("[NODE]: {}", message);
    }
}
