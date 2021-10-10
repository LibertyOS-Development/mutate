#![no_std]
#[macro_export]
macro_rules! mutate
{
	($initval:expr, {
		$( $field:ident $( : $val:expr )? ),+ $(,)?
	}) => ({
		let mut item = $initval;
		$( $crate::mutate!(@mutate item $field $( : $val )?); )+
		item
	});
	(@mutate $item:ident $field:ident : $val:expr) => {
		$item.$field = $val;
	};
	(@mutate $item:ident $field:ident) => {
		$item.$field = $field;
	};
}
