use kuchikiki::traits::*;
use kuchikiki::parse_html;

use crate::enums;
use enums::TransformType;

use std::error::Error;

pub struct HtmlChanger;

impl HtmlChanger {
    pub fn transform(transform_type: TransformType, html: String) -> Result<String, Box<dyn Error>> {
      let wrapped_html = format!("<div id=\"__wrapper__\">{}</div>", html);
      let document = parse_html().one(wrapped_html);

      let wrapper = document
          .select_first("div#__wrapper__")
          .map_err(|_| "Failed to select wrapper div")?;

      let p_tags = wrapper
          .as_node()
          .select("p")
          .map_err(|_| "Failed to select <p> tags")?;

      for css_match in p_tags {
          let node = css_match.as_node();
          for descendant in node.descendants() {
              if let Some(text_node) = descendant.as_text() {
                  let original = text_node.borrow().clone();
                  let transformed = transform_type.transform_string(&original);
                  *text_node.borrow_mut() = transformed;
              }
          }
      }

      let mut result = Vec::new();
      for child in wrapper.as_node().children() {
          child
              .serialize(&mut result)
              .map_err(|_| "Unable to serialization wrapper")?;
      }

      let output = String::from_utf8(result)
        .map_err(|_| "Unable to convert result to UTF-8")?;

      Ok(output)
    }
} 