// fn main() {
//     let rect1 = (30 , 50);

//     println!("The area is {}",area(rect1));
// }


// fn area(dim: (u32, u32)) -> u32{
//     dim.0 * dim.1
// }

//refactoring with structs

// #[derive(Debug)] //enables debug format
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main(){
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30*scale), // dbg! returns ownership of the expression’s value
//         height: 50,
//     };

//     println!("The rect is {rect1:#?}"); //:? or :#? tells the macro to print in 'debug' format
//     println!("The area is {}",areaFind(&rect1));

//     dbg!(&rect1);

// }

// fn areaFind(rect1: &Rectangle) -> u32 { 
// //borrowed immutable referrence in the signature cuz we don't want to 
// //away the ownership from the main function.
//     rect1.width * rect1.height
// }

//refactoring with methods.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//multiple impl blocks are allowed
impl Rectangle { //implementation block.
    //all methods are associated functions (functions inside the impl block)
    //but not all associated functions are methods
    //methods have self as their first parameter.


    fn area (&self) -> u32 { //&self implies the instance of the struct in which the method is called on
        self.width * self.height
    }

    fn can_hold(&self,other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //this associated function takes a 1d input and returns a rect struct with
    // the same value for size
    // self is an aliase for the struct type that comes after impl (Rectangle)
    // this function is namespaced by the structs, called as Rectangle::square(3).
    fn square (size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main(){
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 20,
        height: 60,
    };

    let sqr = Rectangle::square(3);

    println!("The area is {}",rect1.area());
    //calling area as a method of the struct Rectanlge.
    //rust automatically considers rect1.area() as &rect1.area()

    println!("Rect1 can hold Rect2: {}",rect1.can_hold(&rect2));
    println!("Rect1 can hold Rect3: {}",rect1.can_hold(&rect3));
    
}