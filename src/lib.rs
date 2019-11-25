// TODO: Tests

/// https://docs.rs/libflate/0.1.16/libflate/gzip/index.html
use libflate::gzip::Decoder;

/// https://stackoverflow.com/questions/26611664/what-is-the-r-operator-in-rust
/// https://gitlab.com/xmpp-rs/xmpp-rs/blob/master/minidom-rs/examples/articles.rs
use minidom::Element;
use std::io::Read;
use std::time::Duration;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::fmt;
use itertools::Itertools;
use std::cell::{RefCell, RefMut};

const TICKS_PER_SECOND: usize = 254_016_000_000;

trait ElementGetExt {
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
				NotFoundErrorData {
					in_elem: self.clone(),
					name_of_not_found: name.to_owned(),
				}
			)
		)
	}

	fn get_attr(&self, name: &str) -> Result<&str, NotFoundError> {
		if let Some(attr) = self.attr(name) {
			Ok(attr)
		} else {
			Err(
				NotFoundError::Attribute(
					NotFoundErrorData {
						in_elem: self.clone(),
						name_of_not_found: name.to_owned(),
					}
				)
			)
		}
	}
}

trait SequenceErrors {
	fn name(&self) -> &str;
	fn print_has_no(&self, x: &str) {
		println!("<{}> has no <{}> element!", self.name(), x);
	}
}

#[derive(Default, Debug)]
pub struct PremiereSequence {
	id: u32,
	name: String,
	duration: Duration,
	/// AudioTrackGroup and VideoTrackGroup ObjectRef ids collection
	/// Currently being emptied after creation!
	track_groups: Vec<String>,
	cuts: Cuts,
	timeline: Timeline,
	size: Size,
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

#[derive(Default, Debug)]
pub struct Timeline {
	/// Timeline
	tm: Vec<TimelineItem>
}

#[derive(Debug, Default)]
pub struct TimelineItem {
	cut: usize,
	start: f64,
	end: f64,
}

impl Timeline {
	fn add(&mut self, cut: usize, start: f64, end: f64) {
		let tm_item = TimelineItem {
			cut,
			start,
			end,
		};

		if self.tm.is_empty() {
			return self.tm.push(tm_item);
		}

		let mut left = 0;
		let mut right = self.tm.len() - 1;
		loop {
			let middle = (right + left) / 2;
			if self.tm[middle].start < start {
				left = middle + 1;
			} else if self.tm[middle]. start > start {
				right = if middle == 0 { 0 } else { middle - 1 };
			} else {
				break;
			}

			if left > right || (left == 0 && right == 0 && middle == 0) {
				break;
			}
		}
		self.tm.insert(left, tm_item);
		self.fit(start, end);
	}

	fn fit(&mut self, start: f64, end: f64) {
		let mut i = 0;
		while i != self.tm.len() {
			let mut item = &mut self.tm[i];
			i += 1;
			if start > item.end {
				// Starts after
				continue;
			}
			if start > item.start && start < item.end {
				// Starts in the middle
				item.end = start;
			}
			if end > item.end && end < item.end {
				// Ends in the middle
				item.start = end;
			}
			if start < item.start && end > item.end {
				// Covers whole
				self.tm.remove(i);
				i -= 1;
			}
		};
	}
}

#[derive(Clone, Default, Debug)]
struct Size {
	width: u32,
	height: u32,
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

#[derive(Debug)]
pub enum NotFoundError {
	Element(NotFoundErrorData),
	Attribute(NotFoundErrorData),
	Multiple(MultipleNotFoundErrorData),
}

#[derive(Debug)]
pub struct MultipleNotFoundErrorData {
	errors: Vec<NotFoundError>
}

#[derive(Debug)]
pub struct NotFoundErrorData {
	name_of_not_found: String,
	in_elem: Element,
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

impl Eq for PremiereSequence {}

impl SequenceErrors for PremiereSequence {
	fn name(&self) -> &str {
		&self.name
	}
}

#[derive(Default, Debug)]
struct Cut {
	start: f64,
	end: f64,
	//	media: &'a RefCell<PremiereMedia>,
	medium: Box<PremiereMedium>,
}

impl PremiereSequence {
	pub fn new(elem: &Element) -> Result<Self, NotFoundError> {
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
		let mut work_in_point: usize = 0;
		let mut work_out_point: usize = 0;
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
			assert!(track_group.name() == "TrackGroup");
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

#[macro_export]
macro_rules! sorted_vec {
    ($($x:expr),*) => {
		{
			let mut temp_vec = Vec::new();
			$(
				temp_vec.push($x);
			)*
			temp_vec.sort();
			temp_vec
		}
    };
    ($($x:expr,)*) => ($crate::sorted_vec![$($x),*])
}

#[derive(Debug)]
/// Reads data and produces sequences
pub struct PremiereReader {
	media: RefCell<PremiereMedia>,
	sequences: PremiereSequences,
	root: Element,
}

type PremiereSequences = Vec<RefCell<PremiereSequence>>;

#[derive(Debug, Default)]
pub struct PremiereMedia {
	media: HashSet<Box<PremiereMedium>>
}

impl PremiereMedia {
	fn insert(&mut self, medium: PremiereMedium) -> Box<PremiereMedium> {
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

impl PartialEq for PremiereMedium {
	fn eq(&self, other: &Self) -> bool {
		self.file_name == other.file_name
	}
}

impl Eq for PremiereMedium {}

impl Hash for PremiereMedium {
	fn hash<H: Hasher>(&self, state: &mut H) { self.file_name.hash(state) }
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

type HashMapWithVector = HashMap<usize, Vec<String>>;

trait HashMapLengthWithVector {
	fn length(&self) -> usize;
}

impl HashMapLengthWithVector for HashMapWithVector {
	fn length(&self) -> usize {
		self.values()
			.fold(
				0 as usize,
				|acc: usize, entry: &Vec<String>| acc + entry.len(),
			)
	}
}

impl PremiereReader {
	pub fn media(&self) -> &RefCell<PremiereMedia> { &self.media }
	pub fn sequences(&self) -> &Vec<RefCell<PremiereSequence>> {
		&self.sequences
	}
	pub fn new(xml: &[u8]) -> Self {
		let to_str = |bytes: &[u8]|
			String::from_utf8(bytes.to_vec()).unwrap();

		if xml[0..2].to_vec() == [0x1f, 0x8b] {
			let mut buf = Vec::new();
			// https://docs.rs/libflate/0.1.27/libflate/gzip/struct.Decoder.html
			let mut decoder = Decoder::new(xml).unwrap();
			decoder.read_to_end(&mut buf).unwrap();
			Self {
				root: to_str(&buf).parse().expect("XML parsing unzipped error"),
				sequences: Vec::default(),
				media: RefCell::new(PremiereMedia::default()),
			}
		} else {
			Self {
				root: to_str(xml).parse().expect("XML parsing raw XML errror"),
				sequences: Vec::default(),
				media: RefCell::new(PremiereMedia::default()),
			}
		}
	}

	fn get_sequences(root: &Element, sequences: &mut PremiereSequences)
	                 -> Result<HashMap<usize, Vec<String>>, NotFoundError>
	{
		let mut references: HashMap<usize, Vec<String>> = HashMap::new();
		for child in root.children() {
			if child.name() == "Sequence" {
				println!("<Sequence ObjectUID={:?}>", child.attr("ObjectUID").unwrap());
				let seq = PremiereSequence::new(child)?;
				let seq_index = sequences.len();
				references.insert(seq_index, seq.track_groups.to_owned());
				sequences.push(RefCell::new(seq));
			}
		}
		Ok(references)
	}

	pub fn read(&mut self) -> Result<(), NotFoundError> {
		let references = Self::get_sequences(&self.root, &mut self.sequences)?;
		// Non-lexical lifetime in Vec!
		// https://rust-lang.github.io/rfcs/2094-nll.html#problem-case-3-conditional-control-flow-across-functions
		// https://github.com/rust-lang/rust/issues/21906#issuecomment-73296543
		// https://stackoverflow.com/questions/58295535/cannot-borrow-self-as-mutable-more-than-once-at-a-time-when-returning-a-resul
		// https://stackoverflow.com/questions/38023871/returning-a-reference-from-a-hashmap-or-vec-causes-a-borrow-to-last-beyond-the-s
		self.resolve_groups(&references)
	}

	fn parse_video_track_group(&self, vtg: &minidom::element::Element, mut seq: RefMut<PremiereSequence>) -> Result<(), NotFoundError> {
		let (frame_rect_elem, track_group_elem) =
			Self::get_elems_with_names(
				vtg,
				&sorted_vec!["FrameRect", "TrackGroup"],
			).into_iter().tuples().next().unwrap();

// <FrameRect>0,0,1920,1080</FrameRect>
		let frame_rect_text =
			frame_rect_elem.text();

		let mut frame_rect =
			frame_rect_text.split(',').skip(2);

		seq.size.width = frame_rect.next().unwrap().parse().unwrap();
		seq.size.height = frame_rect.next().unwrap().parse().unwrap();
		let mut track_refs: Vec<&str> = Vec::new();
		let mut track_refs_found_with: &FindWith = &FindWith::ID;

		for track in track_group_elem.get("Tracks")?.children() {
			if let Ok(object_u_ref)
			= track.get_attr("ObjectURef") {
				track_refs.push(object_u_ref);
				track_refs_found_with = &FindWith::UID;
			} else {
				track_refs.push(track.get_attr("ObjectRef").unwrap());
				track_refs_found_with = &FindWith::ID;
			}
		}
		for track_ref in track_refs {
			let track_items: &Element
// VideoClipTrack or AudioClipTrack so can't set name
				= self.get_elem_with_id(track_ref, track_refs_found_with.clone())?
				.get("ClipTrack")?
				.get("ClipItems")?
				.get("TrackItems")?;

			let mut track_item_refs: Vec<&str> = Vec::new();
			for track_item in track_items.children() {
				track_item_refs.push(track_item.get_attr("ObjectRef")?);
			}

			for clip_track_item in self.get_elems_with_ids(
				&track_item_refs,
				FindWith::ID,
			) {
				let sub_clip_track_item
					= clip_track_item.get("ClipTrackItem")?;

				let (sub_clip_elem, track_item_elem) = Self::get_elems_with_names(
					sub_clip_track_item,
					&sorted_vec!["SubClip", "TrackItem"],
				).into_iter().tuples().next().unwrap();

				let (end_elem, start_elem) = Self::get_elems_with_names(
					track_item_elem,
					&sorted_vec!["End", "Start"], //TODO: create sorted vector macro
				).into_iter().tuples().next().unwrap();

				let sub_clip = self.get_elem_with_id(
					sub_clip_elem.get_attr("ObjectRef")?,
					FindWith::ID,
				)?;

				let media_name: String;
				let media_path: String;
				let in_point: usize = start_elem.text().parse().unwrap();
				let out_point: usize = end_elem.text().parse().unwrap();
				let frame_rate: usize;
				let duration: usize;

				let (clip_elem, _master_clip_elem, _name_elem) =
					Self::get_elems_with_names(
						sub_clip,
						&sorted_vec!["Clip", "MasterClip", "Name"],
					).into_iter().tuples().next().unwrap();

				let clip
					= self.get_elem_with_id(
					clip_elem.get_attr("ObjectRef")?,
					FindWith::ID,
				)?.get("Clip")?;

				let source_elem
					= clip.get("Source")?;


				let media_uref =
					self.get_elem_with_id(
						source_elem.get_attr("ObjectRef")?,
						FindWith::ID,
					)?
						.get("MediaSource")?
						.get("Media")?
						.get_attr("ObjectURef")?;

				let media = self.get_elem_with_id(media_uref, FindWith::UID)?;
				let (file_path_elem, title_elem, video_stream_elem): (&Element, &Element, &Element)
					= Self::get_elems_with_names(
					media,
					&sorted_vec!["FilePath", "Title", "VideoStream"])
					.into_iter().tuples().next().unwrap();

				media_path = {
					let file_path_elem_text = file_path_elem.text();
					file_path_elem_text.trim().to_owned()
				};

				media_name = {
					let title_elem_text = title_elem.text();
					title_elem_text.trim().to_owned()
				};


				let video_stream
					= self.get_elem_with_id(
					video_stream_elem.get_attr("ObjectRef")?,
					FindWith::ID,
				)?;

				{
					let (duration_elem, frame_rate_elem): (&Element, &Element)
						= Self::get_elems_with_names(
						video_stream,
						&sorted_vec!["Duration", "FrameRate"],
					).into_iter().tuples().next().unwrap();

					frame_rate = frame_rate_elem.text().parse().unwrap_or(0);
					duration = duration_elem.text().parse().unwrap_or(0);
				};

				let medium = PremiereMedium {
					duration: Duration::from_secs_f64(
						duration as f64 / TICKS_PER_SECOND as f64
					),
					file_name: media_name.to_owned(),
					file_path: media_path.to_owned(),
					size: Size {
						width: 1920,
						height: 1080,
					},
					frame_rate,
				};
				let mut media = self.media.borrow_mut();

				let medium_ref
					= media.insert(medium);
				let start = in_point as f64 / TICKS_PER_SECOND as f64;
				let end = out_point as f64 / TICKS_PER_SECOND as f64;

				let cut = Cut {
					start,
					end,
					medium: medium_ref,
				};

				let seq_index = seq.cuts.push(cut);
				seq.timeline.add(seq_index, start, end);
			}
		}
		Ok(())
	}


	fn resolve_groups(&mut self, id_refs: &HashMapWithVector) -> Result<(), NotFoundError> {
		let mut groups_to_find: usize = id_refs.length();
		let mut errors: Vec<NotFoundError> = Vec::new();
		for child in self.root.children() {
			match child.name() {
				// Only videos for now, since audio is useless for my use-case
				"VideoTrackGroup" => {
					if let Some(object_id) = child.attr("ObjectID") {
						groups_to_find -= 1;
						for seq_index in id_refs.keys() {
							if let Some(refs) = id_refs.get(seq_index) {
								for id_ref in refs {
									if object_id == id_ref {
										let seq =
											self.sequences[seq_index.to_owned()].borrow_mut();
										if let Err(err) = self.parse_video_track_group(&child, seq) {
											errors.push(err);
										}
									}
								}
							}
						}
					} else {
						errors.push(
							NotFoundError::Attribute(
								NotFoundErrorData {
									in_elem: child.to_owned(),
									name_of_not_found: String::from("ObjectID"),
								}
							)
						)
					}
				}
				_ => {
					if groups_to_find == 0 {
						break;
					}
				}
			}
		}
		assert_ne!(id_refs.length(), groups_to_find);

		let all_errors = errors.len() == id_refs.length();
		// It's ok as long as not all failed.
		if !all_errors {
			let errors_len = errors.len();
			if errors_len > 0 {
				println!("Ignoring {} errors in resolving video track groups:", errors_len);
				for err in errors {
					println!("{:#?}", err);
				}
			}
			Ok(())
		} else {
			Err(
				NotFoundError::Multiple(
					MultipleNotFoundErrorData {
						errors
					}
				)
			)
		}
	}

	fn get_elem_with_id(&self, identifier: &str, find_with: FindWith) -> Result<&Element, NotFoundError> {
		if let Some(elem) = self.try_get_elem_with_id(identifier, find_with) {
			Ok(elem)
		} else {
			Err(
				NotFoundError::Element(
					NotFoundErrorData {
						name_of_not_found: format!("With id: \"{}\"", identifier),
						in_elem: self.root.clone(),
					}
				)
			)
		}
	}

	fn get_elems_with_ids<'a>(&'a self, identifiers: &'a [&'a str], find_with: FindWith)
	                          -> impl Iterator<Item=&Element> + 'a
	{
		self.root.children().filter(move |elem: &&Element| {
			if let Some(attr) = elem.attr(find_with.value()) {
				identifiers.contains(&attr)
			} else {
				false
			}
		})
	}

	fn get_elems_with_names<'a>(in_elem: &'a Element, names: &[&str])
	                            -> Vec<&'a Element>
	{
		let mut elems_to_find = names.len();
		let mut vec: Vec<&Element> = vec![in_elem; elems_to_find]; // fill vector with dummy references
		for child in in_elem.children() {
			if let Ok(index) = names.binary_search(&child.name()) {
				vec[index] = &child;
				elems_to_find -= 1;
				if elems_to_find == 0 {
					return vec;
				}
			}
		}
		// FIXME: this below thing
		println!("Some elements were not found!\nFound: {:#?}\nSearched: {:#?}", vec, names);
		vec
	}

	fn try_get_elem_with_id(&self, identifier: &str, find_with: FindWith) -> Option<&Element> {
		for child in self.root.children() {
			if let Some(attr) = child.attr(find_with.value()) {
				if attr == identifier {
					return Some(child);
				}
			}
		}
		None
	}

//	fn get_elem_with_name_and_id(&self, elem_name: &'static str, identifier: &str, find_with: FindWith) -> Result<&Element, NotFoundError> {
//		for child in self.root.children() {
//			if child.name() == elem_name {
//				if child.get_attr(find_with.value())? != identifier {
//					continue;
//				}
//				return Ok(child);
//			}
//		}
//		Err(
//			NotFoundError::Element(
//				NotFoundErrorData {
//					in_elem: self.root.clone(),
//					name_of_not_found: elem_name.to_owned(),
//				}
//			)
//		)
//	}
}



