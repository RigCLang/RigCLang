pub(crate) struct TextSpan {
	pub(crate) start: usize,
	pub(crate) end: usize,
}

impl TextSpan {
	pub fn new(start: usize, end: usize) -> Self {
		Self { start, end }
	}

	pub fn empty() -> Self {
		Self { start: 0, end: 0 }
	}

	pub fn len(&self) -> usize {
		self.end - self.start
	}

	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}
}