enum List< T > {
	Empty,
	Cons{
		head : T,
		tail : Box< List< T > >
	},
}

impl< T > List< T > {
	fn mk_empty() -> List< T > {
		List::Empty
	}
	fn cons( value : T, list : List< T > ) -> List < T > {
		List::Cons{ head : value, tail : Box::new( list ) }
	}
	fn mk_one( value : T ) -> List< T > {
		List::cons( value, List::mk_empty() )
	}
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

// see also: "Entirely Too Many Linked Lists" for alternatives

fn main() {
	let list = List::cons( 42, List::cons( 1337, List::mk_one( -1 ) ) );
	for x in &list {
		println!( "{}", x );
	}
}
