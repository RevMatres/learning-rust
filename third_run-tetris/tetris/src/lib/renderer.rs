pub fn render() {
    println!("render was requested to tell you this:");

    // game is imported into lib, so it's available via super:
    super::game::game_types::color_says_hello();
    println!("end of transmission");
}
