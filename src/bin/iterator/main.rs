// allow you iterate over vary structures (array, map and custom data type) in a uniform way

fn main() {
    iter_vec();
}

fn iter_vec() {
    let v = vec![1, 2, 3];
    let v_iter = v.iter();
    for value in v_iter {
        println!("{}", value);
    }

    for value in v {
        println!("{}", value);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn iter_demonstation() {
        let v = vec![1, 2, 3];
        let mut v_iter = v.iter();

        assert_eq!(v_iter.next(), Some(&1));

        print!("{:?}", v);

        let mut v_iter_own = v.into_iter();
        assert_eq!(v_iter_own.next(), Some(1));

        // print!("{:?}", v);
    }

    #[test]
    fn iter_sum() {
        let v = vec![1, 2, 3];
        let v_iter = v.iter();
        let sum = v_iter.sum();
        assert_eq!(6, sum);
    }

    #[test]
    fn iter_map() {
        let v = vec![1, 2, 3];
        let v_result: Vec<_> = v.iter().map(|x| x + 1).collect();

        assert_eq!(v_result, vec![2, 3, 4]);
    }
}

// custom iterator
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Self { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn test_counter() {
    let mut counter = Counter::new();

    assert_eq!(Some(1), counter.next());
    assert_eq!(Some(2), counter.next());
    assert_eq!(Some(3), counter.next());
    assert_eq!(Some(4), counter.next());
    assert_eq!(Some(5), counter.next());
    assert_eq!(None, counter.next());
}

#[test]
fn use_iter_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new())
        .skip(1)
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    assert_eq!(9, sum);
}
