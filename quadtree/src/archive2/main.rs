pub mod app;
pub mod state;
pub mod quadtree;
pub mod inst;

fn main() {
    // window::run();
    println!("{:?}", inst::generate_random_circles(3));
    app::main();
}
