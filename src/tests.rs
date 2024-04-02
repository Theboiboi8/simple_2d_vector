use std::ops::{Add, Sub};
use crate::Vector2D;

#[test]
fn new_vector() {
	let new_vector = Vector2D::new(
		(0.0, 0.0),
		(0.0, 0.0)
	);
	let vector = Vector2D {
		origin: (0.0, 0.0),
		target: (0.0, 0.0)
	};
	assert_eq!(vector, new_vector);
}
#[test]
fn null_vector() {
	let null_vector = Vector2D::null();
	let vector = Vector2D::new(
		(0.0, 0.0),
		(0.0, 0.0)
	);
	assert_eq!(vector, null_vector);
}
#[test]
fn set_origin() {
	let set_origin_vector = Vector2D::null().set_origin((1.1, 2.2));
	let vector = Vector2D::new(
		(1.1, 2.2),
		(0.0, 0.0)
	);
	assert_eq!(vector, set_origin_vector);
}
#[test]
fn set_target() {
	let set_target_vector = Vector2D::null().set_target((1.1, 2.2));
	let vector = Vector2D::new(
		(0.0, 0.0),
		(1.1, 2.2)
	);
	assert_eq!(vector, set_target_vector);
}
#[test]
fn display_vector() {
	let vector = Vector2D::new(
		(2.2, 1.1),
		(1.1, 2.2)
	);
	assert_eq!(format!("{vector}"), "(2.2,1.1)[1.1,2.2]");
}
#[test]
fn add_vectors() {
	let vector1 = Vector2D::new(
		(10.0, 10.0),
		(10.0, 5.0)
	);
	let vector2 = Vector2D::new(
		(10.0, 10.0),
		(5.0, 10.0)
	);
	let result_vector = Vector2D::new(
		(10.0, 10.0),
		(15.0, 15.0)
	);
	assert_eq!(vector1 + vector2, result_vector);
	assert_eq!(vector1 + vector2, vector1.add(vector2));
}
#[test]
fn addition() {
	let vector1 = Vector2D::new(
		(10.0, 10.0),
		(10.0, 5.0)
	);
	let vector2 = Vector2D::new(
		(10.0, 10.0),
		(5.0, 10.0)
	);
	assert_eq!(vector1 + vector2, vector1.add(vector2));
}
#[test]
fn sub_vectors() {
	let vector1 = Vector2D::new(
		(10.0, 10.0),
		(10.0, 5.0)
	);
	let vector2 = Vector2D::new(
		(10.0, 10.0),
		(5.0, 10.0)
	);
	let result_vector = Vector2D::new(
		(10.0, 10.0),
		(5.0, -5.0)
	);
	assert_eq!(vector1 - vector2, result_vector);
	assert_eq!(vector1 - vector2, vector1.sub(vector2));
}
#[test]
fn shift_positive() {
	let vector = Vector2D::new(
		(10.0, 10.0),
		(10.0, 5.0)
	);
	let tuple = (2u16, 1.25);
	let result_vector = Vector2D::new(
		(12.0, 11.25),
		(10.0, 5.0)
	);
	assert_eq!(vector.shift(tuple), result_vector);
}
#[test]
fn shift_negative() {
	let vector = Vector2D::new(
		(10.0, 10.0),
		(10.0, 5.0)
	);
	let tuple = (-2i16, -1.25);
	let result_vector = Vector2D::new(
		(8.0, 8.75),
		(10.0, 5.0)
	);
	assert_eq!(vector.shift(tuple), result_vector);
}
#[test]
fn shift_mixed() {
	let vector = Vector2D::new(
		(10.0, 10.0),
		(10.0, 5.0)
	);
	let tuple = (-2i16, 1.25);
	let result_vector = Vector2D::new(
		(8.0, 11.25),
		(10.0, 5.0)
	);
	assert_eq!(vector.shift(tuple), result_vector);
}