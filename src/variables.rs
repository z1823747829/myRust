struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

    fn origin() -> Point {
        Point { x: 0, y: 0 }
    }

    fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    fn get_x(&self) -> i32 {
        self.x
    }
    fn get_y(&self) -> i32 {
        self.y
    }
}
pub(crate) fn main() {
    let x=5;
    println!("The value of x is: {}", x);
    let x=6;
    println!("The value of x is: {}", x);
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of x is: {}", THREE_HOURS_IN_SECONDS);
    let p=Point::new(3, 4);
    println!("The value of x is: {},{}",p.x,p.y);
}
