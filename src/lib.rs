

pub struct Buffer {
    pub index: usize,
    pub length: usize,
    pub data: Vec<u8>,
}

pub fn new_buffer(size: i32) -> Buffer {
    Buffer {
        index: 0,
        length: size as usize,
        data: Vec::with_capacity(size as usize),
    }
}

impl Buffer {
    pub fn advance(&mut self, amount: usize) -> usize {
        self.index += amount;
        return amount;
    }
    pub fn reset(&mut self) {
        self.index = 0;
        for x in self.data.iter_mut() {
            *x = 0;
        }
    }
    pub fn junk_fill(&mut self) {
        for number in 0..self.data.capacity() {
            self.data.push(number as u8);
        }
    }
}

const SIZE_BYTE: usize = 1;
const SIZE_SHORT: usize = 2;
const SIZE_INT: usize = 4;
const SIZE_LONG: usize = 8;

impl Buffer {
    //Read functions
    pub fn read_u64(&mut self) -> u64 {
        let s = &self.data[self.index..self.index + SIZE_LONG];
        self.index += SIZE_LONG;
        return (s[7] as u64)
            | (s[6] as u64) << 8
            | (s[5] as u64) << 16
            | (s[4] as u64) << 24
            | (s[3] as u64) << 32
            | (s[2] as u64) << 40
            | (s[1] as u64) << 48
            | (s[0] as u64) << 56;
    }
    pub fn read_u32(&mut self) -> u32 {
        let s = &self.data[self.index..self.index + SIZE_INT];
        self.index += SIZE_INT;
        return (s[3] as u32) | (s[2] as u32) << 8 | (s[1] as u32) << 16 | (s[0] as u32) << 24;
    }
    pub fn read_u16(&mut self) -> u16 {
        let s = &self.data[self.index..self.index + SIZE_SHORT];
        self.index += SIZE_SHORT;
        return (s[1] as u16) | (s[0] as u16) << 8;
    }
    pub fn read_u8(&mut self) -> u8 {
        let s = &self.data[self.index];
        self.index += SIZE_BYTE;
        return *s as u8;
    }
    pub fn read_f64(&mut self) -> f64 {
        let s = self.read_u64();
        self.index += SIZE_LONG;
        return s as f64;
    }
    pub fn read_f32(&mut self) -> f32 {
        let s = self.read_u32();
        self.index += SIZE_INT;
        return s as f32;
    }
    pub fn read_string() -> String {
        return String::from("");
    }

    // Write functions
    pub fn write_u64(&mut self, num: u64) {
        let mut x: [u8; SIZE_LONG] = [0; SIZE_LONG];
        x[0] = (num >> 56) as u8;
        x[1] = (num >> 48) as u8;
        x[2] = (num >> 40) as u8;
        x[3] = (num >> 32) as u8;
        x[4] = (num >> 24) as u8;
        x[5] = (num >> 16) as u8;
        x[6] = (num >> 8) as u8;
        x[7] = (num) as u8;
        for item in x.iter() {
            self.data.push(*item);
        }
        self.index += SIZE_LONG;
    }
    pub fn write_u32(&mut self, num: u32) {
        let mut x: [u8; SIZE_INT] = [0; SIZE_INT];
        x[0] = (num >> 24) as u8;
        x[1] = (num >> 16) as u8;
        x[2] = (num >> 8) as u8;
        x[3] = (num) as u8;
        for item in x.iter() {
            self.data.push(*item);
        }
        self.index += SIZE_INT;
    }
    pub fn write_u16(&mut self, num: u16) {
        let mut x: [u8; SIZE_SHORT] = [0; SIZE_SHORT];
        x[0] = (num >> 8) as u8;
        x[1] = (num) as u8;
        for item in x.iter() {
            self.data.push(*item);
        }
        self.index += SIZE_SHORT;
    }
    pub fn write_u8(&mut self, num: u8) {
        self.data.push(num);
        self.index += SIZE_BYTE;
    }
    pub fn write_f64(&mut self, num: f64) {
        self.write_u64(num as u64);
    }
    pub fn write_f32(&mut self, num: f32) {
        self.write_u32(num as u32);
    }
}
