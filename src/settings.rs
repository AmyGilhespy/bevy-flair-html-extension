use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)] // Settings + Default + Serialize + for<'a> Deserialize<'a>
pub struct HtmlUiSettings;
