use bevy::asset::Asset;
use bevy::reflect::TypePath;

use crate::ast::HtmlNode;

#[derive(Clone, Debug, Asset, TypePath)]
pub struct HtmlUiAsset {
	pub source: String,
	pub ast: Vec<HtmlNode>,
}
