const SIZE_OF_QUEUE: usize = 1000;
pub struct Queue {
    pub queue: [u8; SIZE_OF_QUEUE],
    pub insert_pointer: usize, //could also be called end_pointer
    pub get_pointer: usize,    //could also be called start_pointer
    pub alive: bool,
    pub size: usize,
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            queue: [0; SIZE_OF_QUEUE],
            insert_pointer: 0,
            get_pointer: 0,
            alive: true,
            size: 0,
        }
    }

    pub fn insert(&mut self, x: u8) {
        //might add the if !self.alive here
        //need to think first. (and do the math)
        self.queue[self.insert_pointer] = x;
        self.insert_pointer += 1;
        if self.insert_pointer == SIZE_OF_QUEUE {
            self.insert_pointer = 0;
        }
        self.size += 1;
    }

    pub fn is_correct_retrieve(&mut self, x: u8) {
        if self.size == 0 {
            self.alive = false;
        }

        if self.alive {
            self.alive = self.queue[self.get_pointer] == x;
            self.get_pointer += 1;
            if self.get_pointer == SIZE_OF_QUEUE {
                self.get_pointer = 0;
            }
            self.size -= 1;
        }
    }
}
