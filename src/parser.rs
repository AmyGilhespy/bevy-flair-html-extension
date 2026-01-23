use bevy::prelude::Val;

use crate::{
	ast::{HtmlNode, HtmlTag},
	error::HtmlUiError,
};

pub fn parse_htmlish(source: &String) -> Result<Vec<HtmlNode>, HtmlUiError> {
	let mut nodes = Vec::new();
	let mut stack: Vec<(HtmlTag, Vec<String>, Val, Vec<HtmlNode>)> = Vec::new();

	let mut i = 0;
	let bytes = source.as_bytes();

	while i < bytes.len() {
		if bytes[i] == b'<' {
			// Closing tag
			if bytes.get(i + 1) == Some(&b'/') {
				let end = find_byte(bytes, b'>', i)?;
				let tag_name = &source[i + 2..end].trim();

				let (tag, classes, gap, children) = stack
					.pop()
					.ok_or_else(|| HtmlUiError::ParseError("unmatched closing tag".into()))?;

				if tag.as_str() != *tag_name {
					return Err(HtmlUiError::ParseError(format!(
						"expected </{}> but found </{}>",
						tag.as_str(),
						tag_name
					)));
				}

				let node = HtmlNode::Element {
					tag,
					classes,
					gap,
					children,
				};

				if let Some((_, _, _, parent_children)) = stack.last_mut() {
					parent_children.push(node);
				} else {
					nodes.push(node);
				}

				i = end + 1;
			} else {
				// Opening tag
				let end = find_byte(bytes, b'>', i)?;
				let tag_src = &source[i + 1..end];
				let (tag, classes, gap) = parse_tag(tag_src)?;

				stack.push((tag, classes, gap, Vec::new()));
				i = end + 1;
			}
		} else {
			// Text node
			let end = find_byte(bytes, b'<', i).unwrap_or(bytes.len());
			let text = source[i..end].trim();

			if !text.is_empty() {
				let text_node = HtmlNode::Text(text.to_string());
				if let Some((_, _, _, children)) = stack.last_mut() {
					children.push(text_node);
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

fn parse_tag(src: &str) -> Result<(HtmlTag, Vec<String>, Val), HtmlUiError> {
	let parts = split_quoted_whitespace(src);
	let mut parts = parts.into_iter();

	let tag_name = parts
		.next()
		.ok_or_else(|| HtmlUiError::ParseError("empty tag".into()))?;

	let tag = HtmlTag::from_str(tag_name)?;

	let mut classes = Vec::new();
	let mut gap: Val = Val::Auto;

	for part in parts {
		if let Some(rest) = part.strip_prefix("class=\"") {
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

	Ok((tag, classes, gap))
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
