fn main() {

    struct Vec2 {
        x: i32,
        y: i32,
    }
    impl Vec2 {
        fn len(&self) -> f64 {
            ((self.x * self.x + self.y * self.y) as f64).sqrt()
        }
    }

    trait Vec {
        fn len(&self) -> f64;
    }

    impl Vec for Vec2 {
        fn len(&self) -> f64 {
            ((self.x * self.x + self.y * self.y) as f64).sqrt()
        }
    }

    fn print_len(v: &dyn Vec) {
        println!("len: {}", v.len());
    }
    
    let v2 = Vec2{ x: 10, y: 11 };
    
    println!("{}", Vec2::len(&v2));
    
    print_len(&v2);

    println!("Hello, world!");

    let a: u32 = 42;
    let b = a;
    let c: usize = 23;
    let d = 'x';
    let e = false;

    println!("a: {}", std::mem::size_of_val(&a));
    println!("b: {}", std::mem::size_of_val(&b));
    println!("c: {}", std::mem::size_of_val(&c));
    println!("d: {}", std::mem::size_of_val(&d));
    println!("e: {}", std::mem::size_of_val(&e));
}