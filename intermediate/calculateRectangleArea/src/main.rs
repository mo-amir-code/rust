
struct Rectangle{
    width: f64,
    height: f64,
}

impl Rectangle{
    
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }

}


fn main(){
    let rect = Rectangle::new(12.3, 6.5);

    println!("Area of reactangle --> {}", rect.area());
}