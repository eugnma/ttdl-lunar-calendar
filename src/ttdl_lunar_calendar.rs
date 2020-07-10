use lunardate::LunarDate;
use serde_json::{json, Value as JsonValue};
use std::error::Error;

/*
 * Related TTDL (https://github.com/VladimirMarkelov/ttdl) features:
 *   - When executing command list TTDL may call an external application (a "plugin") to transform
 *     the description and/or tags
 *   - ttdl done moves a recurrent todo's due date to the next one, but it does not check if the new
 *     due date is in the future (it is by design)
 *   - All dates are entered and displayed in format - YYYY-MM-DD
 *     (4 year digits - 2 month digits - 2 day digits)
 */

const TTDL_STD_TAGS: &str = "specialTags";
const TTDL_DUE_TAG: &str = "due";
const TTDL_LUNAR_CALENDAR_TAG: &str = "!lunar-calendar";

/// Convert lunar dates to gregorian dates from all "due" tags if exists a single "!lunar-calendar"
/// tag with the value `true`.
pub fn run(input: &str) -> Result<String, Box<dyn Error>> {
    // Extract valid related tags info.
    let default_tags = Vec::new();
    let mut json: JsonValue = serde_json::from_str(input)?;
    let tags = json
        .get(TTDL_STD_TAGS)
        .and_then(|x| x.as_array())
        .unwrap_or(&default_tags);
    let mut lunar_calendar_tag_info = None;
    let mut due_tags_info = Vec::new();
    for (index, tag) in tags.iter().enumerate() {
        let tag = tag.as_object();
        if tag.is_none() || tag.unwrap().is_empty() {
            break;
        }
        let (first_item_key, first_item_value) = tag.unwrap().iter().next().unwrap();
        match first_item_key.as_str() {
            TTDL_DUE_TAG => {
                if let Some(due_tag_value) = first_item_value.as_str() {
                    due_tags_info.push((index, due_tag_value.to_string()));
                }
            }
            TTDL_LUNAR_CALENDAR_TAG => {
                let normalized_tag_value = match first_item_value {
                    JsonValue::Bool(value) => *value,
                    JsonValue::String(value) => value.parse().unwrap_or_default(),
                    _ => false,
                };
                if lunar_calendar_tag_info.is_some() {
                    panic!(format!(r#"Found duplicate "{}""#, TTDL_LUNAR_CALENDAR_TAG));
                }
                lunar_calendar_tag_info = Some((index, normalized_tag_value));
            }
            _ => (),
        }
    }
    if lunar_calendar_tag_info.is_none() {
        return Ok(input.to_string());
    }

    // Prepare for JSON changes.
    let tags = json[TTDL_STD_TAGS].as_array_mut().unwrap();
    let (lunar_calendar_tag_index, lunar_calendar_tag_value) = lunar_calendar_tag_info.unwrap();

    // Convert all values from the "due" tags and let TTDL handle multiple "due" tags.
    if lunar_calendar_tag_value {
        for (due_tag_index, due_tag_value) in due_tags_info {
            let lunar_date_source = parse_ttdl_lunar_date(&due_tag_value);
            if lunar_date_source.is_none() {
                panic!("Unexpected due date format")
            }
            let solar_due_string = to_ttdl_solar_string(lunar_date_source.unwrap())?;
            *tags.get_mut(due_tag_index).unwrap() = json!({ TTDL_DUE_TAG: solar_due_string });
        }
    }

    // Remove the "!lunar-calendar" tag.
    tags.remove(lunar_calendar_tag_index);

    // Return the updated todo item.
    Ok(json.to_string())
}

fn parse_ttdl_lunar_date(lunar_date: &str) -> Option<LunarDateSource> {
    let lunar_items: Vec<_> = lunar_date.split('-').collect();
    let mut lunar_date_source = LunarDateSource::new(1, 1, 1);
    if lunar_items.len() != 3 {
        return None;
    }
    match lunar_items[0].parse() {
        Err(_) => return None,
        Ok(value) => lunar_date_source.year = value,
    };
    match lunar_items[1].parse() {
        Ok(0) | Err(_) => return None,
        Ok(value) => lunar_date_source.month = value,
    };
    match lunar_items[2].parse() {
        Ok(0) | Err(_) => return None,
        Ok(value) => lunar_date_source.day = value,
    };
    Some(lunar_date_source)
}

fn to_ttdl_solar_string(source: LunarDateSource) -> Result<String, Box<dyn Error>> {
    let lunar_date = LunarDate::new(source.year, source.month, source.day, false);
    let solar_date = lunar_date.to_solar_date()?;
    let solar_string = solar_date.format("%Y-%m-%d").to_string();
    Ok(solar_string)
}

#[derive(Debug, PartialEq)]
struct LunarDateSource {
    year: i32,
    month: u32,
    day: u32,
}

impl LunarDateSource {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ttdl_lunar_date() {
        assert_eq!(None, parse_ttdl_lunar_date("YYYY-MM-DD"));
        assert_eq!(None, parse_ttdl_lunar_date("2000-01-00"));
        assert_eq!(None, parse_ttdl_lunar_date("2000-00-01"));
        assert_eq!(None, parse_ttdl_lunar_date("01-01"));
        assert_eq!(None, parse_ttdl_lunar_date("-01-01"));
        assert_eq!(
            Some(LunarDateSource::new(0, 1, 1)),
            parse_ttdl_lunar_date("0000-01-01")
        );
        assert_eq!(
            Some(LunarDateSource::new(2000, 1, 1)),
            parse_ttdl_lunar_date("2000-01-01")
        );
    }

    #[test]
    fn test_to_ttdl_solar_string() {
        assert_eq!(
            "2000-02-05",
            to_ttdl_solar_string(LunarDateSource::new(2000, 1, 1)).unwrap()
        );
    }
}
