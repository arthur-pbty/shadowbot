pub fn parse_color(value: &str) -> Option<u32> {
    let v = value.trim().to_lowercase();
    match v.as_str() {
        "red" | "rouge" => Some(0xED4245),
        "green" | "vert" => Some(0x57F287),
        "blue" | "bleu" => Some(0x5865F2),
        "yellow" | "jaune" => Some(0xFEE75C),
        "orange" => Some(0xFAA61A),
        "purple" | "violet" => Some(0x9B59B6),
        "pink" | "rose" => Some(0xEB459E),
        "white" | "blanc" => Some(0xFFFFFF),
        "black" | "noir" => Some(0x000000),
        _ => {
            let hex = v.trim_start_matches('#').trim_start_matches("0x");
            u32::from_str_radix(hex, 16).ok()
        }
    }
}
