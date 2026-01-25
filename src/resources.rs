use std::collections::HashMap;

use bevy::prelude::*;
use bevy_flair::style::StyleSheet;

use crate::{HtmlCallback, HtmlUiAsset};

#[derive(Default, Resource)]
pub struct HtmlCssUiResource {
	pub html: Handle<HtmlUiAsset>,
	pub css: Option<Handle<StyleSheet>>,
	pub callbacks: HashMap<String, Box<HtmlCallback>>,
}

impl HtmlCssUiResource {
	#[must_use]
	pub fn new(html: Handle<HtmlUiAsset>, css: Option<Handle<StyleSheet>>) -> Self {
		Self {
			html,
			css,
			callbacks: HashMap::new(),
		}
	}

	#[must_use]
	pub fn with_callback<F>(mut self, name: impl Into<String>, f: F) -> Self
	where
		F: Fn(&mut World, Entity) + Send + Sync + 'static,
	{
		self.callbacks.insert(name.into(), Box::new(f));
		self
	}
}
