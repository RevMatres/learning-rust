pub struct Tetris {
    changed: bool,
    event_queue: Vec<Event>,
}

impl Tetris {
    // creates a new Tetris
    pub fn create() -> Tetris {
        Tetris {
            changed: false,
            event_queue: Vec::new(),
        }
    }

    // updates the internal state of Tetris, according to events on the internal event queue
    pub fn update(&mut self) {
        self.changed = true;
    }

    // returns value of `changed` field; `true`, if `update()` has been called and changes to the
    // internal state have occurred since the last call to `get_render_data()`, meaning a rerender is
    // required
    pub fn has_changed(&self) -> bool {
        return self.changed
    }

    // returns a read-pointer to the internal render-buffer
    pub fn get_render_data(&mut self) {
        self.has_rendered()
    }

    // sets the value of the `changed` field to false (after rerender has occured)
    pub fn has_rendered(&mut self) {
        self.changed = false
    }

    // adds a tetris event to the internal event queue
    pub fn add_input_event(&mut self, e: Event) {
        self.event_queue.push(e)
    }
    
    // for Debugging, lists the Event Queue as a Token list
    pub fn list_event_queue(&self) {
        let mut list = String::new();
        for event in &self.event_queue {
            match event {
                Event::Down =>   {list.push('d'); list.push(',')},
                Event::Rotate => {list.push('r'); list.push(',')},
                Event::Left =>   {list.push('l'); list.push(',')},
                Event::Right =>  {list.push('r'); list.push(',')},
            }
        }
        println!("{}", list);
    }
}

pub enum Event {
    Down,
    Rotate,
    Left,
    Right,
}
