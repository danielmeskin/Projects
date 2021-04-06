use std::io;
fn main() {
    println!("Floor tile cost calculator");
    println!("Cost per square foot of tile? ");
    let mut tilecost = String::new();
    io::stdin()
        .read_line(&mut tilecost)
        .expect("Failed to read line");
    let tilecost: f32 = tilecost.trim().parse().expect("Please type a number!");
    println!("Width of floor to be tiled? (In feet)");
    let mut width = String::new();
    io::stdin()
        .read_line(&mut width)
        .expect("Failed to read line");
    let width: f32 = width.trim().parse().expect("Please type a number!");
    println!("Height of floor to be tiled (In feet)");
    let mut height = String::new();
    io::stdin()
        .read_line(&mut height)
        .expect("Failed to read line");
    let height: f32 = height.trim().parse().expect("Please type a number!");
    let square_footage = width * height;
    let cost= square_footage*tilecost;
    println!("The cost of tiling {} square feet of floor is: ${}", square_footage, cost);
}
