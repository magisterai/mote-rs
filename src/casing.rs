//! Unicode simple casing: one scalar per input scalar, independent of locale.
pub(crate) fn convert(text: &str, uppercase: bool) -> String {
    text.chars()
        .map(|character| {
            let mapping = match character {
                '\u{1f80}' => Some(('\u{1f88}', '\u{1f80}')),
                '\u{1f81}' => Some(('\u{1f89}', '\u{1f81}')),
                '\u{1f82}' => Some(('\u{1f8a}', '\u{1f82}')),
                '\u{1f83}' => Some(('\u{1f8b}', '\u{1f83}')),
                '\u{1f84}' => Some(('\u{1f8c}', '\u{1f84}')),
                '\u{1f85}' => Some(('\u{1f8d}', '\u{1f85}')),
                '\u{1f86}' => Some(('\u{1f8e}', '\u{1f86}')),
                '\u{1f87}' => Some(('\u{1f8f}', '\u{1f87}')),
                '\u{1f90}' => Some(('\u{1f98}', '\u{1f90}')),
                '\u{1f91}' => Some(('\u{1f99}', '\u{1f91}')),
                '\u{1f92}' => Some(('\u{1f9a}', '\u{1f92}')),
                '\u{1f93}' => Some(('\u{1f9b}', '\u{1f93}')),
                '\u{1f94}' => Some(('\u{1f9c}', '\u{1f94}')),
                '\u{1f95}' => Some(('\u{1f9d}', '\u{1f95}')),
                '\u{1f96}' => Some(('\u{1f9e}', '\u{1f96}')),
                '\u{1f97}' => Some(('\u{1f9f}', '\u{1f97}')),
                '\u{1fa0}' => Some(('\u{1fa8}', '\u{1fa0}')),
                '\u{1fa1}' => Some(('\u{1fa9}', '\u{1fa1}')),
                '\u{1fa2}' => Some(('\u{1faa}', '\u{1fa2}')),
                '\u{1fa3}' => Some(('\u{1fab}', '\u{1fa3}')),
                '\u{1fa4}' => Some(('\u{1fac}', '\u{1fa4}')),
                '\u{1fa5}' => Some(('\u{1fad}', '\u{1fa5}')),
                '\u{1fa6}' => Some(('\u{1fae}', '\u{1fa6}')),
                '\u{1fa7}' => Some(('\u{1faf}', '\u{1fa7}')),
                '\u{1fb3}' => Some(('\u{1fbc}', '\u{1fb3}')),
                '\u{1fc3}' => Some(('\u{1fcc}', '\u{1fc3}')),
                '\u{1ff3}' => Some(('\u{1ffc}', '\u{1ff3}')),
                _ => None,
            };
            if let Some((upper, lower)) = mapping {
                return if uppercase { upper } else { lower };
            }
            if uppercase {
                let mut characters = character.to_uppercase();
                let first = characters.next().unwrap();
                if characters.next().is_some() {
                    character
                } else {
                    first
                }
            } else {
                character.to_lowercase().next().unwrap()
            }
        })
        .collect()
}
