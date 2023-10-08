fn main() {

    let foo = Color::Green;

    foo.is_green();
    foo.is_green_parts();

}

enum Color {
    Red,
    Blue,
    Green,
    Yellow,
}

impl Color {
    // only need to read data
    fn is_green(&self) -> bool {
        if let Color::Green = self {
            return true;
        }
        return false;
    }

    fn is_green_parts(&self) -> bool {
        match self {
            Color::Red => return false,
            Color::Blue => return true,
            Color::Green => return false,
            Color::Yellow => return true,
        }
    }
}

fn print_color(c: Color){
    match c {
        Color::Red => println!("red"),
        Color::Blue => println!("blue"),
        Color::Green => println!("green"),
        Color::Yellow => println!("yellow"),
    }
}
