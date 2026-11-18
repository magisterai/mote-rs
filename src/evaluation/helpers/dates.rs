//! Calendar arithmetic and formatting on typed dates, without a clock or a time zone.
use super::super::expressions::EvaluatedArgument;
use crate::{
    diagnostics::{Diagnostic, Span},
    model::Value,
};
use time::{Date, Duration, Month};

pub(super) fn calendar_date(value: Value, span: Span) -> Result<Date, Diagnostic> {
    let Value::Date(text) = super::date(value, span)? else {
        unreachable!("the date helper returns a date")
    };
    // The date helper has verified the form and the calendar date.
    let year = text[..4].parse::<i32>().unwrap();
    let month = Month::try_from(text[5..7].parse::<u8>().unwrap()).unwrap();
    Ok(Date::from_calendar_date(year, month, text[8..].parse::<u8>().unwrap()).unwrap())
}

fn value(date: Date, span: Span) -> Result<Value, Diagnostic> {
    if !(0..=9999).contains(&date.year()) {
        return Err(Diagnostic::new(
            span,
            "the resulting date lies outside the years 0000 to 9999",
        ));
    }
    Ok(Value::Date(format!(
        "{:04}-{:02}-{:02}",
        date.year(),
        u8::from(date.month()),
        date.day()
    )))
}

fn whole_number(name: &str, argument: &EvaluatedArgument) -> Result<i64, Diagnostic> {
    match argument.value {
        // A count beyond this bound cannot give a date within the supported years.
        Value::Number(number) if number.fract() == 0.0 && number.abs() <= 4_000_000.0 => {
            Ok(number as i64)
        }
        _ => Err(Diagnostic::new(
            argument.span,
            format!("{name} requires a whole number"),
        )),
    }
}

pub(super) fn evaluate(
    name: &str,
    first: EvaluatedArgument,
    second: EvaluatedArgument,
) -> Result<Value, Diagnostic> {
    let span = first.span;
    let date = calendar_date(first.value, first.span)?;
    let out_of_range = || {
        Diagnostic::new(
            span,
            "the resulting date lies outside the years 0000 to 9999",
        )
    };
    match name {
        "days_between" => {
            let end = calendar_date(second.value, second.span)?;
            Ok(Value::Number((end - date).whole_days() as f64))
        }
        "add_days" => {
            let days = whole_number("add_days", &second)?;
            let moved = date
                .checked_add(Duration::days(days))
                .ok_or_else(out_of_range)?;
            value(moved, span)
        }
        "add_months" => {
            let months = whole_number("add_months", &second)?;
            let index =
                i64::from(date.year()) * 12 + i64::from(u8::from(date.month())) - 1 + months;
            let year = i32::try_from(index.div_euclid(12)).map_err(|_| out_of_range())?;
            let month = Month::try_from(index.rem_euclid(12) as u8 + 1).unwrap();
            if !(0..=9999).contains(&year) {
                return Err(out_of_range());
            }
            // A day that the month lacks gives the last day of the month.
            let day = date.day().min(month.length(year));
            value(Date::from_calendar_date(year, month, day).unwrap(), span)
        }
        "date_part" => {
            let Value::String(part) = &second.value else {
                return Err(Diagnostic::new(
                    second.span,
                    "date_part part must be a string",
                ));
            };
            let number = match part.as_str() {
                "year" => date.year(),
                "month" => i32::from(u8::from(date.month())),
                "day" => i32::from(date.day()),
                "weekday" => i32::from(date.weekday().number_from_monday()),
                "day_of_year" => i32::from(date.ordinal()),
                "week" => i32::from(date.iso_week()),
                _ => {
                    return Err(Diagnostic::new(
                        second.span,
                        "date_part part must be year, month, day, weekday, day_of_year or week",
                    ));
                }
            };
            Ok(Value::Number(f64::from(number)))
        }
        _ => unreachable!("only the date helpers of two arguments are evaluated here"),
    }
}

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
const WEEKDAYS: [&str; 7] = [
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
];

/// Replaces the longest token at each place of the pattern; text in square
/// brackets and every other character stand as written.
pub(super) fn format(date: Date, pattern: &str) -> Result<String, String> {
    let month = MONTHS[usize::from(u8::from(date.month())) - 1];
    let weekday = WEEKDAYS[usize::from(date.weekday().number_days_from_monday())];
    let tokens: [(&str, String); 10] = [
        ("YYYY", format!("{:04}", date.year())),
        ("YY", format!("{:02}", date.year() % 100)),
        ("MMMM", month.to_owned()),
        ("MMM", month[..3].to_owned()),
        ("MM", format!("{:02}", u8::from(date.month()))),
        ("M", u8::from(date.month()).to_string()),
        ("DD", format!("{:02}", date.day())),
        ("D", date.day().to_string()),
        ("dddd", weekday.to_owned()),
        ("ddd", weekday[..3].to_owned()),
    ];
    let mut output = String::new();
    let mut rest = pattern;
    while let Some(character) = rest.chars().next() {
        if character == '[' {
            let Some(end) = rest.find(']') else {
                return Err("format_date pattern has an unclosed square bracket".into());
            };
            output.push_str(&rest[1..end]);
            rest = &rest[end + 1..];
        } else if let Some((token, text)) = tokens.iter().find(|(token, _)| rest.starts_with(token))
        {
            output.push_str(text);
            rest = &rest[token.len()..];
        } else {
            output.push(character);
            rest = &rest[character.len_utf8()..];
        }
    }
    Ok(output)
}
