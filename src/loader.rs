use bevy::{
	asset::{AssetLoader, AsyncReadExt, LoadContext, io::Reader},
	reflect::TypePath,
};

use crate::{
	asset::HtmlUiAsset, error::HtmlUiError, parser::parse_htmlish, settings::HtmlUiSettings,
};

#[derive(TypePath)]
pub struct HtmlUiLoader;

impl AssetLoader for HtmlUiLoader {
	type Asset = HtmlUiAsset;
	type Settings = HtmlUiSettings;
	type Error = HtmlUiError;

	async fn load(
		&self,
		reader: &mut dyn Reader,
		_settings: &Self::Settings,
		_load_context: &mut LoadContext<'_>,
	) -> Result<Self::Asset, Self::Error> {
		let mut source = String::new();
		reader
			.read_to_string(&mut source)
			.await
			.map_err(HtmlUiError::IoError)?;

		let ast = parse_htmlish(&source)?;

		let asset = HtmlUiAsset { source, ast };

		Ok(asset)
	}

	fn extensions(&self) -> &[&str] {
		&["html"]
	}
}
