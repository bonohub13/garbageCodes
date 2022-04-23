trait A {
    fn print(&self) {
        println!("This is default for print() in trait A.");
    }
}

struct X {}
struct Alpha {}
struct Y<'a> {
    a: &'a dyn A,
}

impl A for X {
    fn print(&self) {
        println!("I'm object X!");
    }
}

impl A for Alpha {
    fn print(&self) {
        println!("I'm object Alpha!");
    }
}

impl Default for X {
    fn default() -> Self {
        Self {}
    }
}

impl Default for Alpha {
    fn default() -> Self {
        Self {}
    }
}

impl<'a> Default for Y<'a> {
    fn default() -> Self {
        Self { a: &X {} }
    }
}

fn main() {
    let mut y = Y::default();
    let alpha = Alpha::default();

    y.a.print();

    y.a = &alpha;

    y.a.print();
}
