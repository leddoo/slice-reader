types = [
    ("u8", 1), ("u16", 2), ("u32", 4), ("u64", 8),
    ("i8", 1), ("i16", 2), ("i32", 4), ("i64", 8),
    ("f32", 4), ("f64", 8),
]

for (ty, sz) in types:
    print(f"    pub fn peek_{ty}<B: ByteOrder>(&self) -> Option<{ty}> {{ self.peek_bytes::<{sz}>().map(B::read_{ty}) }}")
    print(f"    pub fn peek_{ty}_ne(&self) -> Option<{ty}> {{ self.peek_{ty}::<NativeEndian>() }}")
    print(f"    pub fn peek_{ty}_le(&self) -> Option<{ty}> {{ self.peek_{ty}::<LittleEndian>() }}")
    print(f"    pub fn peek_{ty}_be(&self) -> Option<{ty}> {{ self.peek_{ty}::<BigEndian>() }}")
    print(f"    pub fn next_{ty}<B: ByteOrder>(&mut self) -> Option<{ty}> {{ self.next_bytes::<{sz}>().map(B::read_{ty}) }}")
    print(f"    pub fn next_{ty}_ne(&mut self) -> Option<{ty}> {{ self.next_{ty}::<NativeEndian>() }}")
    print(f"    pub fn next_{ty}_le(&mut self) -> Option<{ty}> {{ self.next_{ty}::<LittleEndian>() }}")
    print(f"    pub fn next_{ty}_be(&mut self) -> Option<{ty}> {{ self.next_{ty}::<BigEndian>() }}")
    print()

