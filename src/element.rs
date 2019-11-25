/// https://gitlab.com/xmpp-rs/xmpp-rs/blob/master/minidom-rs/examples/articles.rs
pub use minidom::Element;
use crate::errors::{NotFoundError, NotFoundErrorData};

pub trait ElementGetExt {
	fn get(&self, name: &str) -> Result<&Element, NotFoundError>;
	fn get_attr(&self, name: &str) -> Result<&str, NotFoundError>;
}

impl ElementGetExt for Element {
	fn get(&self, name: &str) -> Result<&Element, NotFoundError> {
		for child in self.children() {
			if child.name() == name {
				return Ok(child);
			}
		}

		Err(
			NotFoundError::Element(
				NotFoundErrorData::new(
					name.to_owned(),
					self.clone()
				)
			)
		)
	}

	fn get_attr(&self, name: &str) -> Result<&str, NotFoundError> {
		if let Some(attr) = self.attr(name) {
			Ok(attr)
		} else {
			Err(
				NotFoundError::Attribute(
					NotFoundErrorData::new(
						name.to_owned(),
						self.clone()
					)
				)
			)
		}
	}
}
