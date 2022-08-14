use regex::Regex;

/// A list of characters and their corresponding HTML entities
static HTML_ESCAPABLE: [[&str; 2]; 31] = [
  [r"!", "&#33;"], // Exclamation mark
  ["\"", "&#34;"], // Quotation mark
  [r"#", "&#35;"], // Number sign
  [r"$", "&#36;"], // Dollar sign
  [r"%", "&#37;"], // Percent sign
  [r"&", "&#38;"], // Ampersand
  [r"'", "&#39;"], // Apostrophe
  [r"(", "&#40;"], // Opening/Left Parenthesis
  [r")", "&#41;"], // Closing/Right Parenthesis
  [r"*", "&#42;"], // Asterisk
  [r"+", "&#43;"], // Plus sign
  [r",", "&#44;"], // Comma
  [r"-", "&#45;"], // Hyphen
  [r".", "&#46;"], // Period
  [r":", "&#58;"], // Colon
  [r";", "&#59;"], // Semicolon
  [r"<", "&#60;"], // Less-than
  [r"=", "&#61;"], // Equals sign
  [r">", "&#62;"], // Greater than
  [r"?", "&#63;"], // Question mark
  [r"@", "&#64;"], // At sign
  [r"[", "&#91;"], // Opening/Left square bracket
  [r"\", "&#92;"], // Backslash
  [r"]", "&#93;"], // Closing/Right square bracket
  [r"^", "&#94;"], // Caret
  [r"_", "&#95;"], // Underscore
  [r"`", "&#96;"], // Grave accent
  ["{", "&#123;"], // Opening/Left curly brace
  ["|", "&#124;"], // Vertical bar
  ["}", "&#125;"], // Closing/Right curly brace
  ["~", "&#126;"]  // Tilde
];

/// **Description**:
///
/// Escapes and sanitizes a string of HTML using `lib::HTML_ESCAPABLE` to provide a list of values to escape
///
/// **References**:
/// * [HTML Sanitization](https://en.wikipedia.org/wiki/HTML_sanitization)
/// * [HTML Entity List](https://www.freeformatter.com/html-entities.html#misc-html-entities)
pub fn escape_html(html: &String) -> String {
  // Replace the characters with their matching HTML entities
  let mut result: String = html.to_owned();
  for [character, replacement] in HTML_ESCAPABLE {
    result = result.replace(character, replacement);
  }
  result
}

/// **Description**:
///
/// Convert a string to slug case
///
/// **References**:
/// * [What is a slug?](https://stackoverflow.com/a/19335586/10415695)
pub fn string_to_slug(value: &String) -> String {
  // remove all non alpha numeric values
  let re_non_alpha_num: Regex = Regex::new(r"[^a-zA-Z0-9]+").unwrap();
  let result = re_non_alpha_num.replace_all(&value.to_lowercase(), " ").to_string();
  // replace all spaces with hyphens and trim
  let re_delimiter: Regex = Regex::new(r"\s+").unwrap();
  re_delimiter.replace_all(result.trim(), "-").to_string()
}