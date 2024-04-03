use std::ops::{Add, Sub};
use crate::{Vector2D, Point2D};

#[test]
fn new_vector() {
	let new_vector = Vector2D::new(
		(0.0, 0.0).into(),
		(0.0, 0.0).into()
	);
	let vector = Vector2D {
		origin: (0.0, 0.0).into(),
		target: (0.0, 0.0).into()
	};
	assert_eq!(vector, new_vector);
}
#[test]
fn zero_vector() {
	let zero_vector = Vector2D::zero();
	let vector = Vector2D::new(
		(0.0, 0.0).into(),
		(0.0, 0.0).into()
	);
	assert_eq!(vector, zero_vector);
}

#[test]
fn null_vector() {
	let null_vector = Vector2D::null((5.0, 5.0).into());
	let vector = Vector2D::new(
		(5.0, 5.0).into(),
		(0.0, 0.0).into()
	);
	assert_eq!(vector, null_vector);
}
#[test]
fn with_absolute_target_vector() {
	let vector = Vector2D::new(
		(5.0, 5.0).into(),
		(5.0, 5.0).into()
	);
	let absolute_target_vector = Vector2D::with_absolute_target(
		(5.0, 5.0).into(),
		(10.0, 10.0).into()
	);
	assert_eq!(vector, absolute_target_vector);
}
#[test]
fn set_origin() {
	let set_origin_vector = Vector2D::zero().set_origin((1.1, 2.2).into());
	let vector = Vector2D::new(
		(1.1, 2.2).into(),
		(0.0, 0.0).into()
	);
	assert_eq!(vector, set_origin_vector);
}
#[test]
fn set_target() {
	let set_target_vector = Vector2D::zero().set_target((1.1, 2.2).into());
	let vector = Vector2D::new(
		(0.0, 0.0).into(),
		(1.1, 2.2).into()
	);
	assert_eq!(vector, set_target_vector);
}
#[test]
fn set_target_absolute() {
	let set_target_vector = Vector2D::null((1.0, 5.0).into()).set_target_absolute((1.1, 2.2).into());
	let vector = Vector2D::new(
		(1.0, 5.0).into(),
		(0.1, -2.8).into()
	);
	assert_eq!(vector, set_target_vector);
}
#[test]
fn display_vector() {
	let vector = Vector2D::new(
		(2.2, 1.1).into(),
		(1.1, 2.2).into()
	);
	assert_eq!(format!("{vector}"), "(2.2,1.1)[1.1,2.2]");
}
#[test]
fn add_vectors() {
	let vector1 = Vector2D::new(
		(10.0, 10.0).into(),
		(10.0, 5.0).into()
	);
	let vector2 = Vector2D::new(
		(10.0, 10.0).into(),
		(5.0, 10.0).into()
	);
	let result_vector = Vector2D::new(
		(10.0, 10.0).into(),
		(15.0, 15.0).into()
	);
	assert_eq!(vector1 + vector2, result_vector);
}
#[test]
fn addition() {
	let vector1 = Vector2D::new(
		(10.0, 10.0).into(),
		(10.0, 5.0).into()
	);
	let vector2 = Vector2D::new(
		(10.0, 10.0).into(),
		(5.0, 10.0).into()
	);
	assert_eq!(vector1 + vector2, vector1.add(vector2));
}
#[test]
fn sub_vectors() {
	let vector1 = Vector2D::new(
		(10.0, 10.0).into(),
		(10.0, 5.0).into()
	);
	let vector2 = Vector2D::new(
		(10.0, 10.0).into(),
		(5.0, 10.0).into()
	);
	let result_vector = Vector2D::new(
		(10.0, 10.0).into(),
		(5.0, -5.0).into()
	);
	assert_eq!(vector1 - vector2, result_vector);
}
#[test]
fn subtraction() {
	let vector1 = Vector2D::new(
		(10.0, 10.0).into(),
		(10.0, 5.0).into()
	);
	let vector2 = Vector2D::new(
		(10.0, 10.0).into(),
		(5.0, 10.0).into()
	);
	assert_eq!(vector1 - vector2, vector1.sub(vector2));
}
#[test]
fn shift_positive() {
	let vector = Vector2D::new(
		(10.0, 10.0).into(),
		(10.0, 5.0).into()
	);
	let tuple = (2u16, 1.25);
	let result_vector = Vector2D::new(
		(12.0, 11.25).into(),
		(10.0, 5.0).into()
	);
	assert_eq!(vector.shift(tuple), result_vector);
}
#[test]
fn shift_negative() {
	let vector = Vector2D::new(
		(10.0, 10.0).into(),
		(10.0, 5.0).into()
	);
	let tuple = (-2i16, -1.25);
	let result_vector = Vector2D::new(
		(8.0, 8.75).into(),
		(10.0, 5.0).into()
	);
	assert_eq!(vector.shift(tuple), result_vector);
}
#[test]
fn shift_mixed() {
	let vector = Vector2D::new(
		(10.0, 10.0).into(),
		(10.0, 5.0).into()
	);
	let tuple = (-2i16, 1.25);
	let result_vector = Vector2D::new(
		(8.0, 11.25).into(),
		(10.0, 5.0).into()
	);
	assert_eq!(vector.shift(tuple), result_vector);
}
#[test]
fn get_magnitude() {
	let error_margin = f64::EPSILON;
	let vector = Vector2D::new(
		(0.0, 0.0).into(),
		(3.0, 4.0).into()
	);
	let magnitude = 5.0;
	assert!((vector.get_magnitude() - magnitude).abs() < error_margin);
}
#[test]
fn get_dot_product() {
	let error_margin = f64::EPSILON;
	let vector1 = Vector2D::new(
		(0.0, 0.0).into(),
		(3.0, 4.0).into()
	);
	let vector2 = Vector2D::new(
		(0.0, 0.0).into(),
		(5.0, 7.0).into()
	);
	let dot_product = 43.0;
	assert!((vector1.dot_product(vector2) - dot_product).abs() < error_margin);
}
#[test]
fn get_angle_between() {
	let vector1 = Vector2D::new(
		(0.0, 0.0).into(),
		(3.0, 4.0).into()
	);
	let vector2 = Vector2D::new(
		(0.0, 0.0).into(),
		(5.0, 7.0).into()
	);
	let angle_between = 0.023_251_623;
	assert_eq!(
		(vector1.get_angle_between(vector2)* 1_000_000_000.0).round() / 1_000_000_000.0,
		angle_between
	);
}
#[test]
fn new_point() {
	let point = Point2D {
		x: 0.0,
		y: 1.1,
	};
	let new_point = Point2D::new(
		0.0, 1.1
	);
	assert_eq!(point, new_point);
}
#[test]
fn zero_point() {
	let point = Point2D::new(
		0.0,
		0.0
	);
	let zero_point = Point2D::zero();
	assert_eq!(point, zero_point);
}
#[test]
fn point_distance() {
	let point1 = Point2D::new(
		0.0,
		0.0
	);
	let point2 = Point2D::new(
		1.0,
		0.0
	);
	let distance = 1.0;
	assert_eq!(point1.get_distance(point2), distance);
}
#[test]
fn to_vector2d() {
	let point1 = Point2D::new(
		0.0,
		0.0
	);
	let point2 = Point2D::new(
		1.0,
		0.0
	);
	let vector = Vector2D::new(
		(0.0,0.0).into(),
		(1.0,0.0).into(),
	);
	assert_eq!(point1.to_vector2d(&point2), vector);
}