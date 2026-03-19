use crate::iter_ones::IterOnes;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BitSet {
  bits: Vec<u64>,
  len: usize,
}

impl BitSet {
  pub fn new(len: usize) -> Self {
    debug_assert_ne!(len, 0);
    Self {
      bits: vec![0; len.div_ceil(u64::BITS as usize)],
      len,
    }
  }

  fn pos(bit: usize) -> (usize, u32) {
    (bit / u64::BITS as usize, bit as u32 % u64::BITS)
  }

  pub fn has(&self, bit: usize) -> bool {
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

  /// Merges `other` into self, adding each entry of `other` which was not
  /// already in `self`. Returns true if `self` changed.
  pub fn merge(&mut self, other: &Self) -> bool {
    let mut modified = false;
    for (dst, src) in self.bits.iter_mut().zip(&other.bits) {
      let prev_dst = *dst;
      *dst |= src;
      modified = (*dst != prev_dst) || modified;
    }
    modified
  }

  pub fn for_each(&self) -> impl Iterator<Item = usize> {
    self.bits.iter().enumerate().flat_map(|(index, mask)| {
      mask
        .iter_ones()
        .map(move |bit| index * u64::BITS as usize + bit as usize)
    })
  }

  pub fn full(&self) -> bool {
    if self.len.is_multiple_of(u64::BITS as usize) {
      self.bits.iter().all(|bitv| *bitv == u64::MAX)
    } else {
      let (&last, rest) = self.bits.split_last().unwrap();
      rest.iter().all(|bitv| *bitv == u64::MAX)
        && last == ((1u64 << ((self.len as u32) % u64::BITS)) - 1)
    }
  }
}

#[cfg(test)]
mod tests {
  use googletest::prelude::*;

  use crate::bit_set::BitSet;

  #[gtest]
  fn test_full_odd_size_linear_fill() {
    const SIZE: usize = 147;
    let mut bits = BitSet::new(SIZE);

    for i in 0..SIZE {
      expect_false!(bits.full());
      bits.set(i);
    }

    expect_true!(bits.full());
  }

  #[gtest]
  fn test_full_odd_size_linear_reverse_fill() {
    const SIZE: usize = 147;
    let mut bits = BitSet::new(SIZE);

    for i in 0..SIZE {
      expect_false!(bits.full());
      bits.set(SIZE - i - 1);
    }

    expect_true!(bits.full());
  }

  #[gtest]
  fn test_full_odd_size_sporadic_fill() {
    const SIZE: usize = 147;
    let mut bits = BitSet::new(SIZE);

    for i in 0..SIZE {
      expect_false!(bits.full());
      bits.set((i + 17) % SIZE);
    }

    expect_true!(bits.full());
  }

  #[gtest]
  fn test_full_even_size_linear_fill() {
    const SIZE: usize = 128;
    let mut bits = BitSet::new(SIZE);

    for i in 0..SIZE {
      expect_false!(bits.full());
      bits.set(i);
    }

    expect_true!(bits.full());
  }

  #[gtest]
  fn test_full_even_size_linear_reverse_fill() {
    const SIZE: usize = 128;
    let mut bits = BitSet::new(SIZE);

    for i in 0..SIZE {
      expect_false!(bits.full());
      bits.set(SIZE - i - 1);
    }

    expect_true!(bits.full());
  }

  #[gtest]
  fn test_full_even_size_sporadic_fill() {
    const SIZE: usize = 128;
    let mut bits = BitSet::new(SIZE);

    for i in 0..SIZE {
      expect_false!(bits.full());
      bits.set((i + 17) % SIZE);
    }

    expect_true!(bits.full());
  }
}
