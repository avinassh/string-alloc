use std::mem::size_of;
use std::ptr::NonNull;

static LARGE_STRING_MAX: usize = (1 << 30) - 1;

pub struct StringRS([u8; size_of::<u128>()]);

impl StringRS {
    pub fn len(&self) -> usize {
        let encoded_value = u32::from_be_bytes(self.0[0..4].try_into().unwrap());
        (encoded_value & 0x3FFFFFFF) as usize
    }

    pub fn new(s: &str) -> StringRS {
        debug_assert!(s.len() <= LARGE_STRING_MAX);
        let len = s.len() as u32;
        let mut fs = Self([0; size_of::<u128>()]);

        let encoded_value = 0xC0000000 | (len & 0x3FFFFFFF);
        fs.0[0..4].copy_from_slice(&encoded_value.to_be_bytes());

        let ptr = alloc_string(s);
        fs.0[8..16].copy_from_slice(&(ptr.as_ptr() as usize).to_ne_bytes());

        fs
    }
}

impl From<&str> for StringRS {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

fn alloc_string(s: &str) -> NonNull<u8> {
    let boxed_slice = s.as_bytes().to_owned().into_boxed_slice();
    NonNull::new(Box::into_raw(boxed_slice) as *mut u8).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "really random string";
        let st = String::from(s);
        let fs = StringRS::new(s);
        assert_eq!(st.len(), fs.len())
    }
}
