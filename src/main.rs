// fn main() {
//     let rect1 = (30 , 50);

//     println!("The area is {}",area(rect1));
// }


// fn area(dim: (u32, u32)) -> u32{
//     dim.0 * dim.1
// }

//refactoring with structs

#[derive(Debug)] //enables debug format
struct Rectangle {
    width: u32,
    height: u32,
}

fn main(){
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30*scale), // dbg! returns ownership of the expression’s value
        height: 50,
    };

    println!("The rect is {rect1:#?}"); //:? or :#? tells the macro to print in 'debug' format
    println!("The area is {}",areaFind(&rect1));

    dbg!(&rect1);

}

fn areaFind(rect1: &Rectangle) -> u32 { 
//borrowed immutable referrence in the signature cuz we don't want to 
//away the ownership from the main function.
    rect1.width * rect1.height
}