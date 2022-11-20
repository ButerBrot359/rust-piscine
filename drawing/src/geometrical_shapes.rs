use raster::{Color, Image};
use rand::Rng;


pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color() -> Color;
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

//Point
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {x, y}
    }

    pub fn random(width: i32, height: i32) -> Point {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range::<i32>(0, width + 1);
        let y = rng.gen_range::<i32>(0, height + 1);

        Point::new(x, y)
    }

}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.x, self.y, generate_random_color());
    }

    fn color() -> Color {
        generate_random_color()
    }
}

pub struct Line {
    pub begin: Point,
    pub end: Point,
}

impl Line {
    pub fn new(begin: &Point, end: &Point) -> Line {
        let deref_begin = Point::new(begin.x, begin.y);
        let deref_end = Point::new(end.x, end.y);

        Line {begin: deref_begin, end: deref_end}
    }

    pub fn random(width: i32, height: i32) -> Line {
        let begin = &Point::random(width, height);
        let end = &Point::random(width, height);

        Line::new(begin, end)
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let x1 = self.begin.x;
        let y1 = self.begin.y;

        let x2 = self.end.x;
        let y2 = self.end.y;

        for i in (x1..=x2) {
            let y = 
        }
    }

    fn color() -> Color {
        generate_random_color()
    }

}

// //Triangle
// pub struct Triangle {
//     pub vertices_1: Point,
//     pub vertices_2: Point,
//     pub vertices_3: Point
// }

// impl Triangle {
//     pub fn new() {}
// }

// //Rectangle
// pub struct Rectangle {
//     pub vertices_1: Point,
//     pub vertices_2: Point,
// }

// impl Rectangle {
//     pub fn new() {}
// }

// //Circle
// pub struct Circle {
//     pub center: Point,
//     pub radius: i32
// }

// impl Circle {
//     pub fn new() {}
//     pub fn random() {}
// }

fn generate_random_color() -> Color {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range::<u8>(0, 255);
    let g = rng.gen_range::<u8>(0, 255);
    let b = rng.gen_range::<u8>(0, 255);

    Color::rgb(r,g,b)
}

fn get_y_position(x1: i32, y1: i32, x2: i32, y2: i32, x: i32) -> i32 {
    let y = (((y2 - y1)*(x-x1))/(x2-x1)) + y1;

    y
}
