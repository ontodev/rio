use rio_testsuite::manifest::TestManifest;
use rio_testsuite::parser_evaluator::*;
use rio_testsuite::report::TestOutcome;
use std::error::Error;
use std::path::PathBuf;

fn get_test_path() -> PathBuf {
    let mut base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    base_path.push("rdf-tests");
    return base_path;
}

fn run_testsuite(
    manifest_uri: String,
    number_of_expected_errors: usize,
) -> Result<(), Box<dyn Error>> {
    let test_path = get_test_path();
    let manifest = TestManifest::new(manifest_uri, |url| parse_w3c_rdf_test_file(url, &test_path));

    let results = evaluate_parser_tests(manifest, |url| parse_w3c_rdf_test_file(url, &test_path))?;

    let mut errors = Vec::default();
    for result in results {
        if let TestOutcome::Failed { error } = result.outcome {
            errors.push(format!("{}: failed with error {}", result.test, error))
        }
    }

    assert!(
        errors.len() <= number_of_expected_errors,
        "\n{}\n",
        errors.join("\n")
    );
    Ok(())
}

#[test]
fn ntriples_w3c_testsuite() -> Result<(), Box<dyn Error>> {
    run_testsuite(
        "http://www.w3.org/2013/N-TriplesTests/manifest.ttl".to_owned(),
        0,
    )
}

#[test]
fn turtle_w3c_testsuite() -> Result<(), Box<dyn Error>> {
    run_testsuite(
        "http://www.w3.org/2013/TurtleTests/manifest.ttl".to_owned(),
        6,
    )
}
