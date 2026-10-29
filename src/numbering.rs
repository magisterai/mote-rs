/// Word alphabetic counters repeat a letter after each alphabet: z, aa, bb.
pub(crate) fn format_counter(number: u64, format: &str) -> Result<String, &'static str> {
    let format = format.trim().to_lowercase();
    let lower = matches!(format.as_str(), "lower_alpha" | "lower_roman");
    let mut text = match format.as_str() {
        "lower_alpha" | "upper_alpha" if number > 0 => {
            let count = (number - 1) / 26 + 1;
            if count > 16_384 {
                return Err("numbering marker exceeds 16384 characters");
            }
            let letter = char::from(b'A' + ((number - 1) % 26) as u8);
            letter.to_string().repeat(count as usize)
        }
        "lower_roman" | "upper_roman" if number > 0 => {
            if number / 1000 > 16_384 {
                return Err("numbering marker exceeds 16384 characters");
            }
            let mut remaining = number;
            let mut text = String::new();
            for (value, digits) in [
                (1000, "M"),
                (900, "CM"),
                (500, "D"),
                (400, "CD"),
                (100, "C"),
                (90, "XC"),
                (50, "L"),
                (40, "XL"),
                (10, "X"),
                (9, "IX"),
                (5, "V"),
                (4, "IV"),
                (1, "I"),
            ] {
                while remaining >= value {
                    text.push_str(digits);
                    remaining -= value;
                }
            }
            text
        }
        _ => number.to_string(),
    };
    if lower {
        text.make_ascii_lowercase();
    }
    Ok(text)
}
