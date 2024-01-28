use rand::{seq::SliceRandom, thread_rng};

// The following has been generated with the help of ChatGPT

pub struct ShuffleIterator<T> {
	items: Vec<T>,
}

impl<T> ShuffleIterator<T> {
	pub fn new<I>(iter: I) -> Self
	where
		I: Iterator<Item = T>,
	{
		let mut items: Vec<T> = iter.collect();
		let mut rng = thread_rng();
		items.shuffle(&mut rng);

		ShuffleIterator { items }
	}
}

impl<T> Iterator for ShuffleIterator<T> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		self.items.pop()
	}
}

pub trait ShuffleIterExt: Iterator + Sized {
	fn shuffle(self) -> ShuffleIterator<Self::Item>;
}

impl<T> ShuffleIterExt for T
where
	T: Iterator,
{
	fn shuffle(self) -> ShuffleIterator<Self::Item> {
		ShuffleIterator::new(self)
	}
}
