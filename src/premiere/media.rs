use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use super::Size;

#[derive(Debug, Default)]
pub struct PremiereMedia {
	media: HashSet<Box<PremiereMedium>>
}

impl PremiereMedia {
	pub(crate) fn insert(&mut self, medium: PremiereMedium) -> Box<PremiereMedium> {
		if self.media.contains(&medium) {
			return match self.media.get(&medium) {
				Some(value) => value.to_owned(),
				None => unreachable!()
			};
		}
		self.media.insert(Box::new(medium.to_owned()));
		self.media.get(&medium).unwrap().to_owned()
	}
}

#[derive(Clone, Debug, Default)]
pub struct PremiereMedium {
	file_name: String,
	file_path: String,
	frame_rate: usize,
	duration: Duration,
	size: Size,
}

impl PremiereMedium {
	pub fn new(
		file_name: String,
		file_path: String,
		frame_rate: usize,
		duration: Duration,
		size: Size,
	) -> Self {
		Self {
			file_name,
			file_path,
			frame_rate,
			duration,
			size
		}
	}
}

impl PartialEq for PremiereMedium {
	fn eq(&self, other: &Self) -> bool {
		self.file_name == other.file_name
	}
}

impl Eq for PremiereMedium {}

impl Hash for PremiereMedium {
	fn hash<H: Hasher>(&self, state: &mut H) { self.file_name.hash(state) }
}
