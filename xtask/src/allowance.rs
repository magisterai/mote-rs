//! The rule by which `--allow` records an allowance. An allowance bounds each
//! measurement of a subject by a stored value. The option lowers a stored value
//! to the present measurement, and raises one only for a subject that `--raise`
//! names.

/// The grid on which the stored values of a measurement lie, and the margin by
/// which a measurement may exceed its stored value.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Scale {
    steps_per_unit: f64,
    slack: f64,
}

impl Scale {
    /// A count is stored as it is measured and may not exceed its stored value.
    pub const COUNT: Self = Self::new(1.0, 0.0);

    pub const fn new(steps_per_unit: f64, slack: f64) -> Self {
        Self {
            steps_per_unit,
            slack,
        }
    }

    /// Whether the check passes a measurement under a stored value.
    pub fn admits(self, stored: f64, measured: f64) -> bool {
        measured <= stored + self.slack
    }

    /// The least value of the grid that is not below the measurement.
    pub fn rounded_up(self, measured: f64) -> f64 {
        let steps = (measured * self.steps_per_unit).ceil();
        // The product of a measurement that lies on the grid may exceed its whole
        // number of steps by a rounding error.
        let below = (steps - 1.0) / self.steps_per_unit;
        if below >= measured {
            below
        } else {
            steps / self.steps_per_unit
        }
    }
}

/// Rejects `--raise` without `--allow`, and a subject to raise that is not among
/// the selected subjects.
pub(crate) fn check_raised(
    raised: &[String],
    selected: &[&String],
    allowing: bool,
) -> Result<(), String> {
    if !allowing && !raised.is_empty() {
        return Err("--raise requires --allow".into());
    }
    match raised
        .iter()
        .find(|identifier| !selected.contains(identifier))
    {
        Some(identifier) => Err(format!("--raise {identifier}: the subject is not selected")),
        None => Ok(()),
    }
}

/// One measurement of a subject.
pub(crate) struct Field {
    pub name: &'static str,
    pub scale: Scale,
    pub measured: f64,
}

#[derive(Debug, PartialEq)]
struct Change {
    subject: String,
    field: &'static str,
    before: f64,
    after: f64,
}

/// The account of one use of `--allow`: the subjects recorded for the first
/// time, the stored values lowered and raised, and the subjects refused.
#[derive(Debug, Default)]
pub(crate) struct Ledger {
    recorded: Vec<String>,
    lowered: Vec<Change>,
    raised: Vec<Change>,
    refused: Vec<Change>,
    refused_identifiers: Vec<String>,
}

impl Ledger {
    /// Returns the values to store for a subject, in the order of its fields, or
    /// `None` if the subject is refused and its stored values stand.
    ///
    /// A subject without stored values is recorded. A subject that the check
    /// passes keeps, for each field, the lower of the stored value and the
    /// measurement rounded up, so that no stored value rises. A subject that the
    /// check fails is refused unless `raising` holds, in which case each field
    /// that exceeds its stored value takes the measurement rounded up.
    pub fn allow(
        &mut self,
        subject: &str,
        identifier: &str,
        fields: &[Field],
        stored: Option<&[f64]>,
        raising: bool,
    ) -> Option<Vec<f64>> {
        let rounded = |field: &Field| field.scale.rounded_up(field.measured);
        let Some(stored) = stored else {
            self.recorded.push(subject.to_owned());
            return Some(fields.iter().map(rounded).collect());
        };
        let change = |field: &Field, before: f64, after: f64| Change {
            subject: subject.to_owned(),
            field: field.name,
            before,
            after,
        };
        let exceeds = |field: &Field, stored: f64| !field.scale.admits(stored, field.measured);
        if !raising
            && fields
                .iter()
                .zip(stored)
                .any(|(field, stored)| exceeds(field, *stored))
        {
            for (field, stored) in fields.iter().zip(stored) {
                if exceeds(field, *stored) {
                    self.refused.push(change(field, *stored, rounded(field)));
                }
            }
            self.refused_identifiers.push(identifier.to_owned());
            return None;
        }
        Some(
            fields
                .iter()
                .zip(stored)
                .map(|(field, stored)| {
                    if exceeds(field, *stored) {
                        let after = rounded(field);
                        self.raised.push(change(field, *stored, after));
                        return after;
                    }
                    let after = rounded(field).min(*stored);
                    if after < *stored {
                        self.lowered.push(change(field, *stored, after));
                    }
                    after
                })
                .collect(),
        )
    }

    /// Whether any subject was refused.
    pub fn refused(&self) -> bool {
        !self.refused.is_empty()
    }

    /// The tables of every stored value that changed and of every subject refused.
    pub fn report(&self) -> String {
        let mut text = String::new();
        if !self.recorded.is_empty() {
            text += "\nrecorded without an earlier allowance\n";
            for subject in &self.recorded {
                text += &format!("{subject}\n");
            }
        }
        for (heading, changes) in [
            ("lowered | field | before | after", &self.lowered),
            ("RAISED BY --raise | field | before | after", &self.raised),
            ("REFUSED | field | allowance | required", &self.refused),
        ] {
            if changes.is_empty() {
                continue;
            }
            text += &format!("\n{heading}\n");
            for change in changes {
                text += &format!(
                    "{} | {} | {} | {}\n",
                    change.subject, change.field, change.before, change.after
                );
            }
        }
        if self.refused() {
            text += "A refused subject exceeds its allowance, which stands as it was. The allowance\n\
                     rises only if --allow is given with:\n";
            for identifier in &self.refused_identifiers {
                text += &format!("  --raise {identifier}\n");
            }
        }
        if text.is_empty() {
            text += "\nno allowance changed\n";
        }
        text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TENTHS: Scale = Scale::new(10.0, 0.05);
    const THOUSANDTHS: Scale = Scale::new(1000.0, 0.0005);

    fn fields(count: f64, offset: f64) -> [Field; 2] {
        [
            Field {
                name: "count",
                scale: Scale::COUNT,
                measured: count,
            },
            Field {
                name: "offset",
                scale: TENTHS,
                measured: offset,
            },
        ]
    }

    #[test]
    fn a_passing_subject_lowers_its_stored_values_and_raises_none() {
        let mut ledger = Ledger::default();
        // The offset passes the check only by the slack, and would be stored as
        // 0.4 if it were rounded up without regard to the stored value.
        let within_slack = ledger.allow("a", "a", &fields(2.0, 0.34), Some(&[2.0, 0.3]), false);
        assert_eq!(within_slack, Some(vec![2.0, 0.3]));
        assert_eq!(ledger.report(), "\nno allowance changed\n");

        let improved = ledger.allow("b", "b", &fields(1.0, 0.12), Some(&[2.0, 0.3]), false);
        assert_eq!(improved, Some(vec![1.0, 0.2]));
        assert!(!ledger.refused());
        assert_eq!(
            ledger.report(),
            "\nlowered | field | before | after\nb | count | 2 | 1\nb | offset | 0.3 | 0.2\n"
        );
    }

    #[test]
    fn a_subject_that_exceeds_a_stored_value_is_refused() {
        let mut ledger = Ledger::default();
        // The count improved, and yet no value of a refused subject changes.
        let refused = ledger.allow(
            "probes/a",
            "probe:a",
            &fields(1.0, 0.36),
            Some(&[2.0, 0.3]),
            false,
        );
        assert_eq!(refused, None);
        assert!(ledger.refused());
        let report = ledger.report();
        assert!(
            report.contains(
                "REFUSED | field | allowance | required\nprobes/a | offset | 0.3 | 0.4\n"
            ),
            "{report}"
        );
        assert!(report.contains("  --raise probe:a\n"), "{report}");
        assert!(!report.contains("lowered"), "{report}");

        let more_counted =
            Ledger::default().allow("a", "a", &fields(3.0, 0.1), Some(&[2.0, 0.3]), false);
        assert_eq!(more_counted, None);
    }

    #[test]
    fn a_subject_named_for_raising_takes_its_present_result() {
        let mut ledger = Ledger::default();
        let raised = ledger.allow("a", "a", &fields(1.0, 0.36), Some(&[2.0, 0.3]), true);
        assert_eq!(raised, Some(vec![1.0, 0.4]));
        assert!(!ledger.refused());
        assert_eq!(
            ledger.report(),
            "\nlowered | field | before | after\na | count | 2 | 1\n\
             \nRAISED BY --raise | field | before | after\na | offset | 0.3 | 0.4\n"
        );

        // A subject named for raising that passes the check is raised in nothing.
        let passing =
            Ledger::default().allow("a", "a", &fields(2.0, 0.34), Some(&[2.0, 0.3]), true);
        assert_eq!(passing, Some(vec![2.0, 0.3]));
    }

    #[test]
    fn a_subject_without_an_allowance_is_recorded() {
        let mut ledger = Ledger::default();
        let recorded = ledger.allow("a", "a", &fields(3.0, 0.31), None, false);
        assert_eq!(recorded, Some(vec![3.0, 0.4]));
        assert!(!ledger.refused());
        assert_eq!(
            ledger.report(),
            "\nrecorded without an earlier allowance\na\n"
        );
    }

    #[test]
    fn a_subject_is_raised_only_with_allow_and_only_if_selected() {
        let (first, second) = ("first".to_owned(), "second".to_owned());
        let raised = [first.clone()];
        assert_eq!(check_raised(&raised, &[&first, &second], true), Ok(()));
        assert_eq!(check_raised(&[], &[&first], false), Ok(()));
        assert_eq!(
            check_raised(&raised, &[&first], false),
            Err("--raise requires --allow".to_owned())
        );
        assert_eq!(
            check_raised(&raised, &[&second], true),
            Err("--raise first: the subject is not selected".to_owned())
        );
    }

    #[test]
    fn a_measurement_is_rounded_up_to_the_nearest_value_of_its_grid() {
        for (scale, steps_per_unit) in [(TENTHS, 10.0), (THOUSANDTHS, 1000.0)] {
            for step in 0..=3000 {
                let on_grid = f64::from(step) / steps_per_unit;
                assert_eq!(scale.rounded_up(on_grid), on_grid);
                for part in [0.001, 0.25, 0.5, 0.999] {
                    let measured = (f64::from(step) + part) / steps_per_unit;
                    let rounded = scale.rounded_up(measured);
                    assert_eq!(rounded, f64::from(step + 1) / steps_per_unit, "{measured}");
                    assert!(scale.admits(rounded, measured));
                }
            }
        }
        assert_eq!(Scale::COUNT.rounded_up(3.0), 3.0);
    }

    /// The check and `--allow` agree: a measurement that the check passes leaves
    /// the stored value no higher, a measurement that the check fails is refused,
    /// and the value stored in either case passes the measurement and stays as it
    /// is when the same measurement is allowed again.
    #[test]
    fn the_check_and_the_stored_value_agree_for_every_measurement() {
        for (scale, steps_per_unit) in [(TENTHS, 10.0), (THOUSANDTHS, 1000.0)] {
            for stored_step in 0..=30 {
                let stored = f64::from(stored_step) / steps_per_unit;
                for measured_part in 0..=4000 {
                    let measured = f64::from(measured_part) / (steps_per_unit * 100.0);
                    let field = || {
                        [Field {
                            name: "offset",
                            scale,
                            measured,
                        }]
                    };
                    let mut ledger = Ledger::default();
                    let allowed = ledger.allow("a", "a", &field(), Some(&[stored]), false);
                    if !scale.admits(stored, measured) {
                        assert_eq!(allowed, None, "{stored} {measured}");
                        let raised = ledger
                            .allow("a", "a", &field(), Some(&[stored]), true)
                            .unwrap();
                        assert!(raised[0] > stored && scale.admits(raised[0], measured));
                        continue;
                    }
                    let allowed = allowed.unwrap();
                    assert!(allowed[0] <= stored, "{stored} {measured}");
                    assert!(scale.admits(allowed[0], measured), "{stored} {measured}");
                    assert_eq!(
                        ledger.allow("a", "a", &field(), Some(&allowed), false),
                        Some(allowed)
                    );
                }
            }
        }
    }
}
