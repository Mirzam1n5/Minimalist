pub struct Uart {
    base_address: usize,
}

impl Uart {
    pub fn new(base_address: usize) -> Self {
        Uart {
            base_address
        }
    }

    pub fn init(&mut self) {
        let ptr = self.base_address as *mut u8;
        unsafe {
            ptr.add(3).write_volatile(0b11);
            ptr.add(2).write_volatile(1);
            ptr.add(1).write_volatile(1);
        }
    }
}