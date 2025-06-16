

# Generics on Primitive Types

```rs


fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");
}

// PartialOrd : trait implemented for both char and i32 for order comparisons

fn largest<T : PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


```

# Generics on Struct Types


```rs
struct Point<T,U> {
    x: T,
    y: U,
}

fn main() {
    let integer = Point { x: 5, y: 10.0 };
    let float = Point { x: 1.0, y: 4.0 };
}
```




# Generics on Enums 


```rs

enum Option<T> {
    Some(T),
    None,
}



enum Result<T, E> {
    Ok(T),
    Err(E),
}

```




# Generics In Method Definitions 


```rs

struct Point<T> {
    x: T,
    y: T,
}


// A must if you wish to make it generic else you can just use a concrete type ... 
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// concrete type ... 
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


// multiple types 
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

```