use serde_json::{json, Value as JsonValue};
use std::error::Error;

#[test]
fn test_run_with_empty_lunar_calendar() {
    // There is no such case because the `value` of special tags must consist of
    // non-whitespace characters, see
    // https://github.com/todotxt/todo.txt#additional-file-format-definitions
    // for more details.
}

#[test]
fn test_run_with_multiple_lunar_calendar_special_tags() {
    // There is no such case because special tags is a dictionary, a item has
    // multiple special tags with the same key will be overridden by others, see
    // https://github.com/todotxt/todo.txt#additional-file-format-definitions
    // for more details.
}

#[test]
fn test_run_with_lunar_calendar_but_invalid_lunar_calendar_format() {
    let input = json!({
        "description": "test",
        "specialTags": [
            {
                "due": "2000-01-01"
            },
            {
                "tag": "2000-01-01"
            },
            {
                "!lunar-calendar": "#due, #tag"
            }
        ]
    });
    let expected_output = json!({
        "description": r##"[ERR(ttdl-lunar-calendar) not found " #tag"] test"##,
        "specialTags": [
            {
                "due": "2000-01-01"
            },
            {
                "tag": "2000-01-01"
            },
            {
                "!lunar-calendar": "#due, #tag"
            }
        ]
    });
    let actual_output = run(input).unwrap();
    assert_eq!(expected_output, actual_output);
}

#[test]
fn test_run_with_lunar_calendar_but_duplicated_items() {
    let input = json!({
        "description": "test",
        "specialTags": [
            {
                "due": "2000-01-01"
            },
            {
                "tag": "2000-01-01"
            },
            {
                "!lunar-calendar": "#due,#tag,#tag"
            }
        ]
    });
    let expected_output = json!({
        "description": r##"[ERR(ttdl-lunar-calendar) duplicated "#tag"] test"##,
        "specialTags": [
            {
                "due": "2000-01-01"
            },
            {
                "tag": "2000-01-01"
            },
            {
                "!lunar-calendar": "#due,#tag,#tag"
            }
        ]
    });
    let actual_output = run(input).unwrap();
    assert_eq!(expected_output, actual_output);
}

#[test]
fn test_run_with_lunar_calendar_but_special_tag_not_found() {
    let input = json!({
        "description": "test",
        "specialTags": [
            {
                "due": "2000-01-01"
            },
            {
                "!lunar-calendar": "#due,#nonexistent"
            }
        ]
    });
    let expected_output = json!({
        "description": r##"[ERR(ttdl-lunar-calendar) not found "#nonexistent"] test"##,
        "specialTags": [
            {
                "due": "2000-01-01"
            },
            {
                "!lunar-calendar": "#due,#nonexistent"
            }
        ]
    });
    let actual_output = run(input).unwrap();
    assert_eq!(expected_output, actual_output);
}

#[test]
fn test_run_with_lunar_calendar_but_optional_not_found() {
    let input = json!({
        "description": "test",
        "specialTags": [
            {
                "due": "2000-01-01"
            },
            {
                "!lunar-calendar": "#due,created"
            }
        ]
    });
    let expected_output = json!({
        "description": r##"[ERR(ttdl-lunar-calendar) not found "created"] test"##,
        "specialTags": [
            {
                "due": "2000-01-01"
            },
            {
                "!lunar-calendar": "#due,created"
            }
        ]
    });
    let actual_output = run(input).unwrap();
    assert_eq!(expected_output, actual_output);
}

#[test]
fn test_run_with_lunar_calendar_but_unexpected_date_format() {
    let input = json!({
        "description": "test",
        "specialTags": [
            {
                "due": "2000-01-01"
            },
            {
                "tag": "2000-01-aa"
            },
            {
                "!lunar-calendar": "#due,#tag"
            }
        ]
    });
    let expected_output = json!({
        "description": r##"[ERR(ttdl-lunar-calendar) unexpected format for "#tag"] test"##,
        "specialTags": [
            {
                "due": "2000-01-01"
            },
            {
                "tag": "2000-01-aa"
            },
            {
                "!lunar-calendar": "#due,#tag"
            }
        ]
    });
    let actual_output = run(input).unwrap();
    assert_eq!(expected_output, actual_output);
}

#[test]
fn test_run_with_lunar_calendar_but_unexpected_solar_date() {
    let input = json!({
        "description": "test",
        "specialTags": [
            {
                "due": "2000-01-01"
            },
            {
                "tag": "2000-01-31"
            },
            {
                "!lunar-calendar": "#due,#tag"
            }
        ]
    });
    let expected_description_head = r##"[ERR(ttdl-lunar-calendar) unexpected value for "#tag": "##;
    let actual_output = run(input).unwrap();
    let actual_description = actual_output["description"].as_str().unwrap().to_string();
    assert!(actual_description.contains(expected_description_head));
}

#[test]
fn test_run_with_lunar_calendar_ok() {
    let input = json!({
        "description": "test",
        "specialTags": [
            {
                "due": "2000-01-01"
            },
            {
                "tag-lunar-calendar": "2000-01-01"
            },
            {
                "tag-gregorian-calendar": "2000-01-01"
            },
            {
                "!lunar-calendar": "created,#due,#tag-lunar-calendar"
            }
        ],
        "optional": [
            {
                "created": "2000-01-01"
            },
            {
                "finished": "2000-01-01"
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
                "tag-lunar-calendar": "2000-02-05"
            },
            {
                "tag-gregorian-calendar": "2000-01-01"
            },
            {
                "!lunar-calendar": "created,#due,#tag-lunar-calendar"
            }
        ],
        "optional": [
            {
                "created": "2000-02-05"
            },
            {
                "finished": "2000-01-01"
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
