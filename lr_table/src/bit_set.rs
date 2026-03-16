pub struct BitSet {
  bits: Vec<u64>,
  len: usize,
}

impl BitSet {
  pub fn new(len: usize) -> Self {
    Self {
      bits: vec![0; len.div_ceil(u64::BITS as usize)],
      len,
    }
  }

  fn pos(bit: usize) -> (usize, u32) {
    (bit / u64::BITS as usize, bit as u32 % u64::BITS)
  }

  pub fn get(&self, bit: usize) -> bool {
    debug_assert!(bit < self.len);
    let (index, shift) = Self::pos(bit);
    (self.bits[index] & (1 << shift)) != 0
  }

  pub fn set(&mut self, bit: usize) {
    debug_assert!(bit < self.len);
    let (index, shift) = Self::pos(bit);
    self.bits[index] |= 1 << shift;
  }

  pub fn clear(&mut self, bit: usize) {
    debug_assert!(bit < self.len);
    let (index, shift) = Self::pos(bit);
    self.bits[index] &= !(1 << shift);
  }
}
