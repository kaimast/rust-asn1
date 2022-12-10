use crate::parser::{ParseError, ParseErrorKind, ParseResult};

pub(crate) fn read_base128_int(mut data: &[u8]) -> ParseResult<(u128, &[u8])> {
    let mut ret = 0u128;
    let mut idx = 0;
    while let Some(b) = data.first() {
        let b = *b;
        data = &data[1..];
        ret <<= 7;
        ret |= u128::from(b & 0x7f);
        // Integers must be minimally encoded. `i == 0 && 0x80` would mean
        // that the first byte had a value of 0, which is non-minimal.
        if idx == 0 && b == 0x80 {
            return Err(ParseError::new(ParseErrorKind::InvalidValue));
        }
        if b & 0x80 == 0 {
            return Ok((ret, data));
        }
        idx += 1;
    }
    Err(ParseError::new(ParseErrorKind::InvalidValue))
}

pub(crate) fn base128_length(mut n: u128) -> usize {
    if n == 0 {
        return 1;
    }

    let mut length = 0;
    while n > 0 {
        length += 1;
        n >>= 7;
    }
    length
}

pub(crate) fn write_base128_int(mut data: &mut [u8], n: u128) -> Option<usize> {
    let length = base128_length(n);

    if data.len() < length {
        return None;
    }

    if n == 0 {
        data[0] = 0;
        return Some(1);
    }

    for i in (0..length).rev() {
        let mut o = (n >> (i * 7)) as u8;
        o &= 0x7f;
        if i != 0 {
            o |= 0x80;
        }
        data[0] = o;
        data = &mut data[1..];
    }

    Some(length)
}
