
// methods for structs
struct Elements {
    fire: u8,
    water: u8,
    earth: u8,
    air: u8
}

// the implementation keyword defines methods for the Elements Object
impl Elements {
    // static method, doesn't depend on an instance, often used for constructors
    fn constructor(f:u8,w:u8,e:u8,a:u8) -> Elements {
        Elements {fire:f,water:w,earth:e,air:a}
    }

    fn what_do_you_get_if_you_multiply_9_by_6() -> u8 {
        42
    }

    // instance method, depends on the actual instance
    fn print_elements(&self){
        // NOTE: &self == self: &Self ; Self is the instance
        println!("fire: {}",self.fire);
        println!("water: {}",self.water);
        println!("earth: {}",self.earth);
        println!("air: {}",self.air);
    }
}

// methods for tuples
fn main(){

    let player_1 = Elements {fire: 24, water: 32, earth: 9, air: 255};
    player_1.print_elements();

    println!("");

    let player_2 = Elements::constructor(9,9,27,9);
    player_2.print_elements();

    println!("");
    println!("the answer: {}", Elements::what_do_you_get_if_you_multiply_9_by_6())
}
