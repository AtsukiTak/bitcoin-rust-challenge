pub struct u16_l(pub u16);
pub struct u32_l(pub u32);
pub struct u64_l(pub u64);
pub struct u16_b(pub u16);
pub struct u32_b(pub u32);
pub struct u64_b(pub u64);

use EncodableSized;

macro_rules! impl_encodable_sized {
    ($t: ty, $size: expr) => {
        impl EncodableSized for $t {
            const SIZE: usize = $size;

            type Array = [u8; $size];

            fn bytes(&self) -> [u8; $size] {
                unsafe { *(&self.0 as *const _ as *const [u8; $size]) }
            }
        }
    };
}
