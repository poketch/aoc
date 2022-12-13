use std::cmp::PartialEq;
use std::ops::BitXor;
use std::ops::Shl;


#[derive(Debug)]
pub enum ConversionError {
    Overflow,
    NonBinaryInput,
}

pub fn bits_to_num<T: PartialEq + From<u8> + BitXor<Output=T> + Shl<Output=T> + Clone>(
    bits: &[u8],
) -> Result<T, ConversionError> {
    if bits.len() > (std::mem::size_of::<T>() * 8) {
        return Err(ConversionError::Overflow);
    }
    if bits.iter()
        .filter(|&&bit| bit != 0 && bit != 1).count() > 0 {
        return Err(ConversionError::NonBinaryInput);
    }

    Ok(bits.iter()
        .fold(T::from(0), |result, &bit| {
            (result << T::from(1)) ^ T::from(bit)
        }))
}