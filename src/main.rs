use colored::*;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = match args.get(1) {
        Some(f) => f,
        None => {
            println!("No arg provided");
            process::exit(1);
        }
    };
    let content = std::fs::read_to_string(filename).unwrap();
    let mut bytes = content.bytes();
    while let Some(b) = bytes.next() {
        if b & 0x80 == 0 {
            // ASCII
            println!(
                "{} {} {}",
                "├".bright_black(),
                format!("{:08b}", b).cyan(),
                std::str::from_utf8(&[b])
                    .unwrap()
                    .replace("\x0a", "↵")
                    .replace("\x20", "<space>")
            );
        } else {
            let nb = (!b).leading_zeros();
            let bin_str = format!("{:08b}", b);
            let (first_bin, second_bin) = bin_str.split_at(nb as usize + 1);
            println!(
                "{} {}{}",
                "├┬".bright_black(),
                first_bin.magenta(),
                second_bin.green()
            );
            let mut v: Vec<u8> = Vec::new();
            let mut f: u32 = 0;
            let base = 6 * (nb - 1);
            v.push(b);
            f |= (b as u32 & (u32::pow(2, 7 - nb) - 1)) << base;
            for i in 0..(nb - 1) {
                let next_byte = bytes.next().unwrap();
                if next_byte & 0xC0 != 0x80 {
                    println!("Illegal byte");
                    process::exit(1);
                }
                let bin_str = format!("{:08b}", next_byte);
                let (first_bin, second_bin) = bin_str.split_at(2);
                println!(
                    "{} {}{}",
                    "│├".bright_black(),
                    first_bin,
                    second_bin.green()
                );
                f |= (next_byte as u32 & 0x3f) << base - (i + 1) * 6;
                v.push(next_byte);
            }
            println!(
                "{}   {} {} (U+{:x} - {} bytes) {}",
                "│└".bright_black(),
                format!("{:0b}", f).cyan(),
                std::str::from_utf8(&v[..]).unwrap().bold(),
                f,
                nb,
                format!("https://www.compart.com/en/unicode/U+{:x}", f).bright_black()
            );
        }
    }
}
