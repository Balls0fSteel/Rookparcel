//! Fixed-length bit set used for piece maps (libtorrent `bitfield` / `typed_bitfield`).
//!
//! Minimal Phase 0 version; Phase 1 ports the rest of `bitfield.cpp` (popcount fast paths,
//! `find_first_set`, wire-format helpers).

/// Bit `i` lives in byte `i / 8` at mask `0x80 >> (i % 8)`, the BitTorrent wire order, so the
/// bytes can be sent as a `bitfield` message without conversion.
#[derive(Clone, PartialEq, Eq, Default)]
pub struct Bitfield {
    bytes: Vec<u8>,
    len: usize,
}

impl Bitfield {
    pub fn new(len: usize, value: bool) -> Bitfield {
        let mut bf = Bitfield { bytes: vec![if value { 0xff } else { 0 }; len.div_ceil(8)], len };
        bf.clear_trailing_bits();
        bf
    }

    /// Builds a bitfield from wire-format bytes. Returns `None` if `bytes` is not exactly
    /// `ceil(len / 8)` long. Spare trailing bits are cleared (libtorrent ignores them too).
    pub fn from_bytes(bytes: &[u8], len: usize) -> Option<Bitfield> {
        if bytes.len() != len.div_ceil(8) {
            return None;
        }
        let mut bf = Bitfield { bytes: bytes.to_vec(), len };
        bf.clear_trailing_bits();
        Some(bf)
    }

    /// Grows or shrinks to `len` bits; new bits get `value` (libtorrent `bitfield::resize`).
    pub fn resize(&mut self, len: usize, value: bool) {
        let old = self.len;
        self.bytes.resize(len.div_ceil(8), if value { 0xff } else { 0 });
        self.len = len;
        if value {
            for i in old..len.min(old.div_ceil(8) * 8) {
                self.set(i, true);
            }
        }
        self.clear_trailing_bits();
    }

    pub fn set_all(&mut self, value: bool) {
        self.bytes.fill(if value { 0xff } else { 0 });
        self.clear_trailing_bits();
    }

    /// True if every bit set in `self` is also set in `other`.
    pub fn is_subset_of(&self, other: &Bitfield) -> bool {
        self.len == other.len && self.bytes.iter().zip(&other.bytes).all(|(a, b)| a & !b == 0)
    }

    pub fn none_set(&self) -> bool {
        self.bytes.iter().all(|&b| b == 0)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get(&self, i: usize) -> bool {
        assert!(i < self.len, "bit {i} out of range {}", self.len);
        self.bytes[i / 8] & (0x80 >> (i % 8)) != 0
    }

    pub fn set(&mut self, i: usize, value: bool) {
        assert!(i < self.len, "bit {i} out of range {}", self.len);
        let mask = 0x80 >> (i % 8);
        if value { self.bytes[i / 8] |= mask } else { self.bytes[i / 8] &= !mask }
    }

    pub fn count(&self) -> usize {
        self.bytes.iter().map(|b| b.count_ones() as usize).sum()
    }

    pub fn all_set(&self) -> bool {
        self.count() == self.len
    }

    /// Wire-format bytes (trailing pad bits are always zero).
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn iter(&self) -> impl Iterator<Item = bool> + '_ {
        (0..self.len).map(|i| self.get(i))
    }

    /// Whether every bit in `start..end` is set, looked at from `from` outwards: first up to
    /// `end`, then down to `start`, a whole byte at a time where it can. A range whose missing
    /// bit is near `from` (the piece just passed, in a sequential download) is answered at once.
    pub fn all_set_in(&self, start: usize, end: usize, from: usize) -> bool {
        let end = end.min(self.len);
        if start >= end {
            return true;
        }
        let from = from.clamp(start, end);
        let mut i = from;
        while i < end {
            if i % 8 == 0 && i + 8 <= end {
                if self.bytes[i / 8] != 0xff {
                    return false;
                }
                i += 8;
            } else {
                if !self.get(i) {
                    return false;
                }
                i += 1;
            }
        }
        let mut i = from;
        while i > start {
            if i % 8 == 0 && i - 8 >= start {
                if self.bytes[i / 8 - 1] != 0xff {
                    return false;
                }
                i -= 8;
            } else {
                i -= 1;
                if !self.get(i) {
                    return false;
                }
            }
        }
        true
    }

    fn clear_trailing_bits(&mut self) {
        if self.len % 8 != 0 {
            let last = self.bytes.len() - 1;
            self.bytes[last] &= 0xff << (8 - self.len % 8);
        }
    }
}

impl std::fmt::Debug for Bitfield {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bitfield({}/{} set)", self.count(), self.len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wire_order_and_padding() {
        let mut bf = Bitfield::new(10, false);
        bf.set(0, true);
        bf.set(9, true);
        assert_eq!(bf.as_bytes(), &[0x80, 0x40]);
        assert_eq!(bf.count(), 2);
        let full = Bitfield::new(10, true);
        assert_eq!(full.as_bytes(), &[0xff, 0xc0]);
        assert!(full.all_set());
        assert!(Bitfield::new(0, true).all_set());
    }

    #[test]
    fn from_bytes_and_subsets() {
        assert!(Bitfield::from_bytes(&[0xff], 9).is_none());
        let bf = Bitfield::from_bytes(&[0xff, 0xff], 9).unwrap();
        assert_eq!(bf.as_bytes(), &[0xff, 0x80], "spare bits are cleared");
        let mut half = Bitfield::new(9, false);
        assert!(half.none_set() && half.is_subset_of(&bf));
        half.set(3, true);
        assert!(half.is_subset_of(&bf) && !bf.is_subset_of(&half));
        half.set_all(true);
        assert_eq!(half, bf);
    }

    #[test]
    fn all_set_in_a_range_from_any_start() {
        let mut bf = Bitfield::new(40, true);
        bf.set(3, false);
        bf.set(30, false);
        for from in [0, 3, 4, 10, 17, 29, 31, 40, 99] {
            assert!(bf.all_set_in(4, 30, from), "{from}");
            assert!(bf.all_set_in(31, 40, from), "{from}");
            assert!(!bf.all_set_in(3, 4, from), "{from}");
            assert!(!bf.all_set_in(0, 40, from), "{from}");
            assert!(!bf.all_set_in(8, 31, from), "{from}");
            assert!(!bf.all_set_in(2, 9, from), "{from}");
        }
        assert!(bf.all_set_in(5, 5, 5), "an empty range");
        assert!(bf.all_set_in(31, 100, 0), "clamped to the length");
        // brute force against `get`
        let mut x = 0x2545_f491u32;
        let mut bf = Bitfield::new(77, false);
        for i in 0..77 {
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            bf.set(i, x % 7 != 0);
        }
        for s in 0..77 {
            for e in s..=77 {
                let want = (s..e).all(|i| bf.get(i));
                for from in [s, (s + e) / 2, e] {
                    assert_eq!(bf.all_set_in(s, e, from), want, "{s}..{e} from {from}");
                }
            }
        }
    }

    #[test]
    fn resize_keeps_bits_and_fills_new_ones() {
        let mut bf = Bitfield::new(3, false);
        bf.set(1, true);
        bf.resize(12, true);
        assert_eq!(bf.as_bytes(), &[0x5f, 0xf0]);
        bf.resize(2, false);
        assert_eq!((bf.len(), bf.as_bytes()), (2, &[0x40][..]));
        bf.resize(9, false);
        assert_eq!(bf.as_bytes(), &[0x40, 0x00]);
        let mut empty = Bitfield::default();
        empty.resize(5, true);
        assert!(empty.all_set() && empty.len() == 5);
    }
}
