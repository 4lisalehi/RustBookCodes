struct Point<T> {
    x: T,
    y: T,
}

struct MixedPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> MixedPoint<T, U> {
    fn mixup<V, W> (self, other: MixedPoint<V, W>) -> MixedPoint<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_generic<T>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![24, 50, 35, 450, 100];
    let result = largest(&number_list);
    let integer_point = Point { x: 5, y: 7 };
    let float_point = Point { x: 6.0, y: 8.0 };

    println!("{:?}", integer_point);
    println!("{:?}", float_point);
}
