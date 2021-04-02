pub fn binary_search<T: PartialOrd>(list: &[T], el: T) -> Option<usize> {
	assert!(list.is_sorted());

	let mut left: usize = 0;
	let mut right: usize = list.len() - 1;

	while left != right {
		let pivot: usize = (left + right) / 2  as usize;

		match &list[pivot] {
			n if n < &el => { left = pivot + 1; }
			n if n > &el => { right = pivot - 1; }
			_ => { return Some(pivot); }
		}
	}

	return None;
}

#[cfg(test)]
mod tests {
	use super::{binary_search};

	macro_rules! binary_search_tests {
		($($name:ident: $value:expr,)*) => {
			$(
				#[test]
				fn $name() {
					let (list, searched_element, expected_index) = $value;
					assert_eq!(binary_search(list, searched_element), expected_index)
				}
			)*
		}
	}

	binary_search_tests! {
		simple_list_present: (
			&[1, 2, 3, 4],
			3,
			Some(2)
		),
		simple_list_not_present: (
			&[1, 2, 3, 4],
			0,
			None
		),
		complex_list_not_present: (
			&[1, 32, 49, 291, 312, 333, 1293, 2999, 32184],
			292,
			None
		),
		complex_list_present_present: (
			&[1, 32, 49, 291, 312, 333, 1293, 2999, 32184],
			1293,
			Some(6)
		),
		repeated_element_even: (
			&[1, 1, 1, 1, 1, 1],
			1,
			Some(2)
		),
		repeated_element_odd: (
			&[1, 1, 1, 1, 1, 1, 1],
			1,
			Some(3)
		),
	}

}