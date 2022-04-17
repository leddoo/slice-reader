#[derive(Clone)]
pub struct Reader<'buf, T> {
    pub buffer: &'buf [T],

    /// <= buffer.len()
    pub cursor: usize,
}

impl<'rdr, T> Reader<'rdr, T> {
    pub fn new(buffer: &'rdr [T]) -> Reader<'rdr, T> {
        Reader { buffer: buffer, cursor: 0 }
    }

    pub fn remaining(&self) -> usize {
        self.buffer.len() - self.cursor
    }

    pub fn rest(&self) -> &'rdr [T] {
        &self.buffer[self.cursor..]
    }


    pub fn empty(&self) -> bool {
        !self.has_some()
    }

    pub fn has_some(&self) -> bool {
        self.cursor < self.buffer.len()
    }


    pub fn peek_at(&self, offset: usize) -> Option<&'rdr T> {
        self.buffer.get(self.cursor + offset)
    }

    pub fn peek(&self) -> Option<&'rdr T> {
        self.buffer.get(self.cursor)
    }

    pub fn next(&mut self) -> Option<&'rdr T> {
        self.peek().map(|result| {
            self.cursor += 1;
            result
        })
    }


    pub fn has_n(&self, n: usize) -> bool {
        self.cursor + n <= self.buffer.len()
    }

    pub fn peek_n(&self, n: usize) -> Option<&'rdr [T]> {
        if self.has_n(n) {
            return Some(&self.buffer[self.cursor .. self.cursor + n]);
        }
        None
    }

    pub fn next_n(&mut self, n: usize) -> Option<&'rdr [T]> {
        self.peek_n(n).map(|result| {
            self.cursor += n;
            result
        })
    }
}



use byte_order::*;

impl<'rdr> Reader<'rdr, u8> {
    pub fn peek_bytes<const N: usize>(&self) -> Option<[u8; N]> {
        let mut bytes = [0; N];
        bytes.copy_from_slice(self.peek_n(N)?);
        Some(bytes)
    }

    pub fn next_bytes<const N: usize>(&mut self) -> Option<[u8; N]> {
        self.peek_bytes::<N>().map(|result| {
            self.cursor += N;
            result
        })
    }

    pub fn peek_bytes_endian<const N: usize, B: ByteOrder>(&self) -> Option<[u8; N]> {
        self.peek_bytes::<N>().map(B::swap_bytes)
    }

    pub fn next_bytes_endian<const N: usize, B: ByteOrder>(&mut self) -> Option<[u8; N]> {
        self.next_bytes::<N>().map(B::swap_bytes)
    }
}


// generated using gen.py
impl<'rdr> Reader<'rdr, u8> {
    pub fn peek_u8<B: ByteOrder>(&self) -> Option<u8> { self.peek_bytes::<1>().map(B::read_u8) }
    pub fn peek_u8_ne(&self) -> Option<u8> { self.peek_u8::<NativeEndian>() }
    pub fn peek_u8_le(&self) -> Option<u8> { self.peek_u8::<LittleEndian>() }
    pub fn peek_u8_be(&self) -> Option<u8> { self.peek_u8::<BigEndian>() }
    pub fn next_u8<B: ByteOrder>(&mut self) -> Option<u8> { self.next_bytes::<1>().map(B::read_u8) }
    pub fn next_u8_ne(&mut self) -> Option<u8> { self.next_u8::<NativeEndian>() }
    pub fn next_u8_le(&mut self) -> Option<u8> { self.next_u8::<LittleEndian>() }
    pub fn next_u8_be(&mut self) -> Option<u8> { self.next_u8::<BigEndian>() }

    pub fn peek_u16<B: ByteOrder>(&self) -> Option<u16> { self.peek_bytes::<2>().map(B::read_u16) }
    pub fn peek_u16_ne(&self) -> Option<u16> { self.peek_u16::<NativeEndian>() }
    pub fn peek_u16_le(&self) -> Option<u16> { self.peek_u16::<LittleEndian>() }
    pub fn peek_u16_be(&self) -> Option<u16> { self.peek_u16::<BigEndian>() }
    pub fn next_u16<B: ByteOrder>(&mut self) -> Option<u16> { self.next_bytes::<2>().map(B::read_u16) }
    pub fn next_u16_ne(&mut self) -> Option<u16> { self.next_u16::<NativeEndian>() }
    pub fn next_u16_le(&mut self) -> Option<u16> { self.next_u16::<LittleEndian>() }
    pub fn next_u16_be(&mut self) -> Option<u16> { self.next_u16::<BigEndian>() }

    pub fn peek_u32<B: ByteOrder>(&self) -> Option<u32> { self.peek_bytes::<4>().map(B::read_u32) }
    pub fn peek_u32_ne(&self) -> Option<u32> { self.peek_u32::<NativeEndian>() }
    pub fn peek_u32_le(&self) -> Option<u32> { self.peek_u32::<LittleEndian>() }
    pub fn peek_u32_be(&self) -> Option<u32> { self.peek_u32::<BigEndian>() }
    pub fn next_u32<B: ByteOrder>(&mut self) -> Option<u32> { self.next_bytes::<4>().map(B::read_u32) }
    pub fn next_u32_ne(&mut self) -> Option<u32> { self.next_u32::<NativeEndian>() }
    pub fn next_u32_le(&mut self) -> Option<u32> { self.next_u32::<LittleEndian>() }
    pub fn next_u32_be(&mut self) -> Option<u32> { self.next_u32::<BigEndian>() }

    pub fn peek_u64<B: ByteOrder>(&self) -> Option<u64> { self.peek_bytes::<8>().map(B::read_u64) }
    pub fn peek_u64_ne(&self) -> Option<u64> { self.peek_u64::<NativeEndian>() }
    pub fn peek_u64_le(&self) -> Option<u64> { self.peek_u64::<LittleEndian>() }
    pub fn peek_u64_be(&self) -> Option<u64> { self.peek_u64::<BigEndian>() }
    pub fn next_u64<B: ByteOrder>(&mut self) -> Option<u64> { self.next_bytes::<8>().map(B::read_u64) }
    pub fn next_u64_ne(&mut self) -> Option<u64> { self.next_u64::<NativeEndian>() }
    pub fn next_u64_le(&mut self) -> Option<u64> { self.next_u64::<LittleEndian>() }
    pub fn next_u64_be(&mut self) -> Option<u64> { self.next_u64::<BigEndian>() }

    pub fn peek_i8<B: ByteOrder>(&self) -> Option<i8> { self.peek_bytes::<1>().map(B::read_i8) }
    pub fn peek_i8_ne(&self) -> Option<i8> { self.peek_i8::<NativeEndian>() }
    pub fn peek_i8_le(&self) -> Option<i8> { self.peek_i8::<LittleEndian>() }
    pub fn peek_i8_be(&self) -> Option<i8> { self.peek_i8::<BigEndian>() }
    pub fn next_i8<B: ByteOrder>(&mut self) -> Option<i8> { self.next_bytes::<1>().map(B::read_i8) }
    pub fn next_i8_ne(&mut self) -> Option<i8> { self.next_i8::<NativeEndian>() }
    pub fn next_i8_le(&mut self) -> Option<i8> { self.next_i8::<LittleEndian>() }
    pub fn next_i8_be(&mut self) -> Option<i8> { self.next_i8::<BigEndian>() }

    pub fn peek_i16<B: ByteOrder>(&self) -> Option<i16> { self.peek_bytes::<2>().map(B::read_i16) }
    pub fn peek_i16_ne(&self) -> Option<i16> { self.peek_i16::<NativeEndian>() }
    pub fn peek_i16_le(&self) -> Option<i16> { self.peek_i16::<LittleEndian>() }
    pub fn peek_i16_be(&self) -> Option<i16> { self.peek_i16::<BigEndian>() }
    pub fn next_i16<B: ByteOrder>(&mut self) -> Option<i16> { self.next_bytes::<2>().map(B::read_i16) }
    pub fn next_i16_ne(&mut self) -> Option<i16> { self.next_i16::<NativeEndian>() }
    pub fn next_i16_le(&mut self) -> Option<i16> { self.next_i16::<LittleEndian>() }
    pub fn next_i16_be(&mut self) -> Option<i16> { self.next_i16::<BigEndian>() }

    pub fn peek_i32<B: ByteOrder>(&self) -> Option<i32> { self.peek_bytes::<4>().map(B::read_i32) }
    pub fn peek_i32_ne(&self) -> Option<i32> { self.peek_i32::<NativeEndian>() }
    pub fn peek_i32_le(&self) -> Option<i32> { self.peek_i32::<LittleEndian>() }
    pub fn peek_i32_be(&self) -> Option<i32> { self.peek_i32::<BigEndian>() }
    pub fn next_i32<B: ByteOrder>(&mut self) -> Option<i32> { self.next_bytes::<4>().map(B::read_i32) }
    pub fn next_i32_ne(&mut self) -> Option<i32> { self.next_i32::<NativeEndian>() }
    pub fn next_i32_le(&mut self) -> Option<i32> { self.next_i32::<LittleEndian>() }
    pub fn next_i32_be(&mut self) -> Option<i32> { self.next_i32::<BigEndian>() }

    pub fn peek_i64<B: ByteOrder>(&self) -> Option<i64> { self.peek_bytes::<8>().map(B::read_i64) }
    pub fn peek_i64_ne(&self) -> Option<i64> { self.peek_i64::<NativeEndian>() }
    pub fn peek_i64_le(&self) -> Option<i64> { self.peek_i64::<LittleEndian>() }
    pub fn peek_i64_be(&self) -> Option<i64> { self.peek_i64::<BigEndian>() }
    pub fn next_i64<B: ByteOrder>(&mut self) -> Option<i64> { self.next_bytes::<8>().map(B::read_i64) }
    pub fn next_i64_ne(&mut self) -> Option<i64> { self.next_i64::<NativeEndian>() }
    pub fn next_i64_le(&mut self) -> Option<i64> { self.next_i64::<LittleEndian>() }
    pub fn next_i64_be(&mut self) -> Option<i64> { self.next_i64::<BigEndian>() }

    pub fn peek_f32<B: ByteOrder>(&self) -> Option<f32> { self.peek_bytes::<4>().map(B::read_f32) }
    pub fn peek_f32_ne(&self) -> Option<f32> { self.peek_f32::<NativeEndian>() }
    pub fn peek_f32_le(&self) -> Option<f32> { self.peek_f32::<LittleEndian>() }
    pub fn peek_f32_be(&self) -> Option<f32> { self.peek_f32::<BigEndian>() }
    pub fn next_f32<B: ByteOrder>(&mut self) -> Option<f32> { self.next_bytes::<4>().map(B::read_f32) }
    pub fn next_f32_ne(&mut self) -> Option<f32> { self.next_f32::<NativeEndian>() }
    pub fn next_f32_le(&mut self) -> Option<f32> { self.next_f32::<LittleEndian>() }
    pub fn next_f32_be(&mut self) -> Option<f32> { self.next_f32::<BigEndian>() }

    pub fn peek_f64<B: ByteOrder>(&self) -> Option<f64> { self.peek_bytes::<8>().map(B::read_f64) }
    pub fn peek_f64_ne(&self) -> Option<f64> { self.peek_f64::<NativeEndian>() }
    pub fn peek_f64_le(&self) -> Option<f64> { self.peek_f64::<LittleEndian>() }
    pub fn peek_f64_be(&self) -> Option<f64> { self.peek_f64::<BigEndian>() }
    pub fn next_f64<B: ByteOrder>(&mut self) -> Option<f64> { self.next_bytes::<8>().map(B::read_f64) }
    pub fn next_f64_ne(&mut self) -> Option<f64> { self.next_f64::<NativeEndian>() }
    pub fn next_f64_le(&mut self) -> Option<f64> { self.next_f64::<LittleEndian>() }
    pub fn next_f64_be(&mut self) -> Option<f64> { self.next_f64::<BigEndian>() }
}

