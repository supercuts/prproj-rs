use std::fmt;
use crate::element::Element;

#[derive(Debug)]
pub enum Error {
	NotFound(NotFoundError)
}

#[derive(Debug)]
pub enum NotFoundError {
	Element(NotFoundErrorData),
	Attribute(NotFoundErrorData),
	Multiple(MultipleNotFoundErrorData),
}

#[derive(Debug)]
pub struct MultipleNotFoundErrorData {
	errors: Vec<Error>
}

impl MultipleNotFoundErrorData {
	pub fn new(errors: Vec<Error>) -> Self {
		Self {
			errors
		}
	}
}

#[derive(Debug)]
pub struct NotFoundErrorData {
	name_of_not_found: String,
	in_elem: Element,
}

impl NotFoundErrorData {
	pub fn new(name: String, in_elem: Element) -> Self {
		Self {
			name_of_not_found: name,
			in_elem
		}
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Error::NotFound(not_found_error) => {
				not_found_error.fmt(f)
			}
		}
	}
}

impl fmt::Display for NotFoundError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			NotFoundError::Element(data) =>
				{
					write!(
						f,
						"Element \"{}\" not found in \"{}\"!",
						data.name_of_not_found,
						data.in_elem.name()
					)
				}
			NotFoundError::Attribute(data) => {
				write!(
					f,
					"Attribute \"{}\" not found in \"{}\"!",
					data.name_of_not_found,
					data.in_elem.name()
				)
			}
			NotFoundError::Multiple(data) => {
				writeln!(
					f,
					"Multiple errors occurred ({}):",
					data.errors.len()
				)?;
				assert_ne!(data.errors.len(), 0);
				let mut last_result: fmt::Result = Ok(());
				for err in &data.errors {
					last_result = err.fmt(f);
				}
				last_result
			}
		}
	}
}
