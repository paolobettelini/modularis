use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackedBitArray {
    len: usize,
    bits_per_entry: u8,
    data: Vec<u64>,
}

impl PackedBitArray {
    pub fn filled(len: usize, bits_per_entry: u8, value: u32) -> Self {
        let mut packed = Self {
            len,
            bits_per_entry: validate_bits(bits_per_entry),
            data: vec![0; word_len(len, bits_per_entry)],
        };
        for index in 0..len {
            packed.set(index, value);
        }
        packed
    }

    pub fn from_parts(
        len: usize,
        bits_per_entry: u8,
        data: Vec<u64>,
    ) -> Result<Self, PackedBitArrayError> {
        let bits_per_entry = validate_bits(bits_per_entry);
        let expected = word_len(len, bits_per_entry);
        if data.len() != expected {
            return Err(PackedBitArrayError::InvalidWordCount {
                expected,
                actual: data.len(),
            });
        }
        Ok(Self {
            len,
            bits_per_entry,
            data,
        })
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub const fn bits_per_entry(&self) -> u8 {
        self.bits_per_entry
    }

    pub fn data(&self) -> &[u64] {
        &self.data
    }

    pub fn get(&self, index: usize) -> u32 {
        assert!(index < self.len, "packed array index out of bounds");
        read_packed(&self.data, self.bits_per_entry, index)
    }

    pub fn set(&mut self, index: usize, value: u32) {
        assert!(index < self.len, "packed array index out of bounds");
        assert!(
            u64::from(value) <= entry_mask(self.bits_per_entry),
            "value does not fit packed entry"
        );
        write_packed(&mut self.data, self.bits_per_entry, index, value);
    }

    pub fn repack(&mut self, bits_per_entry: u8) {
        let bits_per_entry = validate_bits(bits_per_entry);
        if bits_per_entry == self.bits_per_entry {
            return;
        }
        let mut replacement = Self::filled(self.len, bits_per_entry, 0);
        for index in 0..self.len {
            replacement.set(index, self.get(index));
        }
        *self = replacement;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackedBitArrayError {
    InvalidWordCount { expected: usize, actual: usize },
}

pub const fn bits_required(value_count: usize) -> u8 {
    if value_count <= 1 {
        return 1;
    }
    (usize::BITS - (value_count - 1).leading_zeros()) as u8
}

fn validate_bits(bits_per_entry: u8) -> u8 {
    assert!(
        (1..=32).contains(&bits_per_entry),
        "bits per entry must be in 1..=32"
    );
    bits_per_entry
}

fn word_len(len: usize, bits_per_entry: u8) -> usize {
    (len * bits_per_entry as usize).div_ceil(u64::BITS as usize)
}

fn entry_mask(bits_per_entry: u8) -> u64 {
    (1_u64 << bits_per_entry) - 1
}

fn read_packed(data: &[u64], bits_per_entry: u8, index: usize) -> u32 {
    let bit_index = index * bits_per_entry as usize;
    let word_index = bit_index / u64::BITS as usize;
    let bit_offset = bit_index % u64::BITS as usize;
    let mask = entry_mask(bits_per_entry);
    let mut value = data[word_index] >> bit_offset;
    if bit_offset + bits_per_entry as usize > u64::BITS as usize {
        value |= data[word_index + 1] << (u64::BITS as usize - bit_offset);
    }
    (value & mask) as u32
}

fn write_packed(data: &mut [u64], bits_per_entry: u8, index: usize, value: u32) {
    let bit_index = index * bits_per_entry as usize;
    let word_index = bit_index / u64::BITS as usize;
    let bit_offset = bit_index % u64::BITS as usize;
    let mask = entry_mask(bits_per_entry);
    let value = u64::from(value) & mask;

    data[word_index] &= !(mask << bit_offset);
    data[word_index] |= value << bit_offset;

    let overflow = bit_offset + bits_per_entry as usize;
    if overflow > u64::BITS as usize {
        let spill_bits = overflow - u64::BITS as usize;
        let spill_mask = (1_u64 << spill_bits) - 1;
        data[word_index + 1] &= !spill_mask;
        data[word_index + 1] |= value >> (bits_per_entry as usize - spill_bits);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_and_writes_entries_crossing_word_boundaries() {
        let mut packed = PackedBitArray::filled(40, 5, 0);
        for index in 0..40 {
            packed.set(index, (index % 31) as u32);
        }
        for index in 0..40 {
            assert_eq!(packed.get(index), (index % 31) as u32);
        }
    }

    #[test]
    fn repacks_without_losing_values() {
        let mut packed = PackedBitArray::filled(128, 2, 0);
        for index in 0..128 {
            packed.set(index, (index % 4) as u32);
        }
        packed.repack(5);
        for index in 0..128 {
            assert_eq!(packed.get(index), (index % 4) as u32);
        }
    }
}
