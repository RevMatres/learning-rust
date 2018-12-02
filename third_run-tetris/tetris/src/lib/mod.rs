mod game;
mod renderer;

struct Honk {
    value: i32,
}

impl Honk {
    fn increment(&mut self) {
        self.value += 1;
    }

    fn runner<F>(&mut self, f: F) where
        F: Fn(&mut Self) {

            f(self);

            // The clojure only captures values from the surrounding scope as reference.
            // That doesn't affect its parameters, though (which is specified as a mutable
            // reference to the self object)!
        }
}

pub fn honk() {
    //println!("Honk.");
    //renderer::render();
    game::game();


    /*
    let mut Tonk = Honk { value: 33 };
    println!("Honk: {}", Tonk.value);

    Tonk.increment();
    Tonk.runner(|a| a.value += 3);

    println!("Tonk: {}", Tonk.value);
    */

}
