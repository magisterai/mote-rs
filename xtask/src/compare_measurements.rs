//! Verification of frozen font-measurement fixtures and comparison of
//! candidate measurements against their expectations.
use crate::{
    Outcome,
    arguments::{Arguments, unknown},
    digest::fingerprint,
    json::{Json, compact, object, pretty, set, string},
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

const POINT_FIELDS: [&str; 5] = ["layout_width", "paint_width", "ascent", "descent", "height"];
const RESULT_FIELDS: [&str; 11] = [
    "id",
    "font_id",
    "variant",
    "metric_source",
    "prepared_text",
    "missing_codepoints",
    "layout_width",
    "paint_width",
    "ascent",
    "descent",
    "height",
];
const VARIANTS: [&str; 4] = ["regular", "bold", "italic", "bold_italic"];
const TOLERANCE: f64 = 1e-9;
const TOLERANCE_EXPONENT: i32 = -9;
const TOLERANCE_TEXT: &str = "1E-9";

const HELP: &str =
    "Usage: cargo xtask compare-measurements --fixture PATH [--actual PATH] [--output PATH]

Verify a frozen font-measurement fixture and compare candidate measurements.
--fixture PATH  the fixture directory containing capture.json
--actual PATH   candidate result JSON; omit to verify fixture integrity only
--output PATH   a new directory for report.json
";

type Results = BTreeMap<String, Json>;

pub(crate) struct Fixture {
    pub capture: Json,
    pub requests: Json,
    pub expected: Results,
}

fn require(condition: bool, message: impl Into<String>) -> Result<(), String> {
    if condition {
        Ok(())
    } else {
        Err(message.into())
    }
}

/// A missing field reads as null, which no check accepts.
fn at<'a>(value: &'a Json, name: &str) -> &'a Json {
    value.get(name).unwrap_or(&Json::Null)
}

fn fields(value: &Json, expected: &[&str], location: &str) -> Result<(), String> {
    let matches = value.field_names().is_some_and(|names| {
        names.len() == expected.len() && expected.iter().all(|field| names.contains(field))
    });
    require(matches, format!("{location}: unexpected or missing fields"))
}

fn identifier(value: &Json) -> bool {
    value.as_str().is_some_and(|text| {
        let mut characters = text.chars();
        characters
            .next()
            .is_some_and(|first| first.is_ascii_lowercase())
            && characters.all(|character| {
                character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_'
            })
    })
}

fn version_one(value: &Json) -> bool {
    at(value, "schema_version").as_integer() == Some(1)
}

pub(crate) fn local_file(root: &Path, relative: &str) -> Result<PathBuf, String> {
    require(!relative.is_empty(), "empty fixture path")?;
    let relative_path = Path::new(relative);
    require(!relative_path.is_absolute(), "absolute fixture path")?;
    let root = fs::canonicalize(root).map_err(|error| format!("{}: {error}", root.display()))?;
    let path = fs::canonicalize(root.join(relative_path))
        .map_err(|_| format!("missing fixture file: {relative}"))?;
    require(path.starts_with(&root), "fixture path leaves its directory")?;
    require(path.is_file(), format!("missing fixture file: {relative}"))?;
    Ok(path)
}

fn fixture_file(root: &Path, relative: &Json) -> Result<PathBuf, String> {
    local_file(root, relative.as_str().ok_or("empty fixture path")?)
}

fn verify_digest(path: &Path, digest: &Json) -> Result<(), String> {
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    let valid = digest.as_str().is_some_and(|digest| {
        digest.len() == 64
            && digest
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    });
    require(valid, format!("invalid fingerprint for {name}"))?;
    require(
        digest.as_str() == Some(fingerprint(path)?.as_str()),
        format!("fingerprint mismatch: {name}"),
    )
}

fn index_records(records: &Json, location: &str) -> Result<Results, String> {
    let records = records
        .as_array()
        .ok_or_else(|| format!("{location}: expected array"))?;
    let mut indexed = BTreeMap::new();
    for record in records {
        require(
            record.as_object().is_some() && identifier(at(record, "id")),
            format!("{location}: invalid identifier"),
        )?;
        let name = at(record, "id").as_str().unwrap_or_default().to_owned();
        require(
            !indexed.contains_key(&name),
            format!("{location}: duplicate identifier {name}"),
        )?;
        indexed.insert(name, record.clone());
    }
    Ok(indexed)
}

fn valid_codepoint(value: &Json) -> bool {
    value
        .as_integer()
        .is_some_and(|value| (0..=0x10FFFF).contains(&value) && !(0xD800..=0xDFFF).contains(&value))
}

pub(crate) fn read_results(path: &Path, requests_digest: &str) -> Result<Results, String> {
    let output = Json::read(path)?;
    fields(
        &output,
        &["schema_version", "requests_sha256", "measurements"],
        "results",
    )?;
    require(version_one(&output), "unsupported result schema version")?;
    require(
        at(&output, "requests_sha256").as_str() == Some(requests_digest),
        "result request fingerprint mismatch",
    )?;
    let results = index_records(at(&output, "measurements"), "measurements")?;
    for (name, result) in &results {
        fields(result, &RESULT_FIELDS, name)?;
        require(
            identifier(at(result, "font_id")),
            format!("{name}: invalid font identifier"),
        )?;
        require(
            at(result, "variant")
                .as_str()
                .is_some_and(|variant| VARIANTS.contains(&variant)),
            format!("{name}: invalid font variant"),
        )?;
        require(
            matches!(
                at(result, "metric_source").as_str(),
                Some("core" | "declared")
            ),
            format!("{name}: invalid metric source"),
        )?;
        require(
            at(result, "prepared_text").as_str().is_some(),
            format!("{name}: invalid prepared text"),
        )?;
        for field in POINT_FIELDS {
            require(
                at(result, field).as_number().is_some(),
                format!("{name}: {field} must be finite numeric points"),
            )?;
        }
        let missing = at(result, "missing_codepoints").as_array();
        require(
            missing.is_some_and(|missing| missing.iter().all(valid_codepoint)),
            format!("{name}: invalid missing codepoints"),
        )?;
        let missing = missing.unwrap_or_default();
        let distinct: BTreeSet<_> = missing.iter().filter_map(Json::as_integer).collect();
        require(
            distinct.len() == missing.len(),
            format!("{name}: duplicate missing codepoints"),
        )?;
    }
    Ok(results)
}

fn verify_font(root: &Path, name: &str, font: &Json) -> Result<(), String> {
    let core = at(font, "format").as_str() == Some("core");
    let mut expected = vec!["id", "family", "variant", "format"];
    if !core {
        expected.extend(["path", "face_index", "sha256", "size_bytes", "source_url"]);
    }
    fields(font, &expected, name)?;
    require(
        matches!(
            at(font, "format").as_str(),
            Some("core" | "truetype" | "cff")
        ),
        format!("{name}: invalid font format"),
    )?;
    let family = at(font, "family").as_str().unwrap_or_default();
    require(!family.is_empty(), format!("{name}: empty font family"))?;
    require(
        at(font, "variant")
            .as_str()
            .is_some_and(|variant| VARIANTS.contains(&variant)),
        format!("{name}: invalid font variant"),
    )?;
    if core {
        return require(
            matches!(family, "Helvetica" | "Times" | "Courier"),
            format!("{name}: invalid core family"),
        );
    }
    require(
        at(font, "face_index").as_integer() == Some(0),
        format!("{name}: only standalone font fixtures are defined"),
    )?;
    let path = fixture_file(root, at(font, "path"))?;
    verify_digest(&path, at(font, "sha256"))?;
    let length = fs::metadata(&path)
        .map_err(|error| format!("{}: {error}", path.display()))?
        .len();
    require(
        at(font, "size_bytes").as_integer() == Some(length.into()),
        format!("{name}: font length mismatch"),
    )?;
    require(
        at(font, "source_url")
            .as_str()
            .is_some_and(|url| url.starts_with("https://")),
        format!("{name}: missing font source URL"),
    )
}

pub(crate) fn load_fixture(root: &Path) -> Result<Fixture, String> {
    let root = fs::canonicalize(root).map_err(|error| format!("{}: {error}", root.display()))?;
    let capture = Json::read(&root.join("capture.json"))?;
    fields(
        &capture,
        &[
            "schema_version",
            "requests",
            "requests_sha256",
            "expected",
            "expected_sha256",
            "case_count",
            "coordinate_unit",
            "absolute_tolerance_points",
            "captured_at_utc",
            "word_review_status",
            "license",
            "license_sha256",
        ],
        "capture",
    )?;
    require(version_one(&capture), "unsupported capture schema version")?;
    require(
        at(&capture, "coordinate_unit").as_str() == Some("point"),
        "unsupported coordinate unit",
    )?;
    require(
        *at(&capture, "absolute_tolerance_points") == Json::Decimal(TOLERANCE),
        "unexpected comparison tolerance",
    )?;
    require(
        at(&capture, "word_review_status").as_str() == Some("unreviewed"),
        "measurement capture cannot establish Word review",
    )?;
    let requests_path = fixture_file(&root, at(&capture, "requests"))?;
    let expected_path = fixture_file(&root, at(&capture, "expected"))?;
    verify_digest(&requests_path, at(&capture, "requests_sha256"))?;
    verify_digest(&expected_path, at(&capture, "expected_sha256"))?;
    verify_digest(
        &fixture_file(&root, at(&capture, "license"))?,
        at(&capture, "license_sha256"),
    )?;
    let requests = Json::read(&requests_path)?;
    fields(
        &requests,
        &["schema_version", "coordinate_unit", "fonts", "cases"],
        "requests",
    )?;
    require(version_one(&requests), "unsupported request schema version")?;
    require(
        at(&requests, "coordinate_unit").as_str() == Some("point"),
        "request coordinate unit mismatch",
    )?;
    let fonts = index_records(at(&requests, "fonts"), "fonts")?;
    for (name, font) in &fonts {
        verify_font(&root, name, font)?;
    }
    let cases = index_records(at(&requests, "cases"), "cases")?;
    require(
        !cases.is_empty() && at(&capture, "case_count").as_integer() == Some(cases.len() as i128),
        "capture case count mismatch",
    )?;
    for (name, request) in &cases {
        fields(
            request,
            &["id", "font_id", "size_points", "layout_kerning", "text"],
            name,
        )?;
        let font_id = at(request, "font_id").as_str().unwrap_or_default();
        require(
            fonts.contains_key(font_id),
            format!("{name}: unknown requested font"),
        )?;
        require(
            at(request, "size_points")
                .as_number()
                .is_some_and(|size| size > 0.0),
            format!("{name}: font size must be finite and positive"),
        )?;
        require(
            at(request, "layout_kerning").as_bool().is_some(),
            format!("{name}: kerning must be boolean"),
        )?;
        require(
            at(request, "text").as_str().is_some(),
            format!("{name}: invalid source text"),
        )?;
    }
    let requests_digest = at(&capture, "requests_sha256").as_str().unwrap_or_default();
    let expected = read_results(&expected_path, requests_digest)?;
    require(
        expected.keys().eq(cases.keys()),
        "expected results do not cover the requested cases",
    )?;
    for (name, result) in &expected {
        let request = &cases[name];
        let font = &fonts[at(request, "font_id").as_str().unwrap_or_default()];
        require(
            at(result, "font_id") == at(request, "font_id")
                && at(result, "variant") == at(font, "variant"),
            format!("{name}: captured font identity mismatch"),
        )?;
        let source = if at(font, "format").as_str() == Some("core") {
            "core"
        } else {
            "declared"
        };
        require(
            at(result, "metric_source").as_str() == Some(source),
            format!("{name}: captured metric source mismatch"),
        )?;
    }
    Ok(Fixture {
        capture,
        requests,
        expected,
    })
}

/// The shortest decimal spelling of a number as an exact mantissa and power of ten.
fn exact_decimal(value: &Json) -> Option<(i128, i32)> {
    match value {
        Json::Integer(value) => Some((*value, 0)),
        Json::Decimal(value) => {
            let spelling = format!("{value:e}");
            let (mantissa, exponent) = spelling.split_once('e')?;
            let exponent: i32 = exponent.parse().ok()?;
            let fraction = mantissa
                .split_once('.')
                .map_or(0, |(_, fraction)| fraction.len());
            let digits: i128 = mantissa.replace('.', "").parse().ok()?;
            Some((digits, exponent - fraction as i32))
        }
        _ => None,
    }
}

fn within_tolerance(first: &Json, second: &Json) -> Result<bool, String> {
    let (Some(first), Some(second)) = (exact_decimal(first), exact_decimal(second)) else {
        return Ok(first == second);
    };
    let unrepresentable = || "unrepresentable metric arithmetic".to_owned();
    let exponent = first.1.min(second.1).min(TOLERANCE_EXPONENT);
    let scale = |(mantissa, power): (i128, i32)| {
        10i128
            .checked_pow((power - exponent) as u32)
            .and_then(|factor| mantissa.checked_mul(factor))
            .ok_or_else(unrepresentable)
    };
    let difference = scale(first)?
        .checked_sub(scale(second)?)
        .ok_or_else(unrepresentable)?;
    Ok(difference.unsigned_abs() <= scale((1, TOLERANCE_EXPONENT))?.unsigned_abs())
}

pub(crate) fn compare(expected: &Results, actual: &Results) -> Result<Vec<Json>, String> {
    let mut compared: Vec<_> = RESULT_FIELDS[1..].to_vec();
    compared.sort();
    let mut differences = Vec::new();
    let names: BTreeSet<_> = expected.keys().chain(actual.keys()).collect();
    for name in names {
        let (Some(first), Some(second)) = (expected.get(name), actual.get(name)) else {
            let reason = if actual.contains_key(name) {
                "unexpected result"
            } else {
                "missing result"
            };
            differences.push(object([
                ("case", string(name)),
                ("field", string("id")),
                ("reason", string(reason)),
            ]));
            continue;
        };
        for field in &compared {
            let (first, second) = (at(first, field), at(second, field));
            let equal = if POINT_FIELDS.contains(field) {
                within_tolerance(first, second)?
            } else {
                first == second
            };
            if !equal {
                differences.push(object([
                    ("case", string(name)),
                    ("field", string(field)),
                    ("expected", first.clone()),
                    ("actual", second.clone()),
                ]));
            }
        }
    }
    Ok(differences)
}

struct Options {
    fixture: PathBuf,
    actual: Option<PathBuf>,
    output: Option<PathBuf>,
}

impl Options {
    fn parse(mut arguments: Arguments) -> Result<Option<Self>, String> {
        let (mut fixture, mut actual, mut output) = (None, None, None);
        while let Some(argument) = arguments.next() {
            match argument.as_str() {
                "--help" | "-h" => {
                    print!("{HELP}");
                    return Ok(None);
                }
                "--fixture" => fixture = Some(arguments.path(&argument)?),
                "--actual" => actual = Some(arguments.path(&argument)?),
                "--output" => output = Some(arguments.path(&argument)?),
                _ => return Err(unknown(&argument)),
            }
        }
        Ok(Some(Self {
            fixture: fixture.ok_or("--fixture is required")?,
            actual,
            output,
        }))
    }
}

type Comparison<'a> = dyn Fn(&Results, &Results) -> Result<Vec<Json>, String> + 'a;

fn measure(
    options: &Options,
    comparison: &Comparison<'_>,
    report: &mut Vec<(String, Json)>,
) -> Result<(), String> {
    let fixture = load_fixture(&options.fixture)?;
    let requests_digest = at(&fixture.capture, "requests_sha256").clone();
    let font_count = at(&fixture.requests, "fonts")
        .as_array()
        .map_or(0, <[_]>::len);
    set(
        report,
        "cases",
        Json::Integer(fixture.expected.len() as i128),
    );
    set(report, "fonts", Json::Integer(font_count as i128));
    set(report, "requests_sha256", requests_digest.clone());
    set(
        report,
        "expected_sha256",
        at(&fixture.capture, "expected_sha256").clone(),
    );
    set(report, "absolute_tolerance_points", string(TOLERANCE_TEXT));
    set(report, "status", string("fixture_verified"));
    let Some(actual) = &options.actual else {
        return Ok(());
    };
    let before = fingerprint(actual)?;
    let results = read_results(actual, requests_digest.as_str().unwrap_or_default())?;
    let differences = comparison(&fixture.expected, &results)?;
    require(
        fingerprint(actual)? == before,
        "candidate results changed during comparison",
    )?;
    let after = load_fixture(&options.fixture)?;
    require(
        after.capture == fixture.capture,
        "fixture changed during comparison",
    )?;
    let status = if differences.is_empty() {
        "matched"
    } else {
        "different"
    };
    set(report, "status", string(status));
    set(report, "actual_sha256", string(before));
    set(report, "differences", Json::Array(differences));
    Ok(())
}

fn execute(options: &Options, comparison: &Comparison<'_>) -> Result<Json, String> {
    if let Some(output) = &options.output
        && output.exists()
    {
        return Err("output directory already exists; choose a new path".into());
    }
    let mut report = vec![
        ("word_review_status".to_owned(), string("unreviewed")),
        ("status".to_owned(), string("error")),
    ];
    if let Err(error) = measure(options, comparison, &mut report) {
        set(&mut report, "status", string("error"));
        set(&mut report, "error", string(error));
    }
    let report = Json::Object(report);
    if let Some(output) = &options.output {
        fs::create_dir_all(output).map_err(|error| format!("{}: {error}", output.display()))?;
        let path = output.join("report.json");
        fs::write(&path, pretty(&report) + "\n")
            .map_err(|error| format!("{}: {error}", path.display()))?;
    }
    Ok(report)
}

fn succeeded(report: &Json) -> bool {
    matches!(
        at(report, "status").as_str(),
        Some("fixture_verified" | "matched")
    )
}

pub(crate) fn run(arguments: Arguments) -> Outcome {
    let Some(options) = Options::parse(arguments)? else {
        return Ok(ExitCode::SUCCESS);
    };
    let report = execute(&options, &compare)?;
    println!("{}", compact(&report));
    Ok(if succeeded(&report) {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::TemporaryDirectory;
    use serde_json::{Value, json};

    struct Setup {
        directory: TemporaryDirectory,
        expected: Value,
        actual: PathBuf,
    }

    impl Setup {
        fn new() -> Self {
            let directory = TemporaryDirectory::new();
            let root = directory.0.clone();
            fs::write(root.join("font.ttf"), b"\0\x01\0\0fixture").unwrap();
            fs::write(root.join("LICENSE"), "Fixture license").unwrap();
            let write = |name: &str, value: &Value| {
                fs::write(root.join(name), value.to_string() + "\n").unwrap();
                fingerprint(&root.join(name)).unwrap()
            };
            let cases: Vec<_> = ["first", "second"]
                .map(|name| {
                    json!({"id": name, "font_id": "face", "size_points": 12,
                           "layout_kerning": true, "text": "AV☃字"})
                })
                .into();
            let requests_digest = write(
                "requests.json",
                &json!({
                    "schema_version": 1, "coordinate_unit": "point",
                    "fonts": [{"id": "face", "family": "Fixture", "variant": "regular",
                               "format": "truetype", "face_index": 0, "path": "font.ttf",
                               "sha256": fingerprint(&root.join("font.ttf")).unwrap(),
                               "size_bytes": 11, "source_url": "https://example.com/font.ttf"}],
                    "cases": cases,
                }),
            );
            let measurements: Vec<_> = ["first", "second"]
                .map(|name| {
                    json!({"id": name, "font_id": "face", "variant": "regular",
                           "metric_source": "declared", "prepared_text": "AV??",
                           "missing_codepoints": [9731, 23383], "layout_width": 28,
                           "paint_width": 28, "ascent": 9, "descent": 3, "height": 13})
                })
                .into();
            let expected = json!({
                "schema_version": 1, "requests_sha256": requests_digest,
                "measurements": measurements,
            });
            let expected_digest = write("expected.json", &expected);
            write(
                "capture.json",
                &json!({
                    "schema_version": 1, "requests": "requests.json",
                    "requests_sha256": requests_digest, "expected": "expected.json",
                    "expected_sha256": expected_digest, "case_count": 2,
                    "coordinate_unit": "point", "absolute_tolerance_points": 1e-9,
                    "captured_at_utc": "2026-01-01T00:00:00+00:00",
                    "word_review_status": "unreviewed", "license": "LICENSE",
                    "license_sha256": fingerprint(&root.join("LICENSE")).unwrap(),
                }),
            );
            write("actual.json", &expected);
            Self {
                actual: root.join("actual.json"),
                directory,
                expected,
            }
        }
        fn root(&self) -> &Path {
            &self.directory.0
        }
        fn write_actual(&self, value: &Value) {
            fs::write(&self.actual, value.to_string() + "\n").unwrap();
        }
        fn results(&self) -> Result<Results, String> {
            let digest = self.expected["requests_sha256"].as_str().unwrap();
            read_results(&self.actual, digest)
        }
        fn run(&self, actual: bool, output: Option<&str>) -> Result<Json, String> {
            self.run_with(actual, output, &compare)
        }
        fn run_with(
            &self,
            actual: bool,
            output: Option<&str>,
            comparison: &Comparison<'_>,
        ) -> Result<Json, String> {
            let options = Options {
                fixture: self.root().to_path_buf(),
                actual: actual.then(|| self.actual.clone()),
                output: output.map(|name| self.root().join(name)),
            };
            execute(&options, comparison)
        }
    }

    fn replace(record: &mut Json, name: &str, value: Json) {
        let Json::Object(fields) = record else {
            panic!("a record object")
        };
        set(fields, name, value);
    }

    fn first_field(differences: &[Json]) -> &str {
        differences[0].get("field").and_then(Json::as_str).unwrap()
    }

    fn status(report: &Json) -> &str {
        report.get("status").and_then(Json::as_str).unwrap()
    }

    #[test]
    fn checked_in_capture_is_complete_and_self_contained() {
        let fixture =
            load_fixture(&crate::repository().join("tests/fixtures/measurements")).unwrap();
        assert_eq!(fixture.capture.get("case_count"), Some(&Json::Integer(960)));
        assert_eq!(fixture.expected.len(), 960);
        let fonts = fixture
            .requests
            .get("fonts")
            .and_then(Json::as_array)
            .unwrap();
        assert_eq!(fonts.len(), 20);
        let declared = fonts
            .iter()
            .filter(|font| font.get("format").and_then(Json::as_str) != Some("core"))
            .count();
        assert_eq!(declared, 8);
    }

    #[test]
    fn result_order_and_numeric_spelling_do_not_change_measurement() {
        let setup = Setup::new();
        let expected = setup.results().unwrap();
        let mut candidate = setup.expected.clone();
        let measurements = candidate["measurements"].as_array_mut().unwrap();
        measurements.reverse();
        measurements[0]["layout_width"] = json!(28.0);
        setup.write_actual(&candidate);
        assert_eq!(compare(&expected, &setup.results().unwrap()).unwrap(), []);
    }

    #[test]
    fn point_tolerance_is_tiny_and_applies_only_to_metrics() {
        let setup = Setup::new();
        let expected = setup.results().unwrap();
        let mut candidate = expected.clone();
        replace(
            candidate.get_mut("first").unwrap(),
            "layout_width",
            Json::Decimal(28.000000001),
        );
        assert_eq!(compare(&expected, &candidate).unwrap(), []);
        replace(
            candidate.get_mut("first").unwrap(),
            "layout_width",
            Json::Decimal(28.000000001001),
        );
        assert_eq!(
            first_field(&compare(&expected, &candidate).unwrap()),
            "layout_width"
        );
        let mut candidate = expected.clone();
        let codepoints = Json::Array(vec![Json::Integer(9732), Json::Integer(23383)]);
        replace(
            candidate.get_mut("first").unwrap(),
            "missing_codepoints",
            codepoints,
        );
        assert_eq!(
            first_field(&compare(&expected, &candidate).unwrap()),
            "missing_codepoints"
        );
    }

    #[test]
    fn font_text_and_missing_glyph_order_are_exact() {
        let setup = Setup::new();
        let expected = setup.results().unwrap();
        for (field, value) in [
            ("font_id", string("another_face")),
            ("variant", string("bold")),
            ("metric_source", string("core")),
            ("prepared_text", string("AV?")),
            (
                "missing_codepoints",
                Json::Array(vec![Json::Integer(23383), Json::Integer(9731)]),
            ),
        ] {
            let mut candidate = expected.clone();
            replace(candidate.get_mut("first").unwrap(), field, value);
            assert_eq!(first_field(&compare(&expected, &candidate).unwrap()), field);
        }
    }

    #[test]
    fn missing_extra_and_duplicate_cases_cannot_pass() {
        let setup = Setup::new();
        let expected = setup.results().unwrap();
        let mut candidate = expected.clone();
        candidate.remove("second");
        candidate.insert("extra".into(), candidate["first"].clone());
        let reasons: BTreeSet<_> = compare(&expected, &candidate)
            .unwrap()
            .iter()
            .filter_map(|difference| {
                difference
                    .get("reason")
                    .and_then(Json::as_str)
                    .map(str::to_owned)
            })
            .collect();
        assert_eq!(
            reasons,
            BTreeSet::from(["missing result".into(), "unexpected result".into()])
        );
        let mut duplicated = setup.expected.clone();
        let first = duplicated["measurements"][0].clone();
        duplicated["measurements"]
            .as_array_mut()
            .unwrap()
            .push(first);
        setup.write_actual(&duplicated);
        assert!(
            setup
                .results()
                .unwrap_err()
                .contains("duplicate identifier")
        );
    }

    #[test]
    fn changed_requests_expected_fonts_and_license_fail_integrity() {
        let setup = Setup::new();
        for name in ["requests.json", "expected.json", "font.ttf", "LICENSE"] {
            let path = setup.root().join(name);
            let original = fs::read(&path).unwrap();
            fs::write(&path, [original.as_slice(), b" "].concat()).unwrap();
            let error = load_fixture(setup.root()).err().unwrap();
            fs::write(&path, original).unwrap();
            assert!(error.contains("fingerprint mismatch"), "{name}: {error}");
        }
    }

    #[test]
    fn nonfinite_boolean_and_ambiguous_json_are_rejected() {
        let setup = Setup::new();
        let original = setup.expected.to_string();
        for value in ["NaN", "Infinity", "true", "\"28\""] {
            let contents =
                original.replacen("\"paint_width\":28", &format!("\"paint_width\":{value}"), 1);
            assert_ne!(contents, original);
            fs::write(&setup.actual, contents).unwrap();
            assert!(setup.results().is_err(), "{value}");
        }
        fs::write(&setup.actual, r#"{"schema_version":1,"schema_version":1}"#).unwrap();
        assert!(
            setup
                .results()
                .unwrap_err()
                .contains("duplicate JSON field")
        );
        let mut candidate = setup.expected.clone();
        candidate["schema_version"] = json!(true);
        setup.write_actual(&candidate);
        assert!(setup.results().unwrap_err().contains("schema version"));
    }

    #[test]
    fn unknown_fields_and_changed_request_identity_are_rejected() {
        let setup = Setup::new();
        let mut candidate = setup.expected.clone();
        candidate["measurements"][0]["ignored_field"] = json!(1);
        setup.write_actual(&candidate);
        assert!(setup.results().unwrap_err().contains("fields"));
        let mut candidate = setup.expected.clone();
        candidate["requests_sha256"] = json!("0".repeat(64));
        setup.write_actual(&candidate);
        assert!(
            setup
                .results()
                .unwrap_err()
                .contains("request fingerprint mismatch")
        );
    }

    #[test]
    fn unrepresentable_metric_arithmetic_reports_an_error() {
        let setup = Setup::new();
        let contents = fs::read_to_string(&setup.actual).unwrap().replacen(
            "\"paint_width\":28",
            "\"paint_width\":1e999999999",
            1,
        );
        fs::write(&setup.actual, contents).unwrap();
        let report = setup.run(true, None).unwrap();
        assert!(!succeeded(&report));
        assert_eq!(status(&report), "error");
        let large = Json::Decimal(1e300);
        assert!(within_tolerance(&large, &Json::Integer(28)).is_err());
    }

    #[test]
    fn fixture_cannot_reference_absolute_or_escaping_paths() {
        let setup = Setup::new();
        let absolute = setup.root().join("font.ttf");
        for relative in [absolute.to_str().unwrap(), "../font.ttf"] {
            assert!(local_file(setup.root(), relative).is_err(), "{relative}");
        }
        let outside = TemporaryDirectory::new();
        let target = outside.0.join("outside.ttf");
        fs::write(&target, b"outside").unwrap();
        std::os::unix::fs::symlink(&target, setup.root().join("linked.ttf")).unwrap();
        let error = local_file(setup.root(), "linked.ttf").unwrap_err();
        assert!(error.contains("leaves its directory"));
    }

    #[test]
    fn fixture_verification_does_not_claim_candidate_or_word_conformance() {
        let setup = Setup::new();
        let report = setup.run(false, None).unwrap();
        assert!(succeeded(&report));
        assert_eq!(status(&report), "fixture_verified");
        let review = report.get("word_review_status").and_then(Json::as_str);
        assert_eq!(review, Some("unreviewed"));
        let report = setup.run(true, Some("review")).unwrap();
        assert!(succeeded(&report));
        assert_eq!(status(&report), "matched");
        let digest = fingerprint(&setup.actual).unwrap();
        assert_eq!(
            report.get("actual_sha256").and_then(Json::as_str),
            Some(digest.as_str())
        );
        assert!(setup.run(false, Some("review")).is_err());
    }

    #[test]
    fn mismatch_is_failure_and_is_written_to_the_report() {
        let setup = Setup::new();
        let mut candidate = setup.expected.clone();
        candidate["measurements"][0]["height"] = json!(14);
        setup.write_actual(&candidate);
        let report = setup.run(true, Some("review")).unwrap();
        assert!(!succeeded(&report));
        assert_eq!(status(&report), "different");
        let differences = report.get("differences").and_then(Json::as_array).unwrap();
        assert_eq!(first_field(differences), "height");
        let saved = Json::read(&setup.root().join("review/report.json")).unwrap();
        assert_eq!(saved, report);
    }

    #[test]
    fn candidate_and_fixture_changes_during_comparison_are_errors() {
        let setup = Setup::new();
        for name in ["actual.json", "font.ttf"] {
            let path = setup.root().join(name);
            let original = fs::read(&path).unwrap();
            let change_input = |_: &Results, _: &Results| {
                fs::write(&path, [original.as_slice(), b" "].concat()).unwrap();
                Ok(Vec::new())
            };
            let report = setup.run_with(true, None, &change_input).unwrap();
            fs::write(&path, &original).unwrap();
            assert!(!succeeded(&report), "{name}");
            assert_eq!(status(&report), "error");
        }
    }
}
