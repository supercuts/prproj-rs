use std::cell::RefCell;
use std::time::Duration;
use std::hash::{Hash, Hasher};
use super::{Cuts, Size};
use itertools::Itertools;
#[cfg(target_arch = "wasm32")]
use {
	wasm_bindgen::prelude::*,
	wasm_bindgen::JsValue,
	crate::wasm_utils::WasmDuration
};

use crate::element::{Element, ElementGetExt};
use crate::timeline::Timeline;
use crate::errors::Error;
use crate::{PremiereReader, TICKS_PER_SECOND};
use crate::sorted_vec;

pub type PremiereSequences = Vec<RefCell<PremiereSequence>>;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Default, Debug)]
pub struct PremiereSequence {
	pub id: u32,
	name: String, // TODO: include
	duration: Duration, // TODO: include
	pub(crate) track_groups: Vec<String>,
	pub(crate) cuts: Cuts, // TODO: include
	pub(crate) timeline: Timeline, // TODO: include
	pub size: Size,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl PremiereSequence {
	#[wasm_bindgen(getter)]
	pub fn track_groups(&self) -> Box<[JsValue]>{
		self.track_groups.to_vec().iter().map(|s| JsValue::from(s)).collect_vec().into_boxed_slice()
	}
	#[wasm_bindgen(getter)]
	pub fn name(&self) -> JsValue {
		JsValue::from(self.name.to_owned())
	}
}


impl Hash for PremiereSequence {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.id.hash(state);
	}
}

impl PartialEq for PremiereSequence {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Eq for PremiereSequence {}

impl PremiereSequence {
	pub fn new(elem: &Element) -> Result<Self, Error> {
		let mut new_seq = PremiereSequence::default();
		let mut length: f64 = 0.;

		let (id_elem, name_elem, node_elem, track_groups_elem)
			: (&Element, &Element, &Element, &Element)
			= PremiereReader::get_elems_with_names(
			elem,
			&sorted_vec!["ID", "Name", "Node", "TrackGroups"],
		).into_iter().tuples().next().unwrap();

		new_seq.name = name_elem.text();
		let properties = node_elem.get("Properties")?;
		let mut node_count = sorted_vec!["MZ.WorkInPoint", "MZ.WorkOutPoint"].len();
		let mut work_in_point: u64 = 0;
		let mut work_out_point: u64 = 0;
		for child in properties.children() {
			match child.name() {
				"MZ.WorkInPoint" => {
					work_in_point = child.text().parse().expect("No MZ.WorkInPoint");
					node_count -= 1;
				}
				"MZ.WorkOutPoint" => {
					work_out_point = child.text().parse().expect("No MZ.WorkOutPoint");
					node_count -= 1;
				}
				_ => {
					if node_count == 0 {
						length = (work_out_point - work_in_point) as f64 / TICKS_PER_SECOND as f64;
						break;
					}
				}
			}
		}
		for track_group in track_groups_elem.children() {
			assert_eq!(track_group.name(), "TrackGroup");
			for track_group_child in track_group.children() {
				if track_group_child.name() == "Second" {
					let ref_id = track_group_child.attr("ObjectRef").unwrap();
					new_seq.track_groups.push(ref_id.to_owned());
					break;
				}
			}
		}
		new_seq.id = id_elem.text().parse().unwrap();

		new_seq.duration = Duration::from_secs_f64(length);
		Ok(new_seq)
	}
}

trait SequenceErrors {
	fn name(&self) -> &str;
	fn print_has_no(&self, x: &str) {
		println!("<{}> has no <{}> element!", self.name(), x);
	}
}

impl SequenceErrors for PremiereSequence {
	fn name(&self) -> &str {
		&self.name
	}
}
