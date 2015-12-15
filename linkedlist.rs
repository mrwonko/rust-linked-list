enum List< T > {
	Empty,
	Cons{
		head : T,
		tail : Box< List< T > >
	},
}

struct ListIterator< 'a, T : 'a > {
	cur : &'a List< T >,
}

impl< 'a, T > IntoIterator for &'a List< T > {
	type Item = &'a T;
	type IntoIter = ListIterator< 'a, T >;
	fn into_iter( self ) -> Self::IntoIter {
		ListIterator{ cur : self }
	}
}

impl < 'a, T > Iterator for ListIterator< 'a, T > {
	type Item = &'a T;
	
	fn next( &mut self ) -> Option< &'a T > {
		match self.cur {
			&List::Empty => None,
			&List::Cons{ ref head, ref tail } => {
				self.cur = tail;
				Some( head )
			},
		}
	}
}

fn main() {
	let list = List::Cons{ head : 42, tail : Box::new(
		List::Cons{ head : 1337, tail : Box::new(
			List::Cons{ head : -1, tail : Box::new(
				List::Empty ) } ) } ) };
	for x in &list {
		println!( "{}", x );
	}
}
