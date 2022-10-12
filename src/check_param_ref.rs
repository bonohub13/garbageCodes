use std::ops::Add;

#[derive(Clone, Copy)]
struct Data {
    i: i32,
}

impl Data {
    pub fn new(i: &i32) -> Self {
        Self { i: i.clone() }
    }
}

impl Add for Data {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self { i: self.i + rhs.i }
    }
}

fn add(i: &mut i32, j: &i32) {
    *i = *i + *j;
}

fn add_datas(i: &mut Data, j: &Data) {
    *i = *i + *j;
}

fn main() {
    let mut a = 1;
    let b = 2;
    let mut obj_a = Data::new(&0);
    let mut obj_b = Data::new(&1);

    println!("a = {0}", a);
    println!("b = {0}", b);
    println!("obj_a = {0}", obj_a.i);
    println!("obj_b = {0}", obj_b.i);

    add(&mut a, &b);

    loop {
        add_datas(&mut obj_a, &obj_b);
        add_datas(&mut obj_b, &obj_a);

        if obj_a.i > 100 {
            break;
        }
        println!("obj_a = {0}", obj_a.i);
    }

    println!("a = {0}", a);
    println!("b = {0}", b);
    println!("obj_a = {0}", obj_a.i);
    println!("obj_b = {0}", obj_b.i);
}
