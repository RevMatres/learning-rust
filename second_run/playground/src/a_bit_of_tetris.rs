mod lib {

    mod tetris {
        pub mod game_types {

            pub struct Wheee(String);
            impl Wheee {
                pub fn create(s: &str) -> Wheee {
                    Wheee(s.to_string())
                }
                pub fn print(&self) {
                    println!("{}", self.0)
                }
            }

        }
    }

    pub mod engine {
        pub fn go() {
            // I know, this link is absolutely fucking rediculous!
            let w = super::tetris::game_types::Wheee::create("hihihihihihihi");
            w.print();
        }
    }

}

/*
 * What To Do About All Of The Mess?
 *
 * - create commits for storage purposes
 * - create mod structure with some basic mock-func.s
 * - use Atom to copy the old code over
 * - continouously test the old code and the new interface with localized testing functions
 *    - take it slow, make sure each part actually fucking works
 *    - separately
 *    - make sure
 *    - srsly, test it all with main.rs by not using all other mods!!!
 * - then write friggin docs
 * - then you may proceed to the new and interesting things
 * - oh, and delete that pointless workspace folder from the mac that you've gotten there...
 *
 */

fn main() {
    lib::engine::go();
}
