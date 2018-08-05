extern crate tetris;
extern crate engine;
extern crate channels;

pub fn tetris_go_single_thread() {

    let engine_closure = || {
        return [0f32,0f32,0f32,0f32]
    };
    engine::go(engine_closure);

}
