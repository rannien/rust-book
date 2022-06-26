struct Color(i32, i32, i32); // tuple struct
struct Point(i32, i32, i32); // tuple struct
struct AlwaysEqual; // unit-like struct

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}
