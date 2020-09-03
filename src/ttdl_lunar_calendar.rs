use lunardate::LunarDate;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
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

const TTDL_LUNAR_CALENDAR_PLUGIN_NAME: &str = "ttdl-lunar-calendar";
const TTDL_LUNAR_CALENDAR_TAG_KEY: &str = "!lunar-calendar";
const TTDL_LUNAR_CALENDAR_TAG_SEPARATOR: &str = ",";

/// Convert lunar dates to gregorian dates for [todo.txt](https://github.com/todotxt/todo.txt)
/// elements specified by the `!lunar-calendar` special tag with a comma separated list, an item of
/// the list can be one of the following:
///
/// - A special tag prefixed with `#` (e.g. `#due` for "due")
/// - An optional (e.g. `created` for "Creation Date")
///
/// > Tips:
/// >
/// > - For definitions of "Special Tags" or "Optional", please see
/// >   [todo.txt format](https://github.com/todotxt/todo.txt) for details.
/// > - For optional names from [TTDL](https://github.com/VladimirMarkelov/ttdl),
/// >   please see [Plugin interaction](https://github.com/VladimirMarkelov/ttdl#plugin-interaction)
/// >   for details.
pub fn run(input: &str) -> Result<String, Box<dyn Error>> {
    /*
     * Parse input
     */
    let message_dto: TtdlPluginMessageDto = serde_json::from_str(input)?;
    let original_message: PolishedTtdlPluginMessage = message_dto.into();
    let mut current_message = original_message.clone();
    let conversion_pointers = current_message.parse_plugin_value();

    /*
     * Convert
     */
    let mut error_message = None;
    let mut history = HashSet::new();
    for item in conversion_pointers {
        let (expression, sorted_value) = match item {
            ConversionPointer::SpecialTags(expression, name) => {
                (expression, current_message.special_tags.get_mut(&name))
            }
            ConversionPointer::Optional(expression, name) => {
                let sorted_value = match current_message.optional.as_mut() {
                    Some(x) => x.get_mut(&name),
                    None => None,
                };
                (expression, sorted_value)
            }
        };
        if history.contains(&expression) {
            error_message = Some(format!(
                r#"duplicated "{expression}""#,
                expression = expression
            ));
            break;
        }
        if sorted_value.is_none() {
            error_message = Some(format!(
                r#"not found "{expression}""#,
                expression = expression
            ));
            break;
        }
        let (value, _) = sorted_value.unwrap();
        let lunar_date_source = parse_ttdl_lunar_date(&value);
        if lunar_date_source.is_none() {
            error_message = Some(format!(
                r#"unexpected format for "{expression}""#,
                expression = expression
            ));
            break;
        }
        match to_ttdl_solar_date_string(lunar_date_source.unwrap()) {
            Ok(solar_date_string) => {
                *value = solar_date_string;
                history.insert(expression);
            }
            Err(err) => {
                error_message = Some(format!(
                    r#"unexpected value for "{expression}": {reason}"#,
                    expression = expression,
                    reason = err
                ));
                break;
            }
        }
    }

    /*
     * Generate output
     */
    let new_message = match error_message {
        Some(x) => {
            let mut current_message = original_message;
            current_message.description = format!(
                "[ERR({plugin_name}) {error_message}] {original_description}",
                plugin_name = TTDL_LUNAR_CALENDAR_PLUGIN_NAME,
                error_message = x,
                original_description = current_message.description
            );
            current_message
        }
        None => current_message,
    };
    let new_message_dto: TtdlPluginMessageDto = new_message.into();
    let output = serde_json::to_string(&new_message_dto)?;
    Ok(output)
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

fn to_ttdl_solar_date_string(source: LunarDateSource) -> Result<String, Box<dyn Error>> {
    let lunar_date = LunarDate::new(source.year, source.month, source.day, false);
    let solar_date = lunar_date.to_solar_date()?;
    let solar_string = solar_date.format("%Y-%m-%d").to_string();
    Ok(solar_string)
}

// https://github.com/VladimirMarkelov/ttdl#plugin-interaction
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TtdlPluginMessageDto {
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    optional: Option<Vec<HashMap<String, String>>>,
    special_tags: Vec<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PolishedTtdlPluginMessage {
    description: String,
    optional: Option<HashMap<String, (String, usize)>>,
    special_tags: HashMap<String, (String, usize)>,
}

enum ConversionPointer {
    Optional(String, String),
    SpecialTags(String, String),
}

impl From<TtdlPluginMessageDto> for PolishedTtdlPluginMessage {
    fn from(standard_message: TtdlPluginMessageDto) -> Self {
        let transformer = |maps: Vec<HashMap<String, String>>| {
            maps.iter()
                .enumerate()
                .flat_map(|(index, map)| {
                    map.iter()
                        .map(move |(k, v)| (k.to_string(), (v.to_string(), index)))
                })
                .collect()
        };
        let polished_optional = standard_message.optional.map(transformer);
        let polished_special_tags = transformer(standard_message.special_tags);
        Self {
            description: standard_message.description,
            optional: polished_optional,
            special_tags: polished_special_tags,
        }
    }
}

impl From<PolishedTtdlPluginMessage> for TtdlPluginMessageDto {
    fn from(polished_message: PolishedTtdlPluginMessage) -> Self {
        let transformer = |map: HashMap<String, (String, usize)>| {
            let mut vec: Vec<_> = map.iter().collect();
            vec.sort_by(|(_, (_, index_a)), (_, (_, index_b))| index_a.cmp(index_b));
            vec.iter()
                .map(|(name, (value, _))| {
                    let mut map = HashMap::new();
                    map.insert(name.to_string(), value.to_string());
                    map
                })
                .collect()
        };
        let standard_optional = polished_message.optional.map(transformer);
        let standard_special_tags = transformer(polished_message.special_tags);
        Self {
            description: polished_message.description,
            optional: standard_optional,
            special_tags: standard_special_tags,
        }
    }
}

impl PolishedTtdlPluginMessage {
    pub fn get_special_tag_value(&self, key: &str) -> Option<String> {
        let (value, _) = &self.special_tags.get(key)?;
        Some(value.to_string())
    }

    pub fn parse_plugin_value(&self) -> Vec<ConversionPointer> {
        // Should always contain the plugin value for the running plugin.
        let plugin_value = self
            .get_special_tag_value(TTDL_LUNAR_CALENDAR_TAG_KEY)
            .unwrap();
        plugin_value
            .split(TTDL_LUNAR_CALENDAR_TAG_SEPARATOR)
            .map(|expression| match expression.strip_prefix("#") {
                Some(x) => ConversionPointer::SpecialTags(expression.to_string(), x.to_string()),
                None => ConversionPointer::Optional(expression.to_string(), expression.to_string()),
            })
            .collect()
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
    fn test_to_ttdl_solar_date_string() {
        assert_eq!(
            "2000-02-05",
            to_ttdl_solar_date_string(LunarDateSource::new(2000, 1, 1)).unwrap()
        );
    }
}
