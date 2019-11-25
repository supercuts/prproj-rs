
/// Is needed despite the Cuts (Vec<Cut>) because
/// sometimes timeline items overlap partially making only
/// one of them visible at a time.
///
/// The Timeline struct allows to preserve the original times
/// of cuts, as well as dynamically *fitting* the timeline with
/// each addition of a Cut.
#[derive(Default, Debug)]
pub struct Timeline {
	/// Timeline
	tm: Vec<TimelineItem>
}

/// Cut is index of the Cut struct with unchanged
/// start and end times as well as a reference to
/// the PremiereMedium.
#[derive(Debug, Default)]
pub struct TimelineItem {
	cut: usize,
	start: f64,
	end: f64,
}

impl Timeline {
	pub(crate) fn add(&mut self, cut: usize, start: f64, end: f64) {
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
			} else if self.tm[middle].start > start {
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
