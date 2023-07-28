// decide between generics and associated types, the question is does it make sense to have multiple implementations for a single type

trait Iterator {
    type Item;

    fn next() -> Option<Self::Item>;
}

struct Counter {}

impl Iterator for Counter {
    type Item = u32;
    fn next() -> Option<Self::Item> {
        return Some(0);
    }
}

trait Iterator2<T> {
    fn next() -> Option<T>;
}

impl Iterator2<u32> for Counter {
    fn next() -> Option<u32> {
        return Some(0);
    }
}

impl Iterator2<u16> for Counter {
    fn next() -> Option<u16> {
        return Some(0);
    }
}

// generic type parameters could specify a defualt concrete type
// overloading operator Add
// trait Add<Rhs = Self> {
//     type output;

//     fn add(self, rhs: Rhs) -> self::output;
// }
use std::ops::Add;

struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// specify differenct type for default Rhs type
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + rhs.0 * 1000)
    }
}

// calling methods with the same name
trait Pilot {
    fn fly(&self);

    fn associated_fn();
}

trait Wizard {
    fn fly(&self);

    fn associated_fn();
}

struct Human;

impl Human {
    fn fly(&self) {
        println!("human fly");
    }

    fn associated_fn() {
        println!("human");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("Pilot fly");
    }

    fn associated_fn() {
        println!("Pilot");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Wizard fly");
    }

    fn associated_fn() {
        println!("Wizard");
    }
}

fn main() {
    let human = Human;
    Pilot::fly(&human);
    Wizard::fly(&human);
    Human::fly(&human);

    Human::associated_fn();
    <Human as Pilot>::associated_fn();
    <Human as Wizard>::associated_fn();
}

// super trait: a trait that depends on another trait
use std::fmt;

trait OutlinePrint: fmt::Display {}
