use crate::UI;

pub struct Events<'a> {
    ui: &'a mut UI,
}

impl<'a> Events<'a> {
    pub fn new(ui: &'a mut UI) -> Self {
        Self { ui }
    }

    pub fn say_hello(self: &Self) {
        self.ui.on_say_hello();
    }

    pub fn new_loop(self: &Self, loop_num: i32) {
        print!("[events] New loop: {}", loop_num);
    }
}
