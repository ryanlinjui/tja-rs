mod directives;
mod parser;
mod types;

pub use directives::*;
pub use parser::*;
pub use types::*;

#[cfg(feature = "python")]
mod python;
#[cfg(feature = "python")]
pub use python::*;

#[cfg(feature = "wasm")]
mod wasm;
#[cfg(feature = "wasm")]
pub use wasm::*;

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_json_snapshot;
    use std::fs;

    #[test]
    fn test_parse_supernova() {
        let content = fs::read_to_string("data/SUPERNOVA.tja").unwrap();
        let mut parser = TJAParser::new();
        parser.parse_str(&content).unwrap();
        let parsed_tja = parser.get_parsed_tja();
        assert_json_snapshot!("supernova", parsed_tja);
    }

    #[test]
    fn test_parse_nijiiro_baton() {
        let content = fs::read_to_string("data/ニジイロバトン.tja").unwrap();
        let mut parser = TJAParser::new();
        parser.parse_str(&content).unwrap();
        let parsed_tja = parser.get_parsed_tja();
        assert_json_snapshot!("nijiiro_baton", parsed_tja);
    }
}
