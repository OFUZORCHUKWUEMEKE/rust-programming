// We use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types.

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item
        }
    }
    largest
}

// fn largestGeneric<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item
//         }
//     }
//     largest
// }

enum Result<T, E> {
    Ok(T),
    Err(E),
}

enum Option<T> {
    Some(T),
    None,
}

struct Point<T> {
    x: T,
    y: T,
}

struct Pointer<T, U> {
    x: T,
    y: U,
}

// Methods

struct Pointee<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let both_integer = Pointer { x: 5, y: 10 };

    let both_float = Pointer { x: 1.0, y: 4.0 };

    let integer_and_float = Pointer { x: 5, y: 4.0 };

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);

    print!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);

    println!("The largest number is {}", result)
}
