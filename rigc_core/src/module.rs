use std::{pin::Pin, ptr::NonNull};

use crate::functions::Function;
use crate::structures::Structure;

pub struct Module {
	pub(crate) _content: String,
	pub functions: Vec< Box< Function > >,
	pub structures: Vec< Box< Structure > >,

	_pin: std::marker::PhantomPinned,
}

impl Module {
	pub fn new(content: String) -> Pin<Box<Self>> {
		let module = Box::pin(Module {
			_content: content,
			functions: Vec::new(),
			structures: Vec::new(),

			_pin: std::marker::PhantomPinned,
		});

		module
	}

	/// Returns the content of the module as a **`'static`** string slice.
	/// The function assumes that the module is pinned so
	/// the returned slice will always be valid.
	/// Example:
	/// ```rust
	/// let parse_result = rigc_parser::parse(&module.as_ref().content_as_static())
	/// ```
	pub fn content_as_static(self: Pin<&Self>) -> &'static str {
		unsafe { &*(self._content.as_str() as *const str) }
	}

	pub fn register_function(self: Pin<&mut Self>, start_idx: usize, end_idx: usize) -> NonNull<Function> {
		let content = self.as_ref().content_as_static();
		let self_ref = unsafe { self.get_unchecked_mut() };
		// SAFETY: get the mutable reference to self without moving it
		let function = Box::new(Function {
			_name: &content[start_idx..end_idx],
			_return_type: &content[0..0],
		});

		self_ref.functions.push(function);

		unsafe { NonNull::new_unchecked(self_ref.functions.last_mut().unwrap().as_mut()) }
	}

	pub fn register_structure(self: Pin<&mut Self>, start_idx: usize, end_idx: usize) -> NonNull<Structure> {
		let content = self.as_ref().content_as_static();
		let self_ref = unsafe { self.get_unchecked_mut() };
		// SAFETY: get the mutable reference to self without moving it
		let structure = Box::new(Structure {
			_name: &content[start_idx..end_idx],
		});

		self_ref.structures.push(structure);

		unsafe { NonNull::new_unchecked(self_ref.structures.last_mut().unwrap().as_mut()) }
	}
}

