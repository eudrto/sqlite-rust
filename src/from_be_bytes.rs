pub trait FromBEBytes {
    fn from_be_bytes(bytes: &mut &[u8]) -> Self;
}

impl<const N: usize> FromBEBytes for [u8; N] {
    fn from_be_bytes(bytes: &mut &[u8]) -> Self {
        let (head, tail) = bytes.split_at(N);
        *bytes = tail;
        head.try_into().unwrap()
    }
}

impl FromBEBytes for u8 {
    fn from_be_bytes(bytes: &mut &[u8]) -> Self {
        u8::from_be_bytes(FromBEBytes::from_be_bytes(bytes))
    }
}

impl FromBEBytes for u16 {
    fn from_be_bytes(bytes: &mut &[u8]) -> Self {
        u16::from_be_bytes(FromBEBytes::from_be_bytes(bytes))
    }
}

impl FromBEBytes for u32 {
    fn from_be_bytes(bytes: &mut &[u8]) -> Self {
        u32::from_be_bytes(FromBEBytes::from_be_bytes(bytes))
    }
}

impl FromBEBytes for u64 {
    fn from_be_bytes(bytes: &mut &[u8]) -> Self {
        u64::from_be_bytes(FromBEBytes::from_be_bytes(bytes))
    }
}

pub fn from_be_bytes<T: FromBEBytes>(bytes: &mut &[u8]) -> T {
    T::from_be_bytes(bytes)
}
