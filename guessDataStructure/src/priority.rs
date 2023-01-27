const SIZE: usize = 100 + 1;
pub struct Priority {
    pub values: [u16; SIZE],
    pub alive: bool,
    pub size: usize,
}

impl Priority {
    pub fn new() -> Priority {
        Priority {
            values:[0; SIZE],
            alive: true,
            size: 0,
        }
    }

    pub fn insert(&mut self, x: u16) {
        self.values[x as usize] += 1;
        self.size += 1
    }

    pub fn is_correct_retrieve(&mut self, x: u16) {
        //add to check if we trying to retrieve an empty list.
        if self.size == 0 {
            self.alive = false;
        }

        if self.alive {
            for index in (0..SIZE).rev() {
                if self.values[index] > 0 {
                    self.values[index] -= 1;
                    self.alive = index == x as usize;
                    self.size -= 1;
                    break;
                }
            }
        }
    }
}
