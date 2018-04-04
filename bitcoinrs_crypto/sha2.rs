pub fn padding(data: &mut Vec<u8>) {
    let data_size = data.len();
    let pad_size = size_need_padding(data.len());
    data.reserve(pad_size);
    data.push(1);
    data.set_len(pad_size);

    let l = data.len();
    let padded_len = l + ((512 - l)
    data.push(1);
    data.reserve(k);
    data.set_len(l + 1 + k);
    let k = if l < 447 { 447 - l } else { (l - 447) % 512 };
    unsafe {
        write_bytes(vec.as_mut_ptr(), 0, k);
    }
}

fn size_need_padding(l: usize) -> usize {
    let zero_padding = if l < 447 { 447 - l } else { (l - 447) % 512 };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn padding_must_be_multiple_of_512() {
        let mut vec = Vec::new();
        vec.extend_from_slice("abcde".as_bytes());
        padding(&mut vec);
        assert_eq!(vec.len(), 512);
    }
}
