use bevy::prelude::Val;

use crate::HtmlUiError;

#[derive(Debug, Clone)]
pub enum HtmlNode {
	Element(HtmlElement),
	Text(String),
}

#[derive(Debug, Clone)]
pub struct HtmlElement {
	pub tag: HtmlTag,
	pub name_id: Option<String>,
	pub classes: Vec<String>,
	pub gap: Val,
	pub autofocus: bool,
	pub children: Vec<HtmlNode>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum HtmlTag {
	Ui,
	VBox,
	HBox,
	Node,
	Label,
	Button,
	Spacer,
}

impl HtmlTag {
	pub fn from_str(s: &str) -> Result<Self, HtmlUiError> {
		match s {
			"ui" => Ok(Self::Ui),
			"vbox" => Ok(Self::VBox),
			"hbox" => Ok(Self::HBox),
			"node" => Ok(Self::Node),
			"label" => Ok(Self::Label),
			"button" => Ok(Self::Button),
			"spacer" => Ok(Self::Spacer),
			_ => Err(HtmlUiError::ParseError(format!("unknown tag `{s}`"))),
		}
	}

	pub fn as_str(self) -> &'static str {
		match self {
			Self::Ui => "ui",
			Self::VBox => "vbox",
			Self::HBox => "hbox",
			Self::Node => "node",
			Self::Label => "label",
			Self::Button => "button",
			Self::Spacer => "spacer",
		}
	}
}
