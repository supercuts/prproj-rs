use std::cell::{RefCell, RefMut};
use crate::element::{Element, ElementGetExt};

/// https://docs.rs/libflate/0.1.16/libflate/gzip/index.html
use libflate::gzip::Decoder;
use std::io::Read;

use std::collections::HashMap;
use crate::errors::{NotFoundError, NotFoundErrorData, MultipleNotFoundErrorData};
use itertools::Itertools;

use super::{PremiereMedia, PremiereSequence, PremiereSequences, Size, Cut, FindWith};
use crate::{sorted_vec, TICKS_PER_SECOND};
use crate::premiere::PremiereMedium;
use std::time::Duration;
use std::path::Path;
use std::fs::File;

#[derive(Debug)]
/// Reads data and produces sequences, media.
/// ```
/// use std::path::Path;
/// use prproj::PremiereReader;
///
/// let mut reader = PremiereReader::from_path(&Path::new(XML_FILE));
///	if let Err(err) = reader.read() {
///        println!("Error reading Premiere Pro file: {}\n{:#?}!", err, err)
/// }
/// println!("Sequences: {:#?}", reader.sequences());
/// println!("Media: {:#?}", reader.media())
/// ```
pub struct PremiereReader {
	media: RefCell<PremiereMedia>,
	sequences: PremiereSequences,
	root: Element,
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

	pub fn from_path(path: &Path) -> Self {
		let mut buffer: Vec<u8> = Vec::new();
		let mut file = File::open(Path::new(path)).unwrap();
		file.read_to_end(&mut buffer).unwrap();
		PremiereReader::new(&buffer)
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

				let medium = PremiereMedium::new(
					media_name.to_owned(),
					media_path.to_owned(),
					frame_rate,
					Duration::from_secs_f64(
						duration as f64 / TICKS_PER_SECOND as f64
					),
					Size {
						width: 1920,
						height: 1080,
					}
				);
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
								NotFoundErrorData::new(
									String::from("ObjectID"),
									child.to_owned(),
								)
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
					MultipleNotFoundErrorData::new(
						errors
					)
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
					NotFoundErrorData::new(
						format!("With id: \"{}\"", identifier),
						self.root.clone(),
					)
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

	pub(crate) fn get_elems_with_names<'a>(in_elem: &'a Element, names: &[&str])
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
}
