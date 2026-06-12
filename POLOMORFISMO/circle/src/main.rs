trait Display {
    fn display(&self) -> String;
}

struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn display(&self) -> String {
        format!("Point(x: {}, y: {})", self.x, self.y)
    }
}

struct Circle {
    x: i32,
    y: i32,
    radius: i32,
}

impl Display for Circle {
    fn display(&self) -> String {
        format!("Circle(x: {}, y: {}, radius: {})", self.x, self.y, self.radius)
    }
}

fn print_display(item: &impl Display) {
    println!("{}", item.display());
}

fn main() {
    let point = Point { x: 5, y: 10 };
    let circle = Circle { x: 15, y: 25, radius: 5 };

    print_display(&point);
    print_display(&circle);
}