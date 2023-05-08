#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(test), no_std)]

use bytes::{Buf, BufMut};

pub fn encode(mut value: u64, mut buf: impl BufMut) {
    loop {
        if value < 0x80 {
            buf.put_u8(value as u8);
            break;
        } else {
            buf.put_u8((value & 0x7f) as u8 | 0x80);
            value >>= 7;
        }
    }
}

pub fn decode(mut buf: impl Buf) -> u64 {
    let mut value = 0;
    for i in 0..10.min(buf.remaining()) {
        let byte = buf.get_u8();
        value |= (byte as u64 & 0x7f) << (i * 7);
        if byte < 0x80 {
            if i == 9 && byte > 0x01 {
                break;
            } else {
                return value;
            }
        }
    }

    panic!("encoded integer doesn't fit in u64");
}

pub fn encoded_len(value: u64) -> usize {
    // Based on [VarintSize64][1].
    // [1]: https://github.com/google/protobuf/blob/3.3.x/src/google/protobuf/io/coded_stream.h#L1301-L1309
    (((value | 0x1).ilog2() * 9 + 73) / 64) as _
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::{prelude::any, prop_assert_eq, proptest};

    #[test]
    fn min() {
        let mut buf = [0u8; 1];
        encode(0, &mut buf[..]);
        assert_eq!(buf, [0u8; 1]);

        let v = decode(&buf[..]);
        assert_eq!(0, v);
    }

    #[test]
    fn max() {
        let mut buf = [0u8; 10];
        encode(u64::MAX, &mut buf[..]);
        let mut expected = [255u8; 10];
        expected[9] = 0x01;
        assert_eq!(buf, expected);

        let v = decode(&buf[..]);
        assert_eq!(u64::MAX, v);
    }

    proptest! {
        #[test]
        fn random(input in any::<u64>()) {
            let mut buf = Vec::new();
            encode(input, &mut buf);
            prop_assert_eq!(encoded_len(input), buf.len());

            let output = decode(&buf[..]);
            prop_assert_eq!(input, output);
        }
    }
}
