mod media;
mod reader;
mod sequence;

pub use media::{PremiereMedia, PremiereMedium};
pub use reader::PremiereReader;
pub use sequence::{PremiereSequence, PremiereSequences};


#[derive(Clone, Default, Debug)]
pub struct Size {
	width: u32,
	height: u32,
}

#[derive(Default, Debug)]
pub(crate) struct Cut {
	start: f64,
	end: f64,
	//	media: &'a RefCell<PremiereMedia>,
	medium: Box<PremiereMedium>,
}

#[derive(Clone)]
enum FindWith {
	ID,
	UID,
}

impl FindWith {
	fn value(&self) -> &'static str {
		match self {
			FindWith::ID => "ObjectID",
			FindWith::UID => "ObjectUID"
		}
	}
}

#[derive(Default, Debug)]
pub struct Cuts {
	cuts: Vec<Cut>
}

impl Cuts {
	fn push(&mut self, cut: Cut) -> usize {
		self.cuts.push(cut);
		self.cuts.len()
	}
}
