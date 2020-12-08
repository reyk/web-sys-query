wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

use wasm_bindgen_test::*;
use web_sys::{DomParser, SupportedType};
use web_sys_query as query;

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
    assert_eq!("hero", hero.attr("id").unwrap());
    assert_eq!("hero", hero.id());

    console_log!("by_id: {:?}", hero);
}

#[wasm_bindgen_test]
fn test_by_selectors() {
    let document = parse_document(HTML5_DOC);
    let matching = document.find("body p, #hero").unwrap();
    assert_eq!(matching.len(), 3);

    console_log!("by_selectors: {:?}", matching);
}