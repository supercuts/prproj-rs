/// https://gitlab.com/xmpp-rs/xmpp-rs/blob/master/minidom-rs/examples/articles.rs
pub use minidom::Element;
use crate::errors::{NotFoundError, NotFoundErrorData, Error};

pub trait ElementGetExt {
	fn get(&self, name: &str) -> Result<&Element, Error>;
	fn get_attr(&self, name: &str) -> Result<&str, Error>;
}

impl ElementGetExt for Element {
	fn get(&self, name: &str) -> Result<&Element, Error> {
		for child in self.children() {
			if child.name() == name {
				return Ok(child);
			}
		}

		Err(
			Error::NotFound(
				NotFoundError::Element(
					NotFoundErrorData::new(
						name.to_owned(),
						self.clone()
					)
				)
			)
		)
	}

	fn get_attr(&self, name: &str) -> Result<&str, Error> {
		if let Some(attr) = self.attr(name) {
			Ok(attr)
		} else {
			Err(
				Error::NotFound(
					NotFoundError::Attribute(
						NotFoundErrorData::new(
							name.to_owned(),
							self.clone()
						)
					)
				)
			)
		}
	}
}
