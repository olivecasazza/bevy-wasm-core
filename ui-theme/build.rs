use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=themeColors.json");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("theme_gen.rs");

    let json_str = fs::read_to_string("themeColors.json")
        .expect("Failed to read themeColors.json");

    let json: serde_json::Value = serde_json::from_str(&json_str)
        .expect("Failed to parse themeColors.json");

    let mut output = String::from("// Auto-generated theme colors from themeColors.json\n\n");

    for (palette_name, shades) in json.as_object().unwrap() {
        let upper_name = palette_name.to_uppercase();
        for (shade, hex) in shades.as_object().unwrap() {
            let hex_str = hex.as_str().unwrap();
            let (r, g, b) = parse_hex(hex_str);
            output.push_str(&format!(
                "pub const {}_{}: (u8, u8, u8) = ({}, {}, {});\n",
                upper_name, shade, r, g, b
            ));
        }
        output.push('\n');
    }

    fs::write(&dest_path, output).expect("Failed to write theme_gen.rs");
}

fn parse_hex(hex: &str) -> (u8, u8, u8) {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    (r, g, b)
}
