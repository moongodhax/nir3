use wasm_bindgen::prelude::*;
use std::vec::Vec;
use std::convert::TryInto;
use std::num::Wrapping;

fn rotl32(value: u64, count: u64) -> u64 {
    (value << count) | (value >> (32 - count))
}

fn tr_f(x: u64, y: u64, z: u64) -> u64 {
    (x & y) | ( ((!x) & 0xffffffff) & z)
}

fn tr_g(x: u64, y: u64, z: u64) -> u64 {
    (x & z) | (y & ((!z) & 0xffffffff))
}

fn tr_h(x: u64, y: u64, z: u64) -> u64 {
    x ^ y ^ z
}

fn tr_i(x: u64, y: u64, z: u64) -> u64 {
    y ^ (x | ((!z) & 0xffffffff))
}

fn transform(
    func: impl Fn(u64, u64, u64) -> u64,
	a: & u64,
    b: & u64,
    c: & u64,
	d: & u64,
    word: u32,
    k: u64,
    s: u8
) -> u64 {
    let f: u64 = *a + func(*b, *c, *d);

    let mut qq: u64 = f as u64 + word as u64;

    qq = qq + k as u64;
    qq = qq & 0xffffffff;
    qq = rotl32(qq, s.into());
    qq = qq & 0xffffffff;
    qq = qq + *b;
    qq & 0xffffffff
}

fn bytes_to_words(message: Vec<u8>) -> Vec<u32> {
    let mut words: Vec<u32> = vec![];

    for i in 0..(message.len() / 4) {
        let start: usize = i*4;
        let word: u32 = u32::from_le_bytes(message[start..start+4].try_into().expect("Срез должен содержать 4 элемента"));
        words.push(word);
    }

    words
}

#[wasm_bindgen]
pub fn md5(bytes: &[u8]) -> String {
    let mut message: Vec<u8> = bytes.to_vec();

    let mut a: u64 = 0x67452301;
    let mut b: u64 = 0xefcdab89;
    let mut c: u64 = 0x98badcfe;
    let mut d: u64 = 0x10325476;

    let size = 64 - (message.len() as i32 % 64);
    let padding_to_add: usize;

    if (size - 8) < 0 {
        padding_to_add = size as usize + (64 - 8);
    } else {
        padding_to_add = size as usize - 8;
    }

    let mut padbuf = vec![0; padding_to_add];
    padbuf[0] = 0x80;

    let bitsize = Wrapping(message.len() as u64 * 8);
    let orig_size: [u8; 8] = bitsize.0.to_le_bytes();
    let mut orig_size = orig_size.to_vec();
    padbuf.append(&mut orig_size);
    message.append(&mut padbuf);

    let words = bytes_to_words(message);

    for i in (0..words.len()).step_by(16) {
        let mut a0 = a;
        let mut b0 = b;
        let mut c0 = c;
        let mut d0 = d;

        //  round 1, apply on all 16 words
        a0 = transform(tr_f, &a0, &b0, &c0, &d0, words[i+0], 0xD76AA478, 7);
        d0 = transform(tr_f, &d0, &a0, &b0, &c0, words[i+1], 0xE8C7B756, 12);
        c0 = transform(tr_f, &c0, &d0, &a0, &b0, words[i+2], 0x242070DB, 17);
        b0 = transform(tr_f, &b0, &c0, &d0, &a0, words[i+3], 0xC1BDCEEE, 22);

        a0 = transform(tr_f, &a0, &b0, &c0, &d0, words[i+4], 0xF57C0FAF, 7);
        d0 = transform(tr_f, &d0, &a0, &b0, &c0, words[i+5], 0x4787C62A, 12);
        c0 = transform(tr_f, &c0, &d0, &a0, &b0, words[i+6], 0xA8304613, 17);
        b0 = transform(tr_f, &b0, &c0, &d0, &a0, words[i+7], 0xFD469501, 22);

        a0 = transform(tr_f, &a0, &b0, &c0, &d0, words[i+8], 0x698098D8, 7);
        d0 = transform(tr_f, &d0, &a0, &b0, &c0, words[i+9], 0x8B44F7AF, 12);
        c0 = transform(tr_f, &c0, &d0, &a0, &b0, words[i+10], 0xFFFF5BB1, 17);
        b0 = transform(tr_f, &b0, &c0, &d0, &a0, words[i+11], 0x895CD7BE, 22);

        a0 = transform(tr_f, &a0, &b0, &c0, &d0, words[i+12], 0x6B901122, 7);
        d0 = transform(tr_f, &d0, &a0, &b0, &c0, words[i+13], 0xFD987193, 12);
        c0 = transform(tr_f, &c0, &d0, &a0, &b0, words[i+14], 0xA679438E, 17);
        b0 = transform(tr_f, &b0, &c0, &d0, &a0, words[i+15], 0x49B40821, 22);

        // round 2, apply on all 16 words
        a0 = transform(tr_g, &a0, &b0, &c0, &d0, words[i+1], 0xF61E2562, 5);
        d0 = transform(tr_g, &d0, &a0, &b0, &c0, words[i+6], 0xC040B340, 9);
        c0 = transform(tr_g, &c0, &d0, &a0, &b0, words[i+11], 0x265E5A51, 14);
        b0 = transform(tr_g, &b0, &c0, &d0, &a0, words[i+0], 0xE9B6C7AA, 20);

        a0 = transform(tr_g, &a0, &b0, &c0, &d0, words[i+5], 0xD62F105D, 5);
        d0 = transform(tr_g, &d0, &a0, &b0, &c0, words[i+10], 0x02441453, 9);
        c0 = transform(tr_g, &c0, &d0, &a0, &b0, words[i+15], 0xD8A1E681, 14);
        b0 = transform(tr_g, &b0, &c0, &d0, &a0, words[i+4], 0xE7D3FBC8, 20);

        a0 = transform(tr_g, &a0, &b0, &c0, &d0, words[i+9], 0x21E1CDE6, 5);
        d0 = transform(tr_g, &d0, &a0, &b0, &c0, words[i+14], 0xC33707D6, 9);
        c0 = transform(tr_g, &c0, &d0, &a0, &b0, words[i+3], 0xF4D50D87, 14);
        b0 = transform(tr_g, &b0, &c0, &d0, &a0, words[i+8], 0x455A14ED, 20);

        a0 = transform(tr_g, &a0, &b0, &c0, &d0, words[i+13], 0xA9E3E905, 5);
        d0 = transform(tr_g, &d0, &a0, &b0, &c0, words[i+2], 0xFCEFA3F8, 9);
        c0 = transform(tr_g, &c0, &d0, &a0, &b0, words[i+7], 0x676F02D9, 14);
        b0 = transform(tr_g, &b0, &c0, &d0, &a0, words[i+12], 0x8D2A4C8A, 20);

        // round 3, apply on all 16 words
        a0 = transform(tr_h, &a0, &b0, &c0, &d0, words[i+5], 0xFFFA3942, 4);
        d0 = transform(tr_h, &d0, &a0, &b0, &c0, words[i+8], 0x8771F681, 11);
        c0 = transform(tr_h, &c0, &d0, &a0, &b0, words[i+11], 0x6D9D6122, 16);
        b0 = transform(tr_h, &b0, &c0, &d0, &a0, words[i+14], 0xFDE5380C, 23);

        a0 = transform(tr_h, &a0, &b0, &c0, &d0, words[i+1], 0xA4BEEA44, 4);
        d0 = transform(tr_h, &d0, &a0, &b0, &c0, words[i+4], 0x4BDECFA9, 11);
        c0 = transform(tr_h, &c0, &d0, &a0, &b0, words[i+7], 0xF6BB4B60, 16);
        b0 = transform(tr_h, &b0, &c0, &d0, &a0, words[i+10], 0xBEBFBC70, 23);

        a0 = transform(tr_h, &a0, &b0, &c0, &d0, words[i+13], 0x289B7EC6, 4);
        d0 = transform(tr_h, &d0, &a0, &b0, &c0, words[i+0], 0xEAA127FA, 11);
        c0 = transform(tr_h, &c0, &d0, &a0, &b0, words[i+3], 0xD4EF3085, 16);
        b0 = transform(tr_h, &b0, &c0, &d0, &a0, words[i+6], 0x04881D05, 23);

        a0 = transform(tr_h, &a0, &b0, &c0, &d0, words[i+9], 0xD9D4D039, 4);
        d0 = transform(tr_h, &d0, &a0, &b0, &c0, words[i+12], 0xE6DB99E5, 11);
        c0 = transform(tr_h, &c0, &d0, &a0, &b0, words[i+15], 0x1FA27CF8, 16);
        b0 = transform(tr_h, &b0, &c0, &d0, &a0, words[i+2], 0xC4AC5665, 23);

        // round 4, apply on all 16 words
        a0 = transform(tr_i, &a0, &b0, &c0, &d0, words[i+0], 0xF4292244, 6);
        d0 = transform(tr_i, &d0, &a0, &b0, &c0, words[i+7], 0x432AFF97, 10);
        c0 = transform(tr_i, &c0, &d0, &a0, &b0, words[i+14], 0xAB9423A7, 15);
        b0 = transform(tr_i, &b0, &c0, &d0, &a0, words[i+5], 0xFC93A039, 21);

        a0 = transform(tr_i, &a0, &b0, &c0, &d0, words[i+12], 0x655B59C3, 6);
        d0 = transform(tr_i, &d0, &a0, &b0, &c0, words[i+3], 0x8F0CCC92, 10);
        c0 = transform(tr_i, &c0, &d0, &a0, &b0, words[i+10], 0xFFEFF47D, 15);
        b0 = transform(tr_i, &b0, &c0, &d0, &a0, words[i+1], 0x85845DD1, 21);

        a0 = transform(tr_i, &a0, &b0, &c0, &d0, words[i+8], 0x6FA87E4F, 6);
        d0 = transform(tr_i, &d0, &a0, &b0, &c0, words[i+15], 0xFE2CE6E0, 10);
        c0 = transform(tr_i, &c0, &d0, &a0, &b0, words[i+6], 0xA3014314, 15);
        b0 = transform(tr_i, &b0, &c0, &d0, &a0, words[i+13], 0x4E0811A1, 21);

        a0 = transform(tr_i, &a0, &b0, &c0, &d0, words[i+4], 0xF7537E82, 6);
        d0 = transform(tr_i, &d0, &a0, &b0, &c0, words[i+11], 0xBD3AF235, 10);
        c0 = transform(tr_i, &c0, &d0, &a0, &b0, words[i+2], 0x2AD7D2BB, 15);
        b0 = transform(tr_i, &b0, &c0, &d0, &a0, words[i+9], 0xEB86D391, 21);

        // add to results
        a = (a + a0) & 0xffffffff;
        b = (b + b0) & 0xffffffff;
        c = (c + c0) & 0xffffffff;
        d = (d + d0) & 0xffffffff;
    }

    let bytes1: [u8; 4] = (a as u32).to_le_bytes();
    let bytes2: [u8; 4] = (b as u32).to_le_bytes();
    let bytes3: [u8; 4] = (c as u32).to_le_bytes();
    let bytes4: [u8; 4] = (d as u32).to_le_bytes();

    let message = format!(
        "{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes1[0], bytes1[1], bytes1[2], bytes1[3], bytes2[0], bytes2[1], bytes2[2], bytes2[3],
        bytes3[0], bytes3[1], bytes3[2], bytes3[3], bytes4[0], bytes4[1], bytes4[2], bytes4[3]
    );

    message
}
