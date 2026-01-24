use bevy::prelude::Val;

use crate::{
	ast::{HtmlElement, HtmlNode, HtmlTag},
	error::HtmlUiError,
};

#[allow(clippy::type_complexity)]
pub fn parse_htmlish(source: &String) -> Result<Vec<HtmlNode>, HtmlUiError> {
	let mut nodes = Vec::new();
	let mut stack: Vec<HtmlElement> = Vec::new();

	let mut i = 0;
	let bytes = source.as_bytes();

	while i < bytes.len() {
		if bytes[i] == b'<' {
			// Closing tag
			if bytes.get(i + 1) == Some(&b'/') {
				let end = find_byte(bytes, b'>', i)?;
				let tag_name = &source[i + 2..end].trim();

				let HtmlElement {
					tag,
					name_id,
					classes,
					gap,
					children,
				} = stack
					.pop()
					.ok_or_else(|| HtmlUiError::ParseError("unmatched closing tag".into()))?;

				if tag.as_str() != *tag_name {
					return Err(HtmlUiError::ParseError(format!(
						"expected </{}> but found </{}>",
						tag.as_str(),
						tag_name
					)));
				}

				let node = HtmlNode::Element(HtmlElement {
					tag,
					name_id,
					classes,
					gap,
					children,
				});

				if let Some(parent) = stack.last_mut() {
					parent.children.push(node);
				} else {
					nodes.push(node);
				}

				i = end + 1;
			} else {
				// Opening tag
				let end = find_byte(bytes, b'>', i)?;
				let tag_src = &source[i + 1..end];
				let element = parse_tag(tag_src)?;

				stack.push(element);
				i = end + 1;
			}
		} else {
			// Text node
			let end = find_byte(bytes, b'<', i).unwrap_or(bytes.len());
			let text = source[i..end].trim();

			if !text.is_empty() {
				let text_node = HtmlNode::Text(text.to_string());
				if let Some(elem) = stack.last_mut() {
					elem.children.push(text_node);
				} else {
					nodes.push(text_node);
				}
			}

			i = end;
		}
	}

	if !stack.is_empty() {
		return Err(HtmlUiError::ParseError("unclosed tag".into()));
	}

	Ok(nodes)
}

#[cfg(test)]
mod parse_htmlish_tests {
	use bevy::ui::Val;

	use crate::ast::HtmlElement;

	use super::super::ast::{HtmlNode, HtmlTag};
	use super::parse_htmlish;

	const GOOD_HTML: &str = r#"
		<ui class="a">
			<button id="my-button" class="b c">
				<vbox class="d" gap="12.25px">
					<spacer></spacer>
					<label id="my-label">Hello, world</label>
					<spacer></spacer>
				</vbox>
			</button>
		</ui>
		"#;

	#[test]
	fn test() {
		let good_parsed = parse_htmlish(&GOOD_HTML.into());
		assert!(good_parsed.is_ok(), "Good HTML returned an error.");
		let good = good_parsed.unwrap(); // Vec<HtmlNode>
		assert_eq!(good.len(), 1, "Wrong number of HTML <ui> nodes.");
		let HtmlNode::Element(HtmlElement {
			tag,
			name_id,
			classes,
			gap,
			children,
		}) = &good[0]
		else {
			panic!("<ui> is not Element");
		};
		assert_eq!(*tag, HtmlTag::Ui, "<ui> tag was not <ui>.");
		assert!(name_id.is_none(), "<ui> had non-existent id field.");
		assert_eq!(*classes, vec!["a".to_owned()]);
		assert_eq!(*gap, Val::Auto);
		assert_eq!(children.len(), 1, "Wrong number of HTML <button> nodes.");
		let HtmlNode::Element(HtmlElement {
			tag,
			name_id,
			classes,
			gap,
			children,
		}) = &children[0]
		else {
			panic!("<button> is not Element");
		};
		assert_eq!(*tag, HtmlTag::Button, "<button> tag was not <button>.");
		assert!(
			*name_id == Some("my-button".to_owned()),
			"<button> lacked id field."
		);
		assert_eq!(*classes, vec!["b".to_owned(), "c".to_owned()]);
		assert_eq!(*gap, Val::Auto);
		assert_eq!(children.len(), 1, "Wrong number of HTML <vbox> nodes.");
		let HtmlNode::Element(HtmlElement {
			tag,
			name_id,
			classes,
			gap,
			children,
		}) = &children[0]
		else {
			panic!("<vbox> is not Element");
		};
		assert_eq!(*tag, HtmlTag::VBox, "<vbox> tag was not <vbox>.");
		assert!(name_id.is_none(), "<vbox> had non-existent id field.");
		assert_eq!(*classes, vec!["d".to_owned()]);
		assert_eq!(*gap, Val::Px(12.25));
		assert_eq!(children.len(), 3, "Wrong number of HTML <vbox> children.");
		let child0 = &children[0];
		let child1 = &children[1];
		let child2 = &children[2];
		let HtmlNode::Element(HtmlElement {
			tag,
			name_id,
			classes,
			gap,
			children,
		}) = child0
		else {
			panic!("<spacer> #0 is not Element");
		};
		assert_eq!(*tag, HtmlTag::Spacer, "<spacer> tag was not <spacer>.");
		assert!(name_id.is_none(), "<spacer> had non-existent id field.");
		assert_eq!(classes.len(), 0, "Wrong number of HTML <spacer> classes.");
		assert_eq!(*gap, Val::Auto);
		assert_eq!(children.len(), 0, "Wrong number of HTML <spacer> children.");
		let HtmlNode::Element(HtmlElement {
			tag,
			name_id,
			classes,
			gap,
			children,
		}) = child1
		else {
			panic!("<label> #1 is not Element");
		};
		assert_eq!(*tag, HtmlTag::Label, "<label> tag was not <label>.");
		assert!(
			*name_id == Some("my-label".to_owned()),
			"<label> lacked id field."
		);
		assert_eq!(classes.len(), 0, "Wrong number of HTML <label> classes.");
		assert_eq!(*gap, Val::Auto);
		assert_eq!(children.len(), 1, "Wrong number of HTML <label> children.");
		let HtmlNode::Text(text) = &children[0] else {
			panic!("<label> text is not Text");
		};
		assert_eq!(text, "Hello, world");
		let HtmlNode::Element(HtmlElement {
			tag,
			name_id,
			classes,
			gap,
			children,
		}) = child2
		else {
			panic!("<spacer> #2 is not Element");
		};
		assert_eq!(*tag, HtmlTag::Spacer, "<spacer> tag was not <spacer>.");
		assert!(name_id.is_none(), "<spacer> had non-existent id field.");
		assert_eq!(classes.len(), 0, "Wrong number of HTML <spacer> classes.");
		assert_eq!(*gap, Val::Auto);
		assert_eq!(children.len(), 0, "Wrong number of HTML <spacer> children.");
	}
}

fn parse_tag(src: &str) -> Result<HtmlElement, HtmlUiError> {
	let parts = split_quoted_whitespace(src);
	let mut parts = parts.into_iter();

	let tag_name = parts
		.next()
		.ok_or_else(|| HtmlUiError::ParseError("empty tag".into()))?;

	let tag = HtmlTag::from_str(tag_name)?;

	let mut name_id = None;
	let mut classes = Vec::new();
	let mut gap: Val = Val::Auto;

	for part in parts {
		if let Some(rest) = part.strip_prefix("id=\"") {
			name_id = Some(rest.trim_end_matches('"').into());
		} else if let Some(rest) = part.strip_prefix("class=\"") {
			let value = rest.trim_end_matches('"');
			classes.extend(
				value
					.split_whitespace()
					.map(std::string::ToString::to_string),
			);
		} else if let Some(rest) = part.strip_prefix("gap=\"") {
			gap = parse_val(rest.trim_end_matches('"'))?;
		}
	}

	Ok(HtmlElement {
		tag,
		name_id,
		classes,
		gap,
		children: Vec::new(),
	})
}

fn split_quoted_whitespace(s: &str) -> Vec<&str> {
	let mut parts = Vec::new();
	let mut start = 0;
	let mut in_quotes = false;

	for (i, c) in s.char_indices() {
		match c {
			'"' => in_quotes = !in_quotes,
			c if c.is_whitespace() && !in_quotes => {
				if start < i {
					parts.push(&s[start..i]);
				}
				start = i + c.len_utf8();
			}
			_ => {}
		}
	}

	if start < s.len() {
		parts.push(&s[start..]);
	}

	parts
}

fn parse_val(string: &str) -> Result<Val, HtmlUiError> {
	if let Some(pc) = string.strip_suffix("%") {
		Ok(Val::Percent(pc.parse::<f32>().map_err(|err| {
			HtmlUiError::ParseError(format!("invalid gap tag: {err}"))
		})?))
	} else if let Some(px) = string.strip_suffix("px") {
		Ok(Val::Px(px.parse::<f32>().map_err(|err| {
			HtmlUiError::ParseError(format!("invalid gap tag: {err}"))
		})?))
	} else if let Some(vmax) = string.strip_suffix("vmax") {
		Ok(Val::VMax(vmax.parse::<f32>().map_err(|err| {
			HtmlUiError::ParseError(format!("invalid gap tag: {err}"))
		})?))
	} else if let Some(vmin) = string.strip_suffix("vmin") {
		Ok(Val::VMin(vmin.parse::<f32>().map_err(|err| {
			HtmlUiError::ParseError(format!("invalid gap tag: {err}"))
		})?))
	} else if let Some(vw) = string.strip_suffix("vw") {
		Ok(Val::Vw(vw.parse::<f32>().map_err(|err| {
			HtmlUiError::ParseError(format!("invalid gap tag: {err}"))
		})?))
	} else if let Some(vh) = string.strip_suffix("vh") {
		Ok(Val::Vh(vh.parse::<f32>().map_err(|err| {
			HtmlUiError::ParseError(format!("invalid gap tag: {err}"))
		})?))
	} else if string == "auto" {
		Ok(Val::Auto)
	} else {
		Ok(Val::Px(string.parse::<f32>().map_err(|err| {
			HtmlUiError::ParseError(format!("invalid gap tag: {err}"))
		})?))
	}
}

fn find_byte(bytes: &[u8], needle: u8, start: usize) -> Result<usize, HtmlUiError> {
	bytes[start..]
		.iter()
		.position(|&b| b == needle)
		.map(|p| start + p)
		.ok_or_else(|| HtmlUiError::ParseError("unexpected end of input".into()))
}
