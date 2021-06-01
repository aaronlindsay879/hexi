use hexi_lib::hexi::Hexi;

fn main() {
    if let Err(e) = Hexi::default().run() {
        println!("\nSomething went wrong!\n{}", e);
    }
}
