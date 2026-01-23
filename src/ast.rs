use bevy::prelude::Val;

use crate::HtmlUiError;

#[derive(Debug, Clone)]
pub enum HtmlNode {
	Element {
		tag: HtmlTag,
		classes: Vec<String>,
		gap: Val,
		children: Vec<HtmlNode>,
	},
	Text(String),
}

#[derive(Debug, Clone, Copy)]
pub enum HtmlTag {
	Ui,
	VBox,
	HBox,
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
			Self::Label => "label",
			Self::Button => "button",
			Self::Spacer => "spacer",
		}
	}
}
