// original hand-coded version
const fn make_whitespace() -> [u8; 256] {
    let mut map: [u8; 256] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const ZERO_16: [u8; 8] = [0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x20, 0x85, 0xa0];
    let mut ix = 0;
    while ix < ZERO_16.len() {
        map[ZERO_16[ix] as usize] = 1 << 0;
        ix += 1;
    }
    const TWO_16: [u8; 15] = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x28, 0x29, 0x2f, 0x5f];
    ix = 0;
    while ix < TWO_16.len() {
        map[TWO_16[ix] as usize] |= 1 << 2;
        ix += 1;
    }

    map
}

const WHITESPACE_MAP: [u8; 256] = make_whitespace();

#[inline]
pub const fn mapped_if(c: char) -> bool {
    let high_3_bits = c as u32 >> 8;
    if high_3_bits == 0x00 {
        WHITESPACE_MAP[c as usize & 0xff] & (1 << 0) != 0
    } else if high_3_bits < 0x20 {
        c == '\u{1680}'
    } else if high_3_bits < 0x30 {
        WHITESPACE_MAP[c as usize & 0xff] & (1 << 2) != 0
    } else {
        c == '\u{3000}'
    }
    //match c as u32 >> 8 {
    //    0x00 => WHITESPACE_MAP[c as usize & 0xff] & (1 << 0) != 0,
    //    0x16 => c == '\u{1680}',
    //    0x20 => WHITESPACE_MAP[c as usize & 0xff] & (1 << 2) != 0,
    //    0x30 => c == '\u{3000}',
    //    _ => false,
    //}
}

#[inline]
pub const fn match_ws(c: char) -> bool {
    match c {
        '\u{9}'..='\u{d}'|
        '\u{20}'|
        '\u{85}'|
        '\u{a0}'|
        '\u{1680}'|
        '\u{2000}'..='\u{200a}'|
        '\u{2028}'|'\u{2029}'|
        '\u{202f}'|
        '\u{205f}'|
        '\u{3000}'
            => true,
        _ => false,
    }
}

// generated version using unicode-table-generator (copy pasted)
#[rustfmt::skip]
pub mod white_space {
    static WHITESPACE_MAP: [u8; 256] = [
        2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    #[inline]
    pub fn lookup(c: char) -> bool {
        //let high_bits = c as u32 >> 8;
        //if high_bits == 0 {
        //    WHITESPACE_MAP[c as usize & 0xff] & 1 != 0
        //} else if high_bits == 22 {
        //    c as u32 == 0x1680
        //} else if high_bits == 32 {
        //    WHITESPACE_MAP[c as usize & 0xff] & 2 != 0
        //} else {
        //    false
        //}
        match c as u32 >> 8 {
            0 => WHITESPACE_MAP[c as usize & 0xff] & 1 != 0,
            22 => c as u32 == 0x1680,
            32 => WHITESPACE_MAP[c as usize & 0xff] & 2 != 0,
            48 => c as u32 == 0x3000,
            _ => false,
        }
    }
}
