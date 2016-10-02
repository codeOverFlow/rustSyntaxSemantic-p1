// suppress some warnings
#![allow(dead_code)]
#![allow(unused_variables)]


fn void_fn() -> () {
    println!("Message from a function with no parameters");
}

fn print_int(x: i32) -> () {
    println!("function with {} as parameter", x);
}

fn add(x: i32, y: i32) -> i32 {
    x+y // no need of ; because the last expression is the returned one
}

// example of diverging function (can't return anything)
fn diverge() -> ! { // '!' is used to mark a divergent function
    panic!("Never return");
}


fn main() {
    println!("-------------- BINDINGS --------------");
    let x = 5; // immutable binding for x and value 5
    println!("x = {}", x);

    let (x, y) = (1,2); // immutable binding for x and y with value 1 and 2
    println!("(x,y) = ({},{})", x, y);

    let x : u16 = 42; // explicit type declaration for x to be an u16 integer
    println!("x = {}", x);

    let mut x = 6; // mutable binding for x and value 6
    println!("x = {}", x);
    x = 7; // mutable binding can be changed while immutable can only be shadowed
    println!("x = {}", x);

    // example of shadowing with scope using previous x
    {
        let x = 42;
        println!("x = {}", x);
    }
    // note that the mutability AND the value weren't lost with shadowing
    println!("x = {}", x);

    println!("-------------- FUNCTIONS --------------");
    void_fn();
    print_int(4);
    println!("add 5 + 1 = {}", add(5, 1));
    // divergent function can be used as any type
    // let x: i32 = diverge();
    // let x: String = diverge();
    // functors
    let f : fn(i32,i32) -> i32 = add;
    let ff = add; // uses type inference
    
    println!("-------------- ARRAYS --------------");
    let a = [1,2,3];
    let a: [i32; 3] = [4,5,6];
    let a = [42; 3]; // a = [42, 42, 42] 
    println!("a = {:?}", a); // use debug display because array does not implement Trait Display
    println!("a has {} elements", a.len());
    println!("a has {:>04b} elements", a.len());
}
