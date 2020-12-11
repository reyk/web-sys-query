wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

use std::convert::TryFrom;
use wasm_bindgen_test::*;
use web_sys::{DomParser, SupportedType};
use web_sys_query::{self as query, query};

const HTML5_DOC: &str = r#"
<!DOCTYPE html>
<html>
<head>
<title>Page Title</title>
</head>
<body>

<h1 id="hero">This is a Heading</h1>
<p>This is a paragraph.</p>
<p>This is another paragraph.</p>

</body>
</html>"#;

fn parse_document(source: &str) -> query::Document {
    console_log::init_with_level(log::Level::Info).ok();

    let parser = DomParser::new().unwrap();
    parser
        .parse_from_string(source, SupportedType::TextHtml)
        .unwrap()
        .into()
}

#[wasm_bindgen_test]
fn test_by_id() {
    let document = parse_document(HTML5_DOC);
    let hero = document.find("#hero").unwrap().first().unwrap();
    console_log!("by_id: {:?}", hero);

    assert_eq!("hero", hero.attr("id").unwrap());
    assert_eq!("hero", hero.id());
}

#[wasm_bindgen_test]
fn test_by_selectors() {
    let document = parse_document(HTML5_DOC);
    let matching = document.find("body p, #hero").unwrap();
    console_log!("by_selectors: {:?}", matching);

    assert_eq!(matching.len(), 3);
}

#[wasm_bindgen_test]
fn test_order() {
    let document = parse_document(HTML5_DOC);
    let matching = document.find("*").unwrap();
    console_log!("order: {:?}", matching);

    let zero = matching.get(0).unwrap();
    assert_eq!(zero.local_name(), "html", "{:?}", matching);
    let five = matching.get(4).unwrap();
    assert_eq!(five.local_name(), "h1", "{:?}", matching);
}

#[wasm_bindgen_test]
fn test_document_root() {
    let document = parse_document(HTML5_DOC);
    let root = query::Element::try_from(&document).unwrap();
    assert_eq!(root.local_name(), "html", "{:?}", document);
}

#[wasm_bindgen_test]
fn test_query_document() {
    let document = parse_document(HTML5_DOC);
    let matching = query!(document, "title").unwrap();
    console_log!("query_document: {:?}", matching);

    let title = matching.get(0).unwrap();
    assert_eq!(title.text().unwrap(), "Page Title");
}

#[wasm_bindgen_test]
fn test_query() {
    let matching = query!("*").unwrap();
    console_log!("query: {:?}", matching);
    assert!(matching.len() > 0);
}
