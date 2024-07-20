
// fn main() {
//     let nums = vec![2, 6, 23, 1, 7, 0];

//     println!("Largest num: {}", largest(&nums));
    
//     let chars = vec!["c", "z", "e", "T"];
    
//     println!("Largest char: {}", largest(&chars));
// }

// fn largest<T: PartialOrd + Copy>(list: &Vec<T>) -> &T {
//     let mut largest = &list[0];

//     for item in list{
//         if item > largest {
//             largest = item;
//         } 
//     }

//     largest
// }




// <------------   Generics with structs --------------->

// struct Point<T>{
//     x: T,
//     y: T
// }

// #[derive(Debug)]
// struct Point<T, U>{
//     x: T,
//     y: U
// }

// impl<T, U> Point<T, U> {
//     fn x(&self) -> &T {
//         &self.x
//     }

//     fn mixup<A, B>(self, p: Point<A, B>) -> Point<T, B>{
//         Point{
//             x: self.x,
//             y: p.y
//         }
//     }
// }

// fn main() {
//     let p = Point { x:12, y:34.6 };

//     println!("p.x ==>    {}", p.x());

//     let p2 = Point { x:"This is x", y: "This is y" };

//     let p3 = p.mixup(p2);

//     println!("(p.x, p2.y) ==>    {:?}", p3);
// }




// <------------   Generics with enums --------------->
#[derive(Debug)]
enum Option<T>{
    Some(T),
    None
}

fn main(){
    let integer = Option::Some(123);
    let float = Option::Some(123.123);

    println!("Integer: {:?}, Float: {:?}", integer, float);
}