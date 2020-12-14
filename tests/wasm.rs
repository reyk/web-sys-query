wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

use std::{collections::HashMap, convert::TryFrom};
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

<form>
  <div><input type="text" name="a" value="1" id="a"></div>
  <div><input type="text" name="b" value="2" id="b"></div>
  <div><input type="hidden" name="c" value="3" id="c"></div>
  <div>
    <textarea name="d" rows="8" cols="40">4</textarea>
  </div>
  <div><select name="e">
    <option value="5" selected="selected">5</option>
    <option value="6">6</option>
    <option value="7">7</option>
  </select></div>
  <div>
    <input type="checkbox" name="f" value="8" id="f">
  </div>
  <div>
    <input type="submit" name="g" value="Submit" id="g">
  </div>
</form>

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

#[wasm_bindgen_test]
fn test_form() {
    let document = parse_document(HTML5_DOC);
    let form = query!(document, "form").unwrap().first().unwrap();
    let kv: HashMap<_, _> = form.serialize_array().unwrap();
    console_log!("form: {:?}", kv);
    assert_eq!(kv.len(), 7);
    assert_eq!(kv.get("a").unwrap(), "1");
}

#[wasm_bindgen_test]
fn test_form_element() {
    let document = parse_document(HTML5_DOC);
    let form = query!(document, "form").unwrap().first().unwrap();
    let kv: query::FormData = form.serialize_array().unwrap();
    console_log!("form_element: {:?}", kv);
    assert_eq!(kv.len(), 7);
}

#[wasm_bindgen_test]
fn test_form_collection() {
    let document = parse_document(HTML5_DOC);
    let form = query!(document, "form").unwrap();
    let single: query::FormData = form.serialize_array().unwrap();
    let collection: query::FormData = form.first().unwrap().serialize_array().unwrap();
    console_log!("form_collection: {:?}", collection);
    assert_eq!(single, collection);
}
