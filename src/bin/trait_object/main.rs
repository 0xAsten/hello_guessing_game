trait Draw {
    fn draw(&self);
}

// dynamic dispactch vs static dispatch
struct Screen {
    pub compents: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for component in self.compents.iter() {
            component.draw();
        }
    }
}

struct Button();

struct SelectBox();

impl Draw for Button {
    fn draw(&self) {
        //
    }
}

impl Draw for SelectBox {
    fn draw(&self) {
        //
    }
}

fn main() {
    let screen = Screen {
        compents: vec![Box::new(SelectBox()), Box::new(Button())],
    };

    screen.run();
}
