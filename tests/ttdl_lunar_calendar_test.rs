use serde_json::{json, Value as JsonValue};
use std::error::Error;

#[test]
fn test_run_without_lunar_due() {
    let input = json!({
        "description": "test",
        "specialTags": [
            {
                "due": "2000-01-01"
            }
        ]
    });
    let expected_output = input.clone();
    let actual_output = run(input).unwrap();
    assert_eq!(expected_output, actual_output);
}

#[test]
fn test_run_disabled() {
    let input = json!({
        "description": "test",
        "specialTags": [
            {
                "due": "2000-01-01"
            },
            {
                "!lunar-calendar": false
            }
        ]
    });
    let expected_output = json!({
        "description": "test",
        "specialTags": [
            {
                "due": "2000-01-01"
            }
        ]
    });
    let actual_output = run(input).unwrap();
    assert_eq!(expected_output, actual_output);
}

#[test]
fn test_run_enabled_with_bool() {
    test_run_enabled_with_single_due()
}

#[test]
fn test_run_enabled_with_bool_string() {
    let input = json!({
        "description": "test",
        "specialTags": [
            {
                "due": "2000-01-01"
            },
            {
                "!lunar-calendar": "true"
            }
        ]
    });
    let expected_output = json!({
        "description": "test",
        "specialTags": [
            {
                "due": "2000-02-05"
            }
        ]
    });
    let actual_output = run(input).unwrap();
    assert_eq!(expected_output, actual_output);
}

#[test]
fn test_run_enabled_without_dues() {
    let input = json!({
        "description": "test",
        "specialTags": [
            {
                "!lunar-calendar": true
            }
        ]
    });
    let expected_output = json!({
        "description": "test",
        "specialTags": []
    });
    let actual_output = run(input).unwrap();
    assert_eq!(expected_output, actual_output);
}

#[test]
fn test_run_enabled_with_single_due() {
    let input = json!({
        "description": "test",
        "specialTags": [
            {
                "due": "2000-01-01"
            },
            {
                "!lunar-calendar": true
            }
        ]
    });
    let expected_output = json!({
        "description": "test",
        "specialTags": [
            {
                "due": "2000-02-05"
            }
        ]
    });
    let actual_output = run(input).unwrap();
    assert_eq!(expected_output, actual_output);
}

#[test]
fn test_run_enabled_with_multiple_dues() {
    let input = json!({
        "description": "test",
        "specialTags": [
            {
                "due": "2000-01-01"
            },
            {
                "due": "2000-01-02"
            },
            {
                "!lunar-calendar": true
            }
        ]
    });
    let expected_output = json!({
        "description": "test",
        "specialTags": [
            {
                "due": "2000-02-05"
            },
            {
                "due": "2000-02-06"
            }
        ]
    });
    let actual_output = run(input).unwrap();
    assert_eq!(expected_output, actual_output);
}

fn run(input: JsonValue) -> Result<JsonValue, Box<dyn Error>> {
    let input = input.to_string();
    let output = ttdl_lunar_calendar::run(&input)?;
    let output: JsonValue = serde_json::from_str(&output)?;
    Ok(output)
}
