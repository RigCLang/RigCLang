use std::{pin::Pin, ptr::NonNull};

use crate::functions::function::Function;

pub struct Module {
	pub content: String,
	pub functions: Vec< Box< Function > >,

	_pin: std::marker::PhantomPinned,
}

impl Module {
	pub fn new(content: String) -> Pin<Box<Self>> {
		let module = Box::pin(Module {
			content,
			functions: Vec::new(),

			_pin: std::marker::PhantomPinned,
		});

		module
	}

	pub fn register_function(self: Pin<&mut Self>, start_idx: usize, end_idx: usize) {
		let self_ref = unsafe { self.get_unchecked_mut() };
		// SAFETY: get the mutable reference to self without moving it
		let function = Box::new(Function {
			name: &self_ref.content[start_idx..end_idx]
		});

		self_ref.functions.push(function);
	}
}

