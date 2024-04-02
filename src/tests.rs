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
fn zero_vector() {
	let zero_vector = Vector2D::zero();
	let vector = Vector2D::new(
		(0.0, 0.0),
		(0.0, 0.0)
	);
	assert_eq!(vector, zero_vector);
}

#[test]
fn null_vector() {
	let null_vector = Vector2D::null((5.0, 5.0));
	let vector = Vector2D::new(
		(5.0, 5.0),
		(0.0, 0.0)
	);
	assert_eq!(vector, null_vector);
}
#[test]
fn with_absolute_target_vector() {
	let vector = Vector2D::new(
		(5.0, 5.0),
		(5.0, 5.0)
	);
	let absolute_target_vector = Vector2D::with_absolute_target(
		(5.0, 5.0),
		(10.0, 10.0)
	);
	assert_eq!(vector, absolute_target_vector);
}
#[test]
fn set_origin() {
	let set_origin_vector = Vector2D::zero().set_origin((1.1, 2.2));
	let vector = Vector2D::new(
		(1.1, 2.2),
		(0.0, 0.0)
	);
	assert_eq!(vector, set_origin_vector);
}
#[test]
fn set_target() {
	let set_target_vector = Vector2D::zero().set_target((1.1, 2.2));
	let vector = Vector2D::new(
		(0.0, 0.0),
		(1.1, 2.2)
	);
	assert_eq!(vector, set_target_vector);
}
#[test]
fn set_target_absolute() {
	let set_target_vector = Vector2D::null((1.0, 5.0)).set_target_absolute((1.1, 2.2));
	let vector = Vector2D::new(
		(1.0, 5.0),
		(0.1, -2.8)
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
}
#[test]
fn subtraction() {
	let vector1 = Vector2D::new(
		(10.0, 10.0),
		(10.0, 5.0)
	);
	let vector2 = Vector2D::new(
		(10.0, 10.0),
		(5.0, 10.0)
	);
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
#[test]
fn get_magnitude() {
	let error_margin = f64::EPSILON;
	let vector = Vector2D::new(
		(0.0, 0.0),
		(3.0, 4.0)
	);
	let magnitude = 5.0;
	assert!((vector.get_magnitude() - magnitude).abs() < error_margin);
}
#[test]
fn get_dot_product() {
	let error_margin = f64::EPSILON;
	let vector1 = Vector2D::new(
		(0.0, 0.0),
		(3.0, 4.0)
	);
	let vector2 = Vector2D::new(
		(0.0, 0.0),
		(5.0, 7.0)
	);
	let dot_product = 43.0;
	assert!((vector1.dot_product(vector2) - dot_product).abs() < error_margin);
}
#[test]
fn get_angle_between() {
	let error_margin = f64::EPSILON;
	let vector1 = Vector2D::new(
		(0.0, 0.0),
		(3.0, 4.0)
	);
	let vector2 = Vector2D::new(
		(0.0, 0.0),
		(5.0, 7.0)
	);
	let angle_between = 0.023_251_623;
	assert_eq!((vector1.get_angle_between(vector2)* 1000000000.0).round() / 1000000000.0, angle_between);
}