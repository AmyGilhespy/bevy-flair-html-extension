use bevy::prelude::*;
use bevy_flair::style::StyleSheet;

use crate::HtmlUiAsset;

#[derive(Clone, Debug, Default, Eq, PartialEq, Resource)]
pub struct HtmlCssUiResource {
	pub html: Handle<HtmlUiAsset>,
	pub css: Option<Handle<StyleSheet>>,
}
