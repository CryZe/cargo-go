use regex::Regex;

pub fn find(key: &str, json: &str) -> Result<Option<String>, String> {
    let pattern = ok!(Regex::new(&format!(r#""{}":"([^"]*)""#, key)));
    match pattern.captures(json) {
        Some(captures) => Ok(Some(captures.at(1).unwrap().into())),
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    const RESPONSE: &'static str = include_str!("../tests/fixtures/cargo.json");

    #[test]
    fn find() {
        assert_eq!(super::find("homepage", RESPONSE), Ok(Some("https://crates.io".into())));
    }
}
