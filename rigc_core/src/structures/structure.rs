pub struct Structure {
	pub(crate) _name: *const str
}

impl Structure {
	pub fn name(&self) -> &'static str {
		unsafe { &*self._name }
	}
}