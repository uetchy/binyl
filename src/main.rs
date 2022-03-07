use anyhow::{anyhow, Context, Result};
use colored::*;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let content = match args.get(1) {
        // read from file
        Some(filename) => std::fs::read_to_string(&filename)
            .with_context(|| format!("could not read file `{}`", filename))?,

        // read from stdin
        None => {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line)?;
            line
        }
    };

    parse_string(&content)
}

fn parse_string(content: &str) -> Result<()> {
    let mut bytes = content.bytes();

    while let Some(b) = bytes.next() {
        if b & 0x80 == 0 {
            // if MSB is 0, is is just ASCII.
            println!(
                "{} {} {} (U+{:04X} ASCII={})",
                "├".bright_black(),
                format!("{:08b}", b).cyan(),
                std::str::from_utf8(&[b])?
                    .replace("\x0a", "↵")
                    .replace("\x20", "<space>")
                    .bold(),
                b,
                b,
            );
        } else {
            let nb = (!b).leading_zeros();
            let (first, last) = split_octet(b, nb);

            print!("{} ", "├".bright_black());
            print!(
                "{}{}",
                format!("{:04b}", first).magenta(),
                format!("{:04b}", last).green(),
            );

            let mut v: Vec<u8> = vec![];
            let mut f: u32 = 0;
            let base = 6 * (nb - 1);

            v.push(b);

            f |= (b as u32 & (u32::pow(2, 7 - nb) - 1)) << base;

            for i in 0..(nb - 1) {
                let next_byte = bytes.next().unwrap();

                // if first 2 MSB are not '10', it's illegal sequence.
                if next_byte & 0xC0 != 0x80 {
                    return Err(anyhow!("Illegal byte"));
                }

                let (first, last) = split_octet(next_byte, 2);

                print!(
                    "{}{}",
                    format!("{:02b}", first).bright_black(),
                    format!("{:06b}", last).green()
                );

                f |= (next_byte as u32 & 0x3f) << base - (i + 1) * 6;
                v.push(next_byte);
            }

            println!(
                " ({} bytes) = {} {} (U+{:04X})",
                nb,
                format!("{:0b}", f).cyan(),
                std::str::from_utf8(&v[..])
                    .unwrap_or("[INVALID CODE]")
                    .bold(),
                f
            );
        }
    }

    Ok(())
}

fn split_octet(octet: u8, separate_at: u32) -> (u8, u8) {
    let mask = u8::pow(2, separate_at) - 1;
    let first = (octet & !mask) >> (8 - separate_at);
    let last = octet & mask;
    (first, last)
}
