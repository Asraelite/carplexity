use std::marker::PhantomData;

pub struct Id<T> {
	marker: PhantomData<T>,
	pub value: u64,
}

impl<T> Id<T> {
	pub const fn new(id: u64) -> Self {
		Self {
			marker: PhantomData,
			value: id,
		}
	}
}

impl<T> Clone for Id<T> {
	fn clone(&self) -> Self {
		Self::new(self.value)
	}
}

impl<T> Copy for Id<T> {}

impl<T> PartialEq for Id<T> {
	fn eq(&self, other: &Self) -> bool {
		self.value == other.value
	}
}

impl<T> Eq for Id<T> {}

impl<T> std::hash::Hash for Id<T> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.value.hash(state);
	}
}

impl<T> std::fmt::Display for Id<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.value)
	}
}

impl<T> std::fmt::Debug for Id<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let type_name = std::any::type_name::<T>();
		let final_name = type_name.split("::").last().unwrap_or(type_name);
		write!(f, "Id<{}>({})", final_name, self.value)
	}
}
