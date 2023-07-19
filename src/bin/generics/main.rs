#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mix<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![1, 2, 10, 100, 50, 25];
    let largest = fined_largest_number(number_list);
    println!("{}", largest);

    let p1 = Point { x: 1, y: 1.1 };
    println!("{:?}", p1);

    let p2 = Point { x: 'c', y: 1 };
    println!("{:?}", p2);
}

fn fined_largest_number<T: PartialOrd + Copy>(v: Vec<T>) -> T {
    let mut largest = v[0];
    for i in v {
        if i > largest {
            largest = i;
        }
    }

    largest
}
