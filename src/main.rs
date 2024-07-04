/*
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
*/

fn main() {
    let _sum = 5 + 10;
    let _x: f32 = 5 as f32 /3 as f32;
    
    let _tup: (i32, f64) = (500, 9.8);
    let mut _arr: [i8; 3] = Default::default();
    arr[0] = 9 as i16; 

    let a = [1, 2, 3, 4, 5];
    println!("Enter your index");   

    let mut index: usize;

    loop {
        let mut index = String::new(); 

        std::io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line!");

        let index: usize = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number");
                continue;
            },
        }
    }   
    
    let element = a[index];

    println!("The value of the element is: {element}");
}
