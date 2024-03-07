pub struct NesController {
    // 0 - A
    // 1 - B
    // 2 - Select
    // 3 - Start
    // 4 - Up
    // 5 - Down
    // 6 - Left
    // 7 - Right
    bits: [bool; 8],
    cur_idx: usize,
}

impl NesController {
    pub fn new() -> Self {
        NesController {
            bits: [false; 8],
            cur_idx: 0,
        }
    }

    pub fn set_input(&mut self, input: usize) {
        self.bits[input] = true;
    }

    pub fn clear_input(&mut self, input: usize) {
        self.bits[input] = false;
    }

    pub fn poll(&mut self) {
        self.cur_idx = 0;
    }

    pub fn read_input(&mut self) -> u8 {
        let ret = self.bits[self.cur_idx] as u8;
        self.cur_idx = (self.cur_idx + 1).min(7);
        ret
    }
}
