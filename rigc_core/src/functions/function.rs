pub struct Function
{
	pub(crate) _name: *const str,
	pub(crate) _return_type: *const str,
}

impl Function {
	pub fn name(&self) -> &'static str
	{
		unsafe { &*self._name }
	}

	pub fn return_type(&self) -> &'static str
	{
		unsafe { &*self._return_type }
	}

	pub fn set_return_type(&mut self, return_type: &'static str)
	{
		self._return_type = return_type;
	}
}