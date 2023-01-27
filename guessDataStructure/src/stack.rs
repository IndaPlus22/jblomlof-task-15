const SIZE_OF_STACK: usize = 1000;

pub struct Stack {
    
    /* an array where we store the 'stack pointer' to what element we lastly added.
    So we just add the last element to [pointer + 1]
    and then retrieve it with [pointer]
    stack size is 1 000, since max 1000 commands allowed
    (could prob be 500 or smth. but 1000 to be sure)
    */
    pub stack: [u8; SIZE_OF_STACK],
    pub pointer: usize,
    pub alive: bool,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            stack: [0; SIZE_OF_STACK],
            pointer: 0,
            alive: true,
        }
    }

    pub fn insert(&mut self, x: u8) {
        self.stack[self.pointer] = x;
        self.pointer += 1;

    }

    pub fn is_correct_retrieve(&mut self, x: u8) {
        if self.pointer == 0 {
            self.alive = false;
        }
        if self.alive {
        self.pointer -= 1;
        self. alive = self.stack[self.pointer] == x;
        }
    }
}