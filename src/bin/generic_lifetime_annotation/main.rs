struct ImportantExcerpt<'a> {
    part: &'a str,
}

// rule1: each parameter reference has to have a lifetime
// rule3: if one of our funciton input parameter is a reference to self, the all output lifetimes will be the same as self
impl<'a> ImportantExcerpt<'a> {
    fn return_part(&self) -> &str {
        self.part
    }
}

fn main() {
    let x = String::from("abc");

    let z1;
    let z2;

    {
        let y = String::from("xyz");

        z1 = longest_1(&x, &y);

        println!("The longest string is {}", z1);

        z2 = longest_2(&x, &y);
    }

    println!("The longest string is {}", z2);
}

fn longest_1<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
