use std::{thread, time::Duration};

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation: calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {}

// for Closuer, no need to specify the input parameter type and return value. The compile can infer the type and return value just like variables
fn generate_workout(num: u32) {
    let mut cached_result = Cacher::new(|num| {
        thread::sleep(Duration::from_secs(2));
        num
    });
}

// Unlike functions, Closuer have access to variables that are defined within the scope in which the Closuer is defined
fn closuer_env() {
    let x = 4;
    let equal_to_x = |z| x == z;

    print!("{}", x);

    let y = 4;
    assert!(equal_to_x(y));
}

// When you create a closure, rust infers which of the three traits to use based on how you use the value inside the closure env
// Fn, FnOnce, FnMut
// We could force the Closure to take the ownership of the value used by specifying the move keyworkd in front of the closure defination
fn closuer_env_FnOnce() {
    let x = vec![2, 3, 4];
    let equal_to_x = move |z| x == z;

    // print!("{:?}", x);

    let y = vec![2, 3, 4];
    assert!(equal_to_x(y));
}
