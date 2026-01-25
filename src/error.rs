use thiserror::Error;

#[derive(Debug, Error)]
pub enum HtmlUiError {
	#[error("HTML source is not valid UTF-8")]
	InvalidUtf8,

	#[error("parse error: {0}")]
	ParseError(String),

	#[error("io error: {0}")]
	IoError(std::io::Error),

	#[error("resource not found")]
	ResourceNotFound,

	#[error("asset not found")]
	AssetNotFound,
}
