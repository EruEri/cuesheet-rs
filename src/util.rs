pub(crate) fn cue_format_string_value(s: &str) -> String {
    match s.contains(' ') {
        true => format!("\"{}\"", s),
        false => s.to_string(),
    }
}
