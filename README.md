# Simple 2D Vector
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/theboiboi8/simple_2d_vector?style=for-the-badge&labelColor=000000&color=dfff00)
[![Static Badge](https://img.shields.io/badge/available-crates.io-dfff00?style=for-the-badge&label=Available%20on&labelColor=000000&color=DFFF00)](https://crates.io/crates/simple_2d_vector)

Simple grid-based 2-dimensional vectors in Rust

## Getting Started
To get started with `simple_2d_vector`, add it to your project using `cargo add simple_2d_vector`.

You can then use it by using the provided `Vector2D` struct.

## Examples

#### Creating a `Vector2D` with `Vector2D::new()` and comparing it to a `Vector2D::null()`
```rust
use vector2d::Vector2D;

fn main() {
    let vector = Vector2D::new(
        (0.0, 0.0).into(), // The origin of the vector
        (0.0, 0.0).into()  // The target of the vector
    );
    
    // Null vectors are vectors with a length of zero
    // They are also called zero-length vectors as they only have an origin
    let null_vector = Vector2D::null((0.0, 0.0)); // A null vector
    
    assert_eq!(vector, null_vector); // The two vectors are the same
}
```

#### Performing addition and subtraction with vectors
```rust
use vector2d::Vector2D;

fn main() { 
    let vector1 = Vector2D::new(
        (10.0, 10.0).into(), 
        (10.0, 5.0).into()
    );
    
    let vector2 = Vector2D::new(
        (10.0, 10.0).into(), 
        (5.0, 10.0).into()
    );
    
    let result_vector_addition = Vector2D::new(
        (10.0, 10.0).into(), 
        (15.0, 15.0).into()
    );
    
    let result_vector_subtraction = Vector2D::new(
        (10.0, 10.0).into(), 
        (5.0, -5.0).into()
    );
    
    assert_eq!(vector1 + vector2, result_vector_addition);
    assert_eq!(vector1 - vector2, result_vector_subtraction);
}
```

#### Shifting a vector

```rust
use vector2d::Vector2D;

fn main() { 
    let vector = Vector2D::new(
        (10.0, 10.0).into(), 
        (10.0, 5.0).into()
    );
    
    // `Vector2D.shift` automatically converts applicable types into f32
    let shift = (-2i16, 1.25); // This allows for a mismatch of types
    
    // Shifting a vector moves only its `origin`,
    // as it's `target` is relative to its `origin`
    let result_vector = Vector2D::new(
        (8.0, 11.25).into(), 
        (10.0, 5.0).into()
    );
    
    assert_eq!(vector.shift(shift), result_vector);
}
```
