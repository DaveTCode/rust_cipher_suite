const LETTER_FREQUENCY: [f32; 26] = [
    12.02, 9.10, 8.12, 7.68, 7.31, 6.95, 6.28, 6.02, 5.92, 4.32, 3.98, 2.88, 2.71, 2.61, 2.30,
    2.11, 2.09, 2.03, 1.82, 1.49, 1.11, 0.69, 0.17, 0.11, 0.10, 0.07,
];

fn score_offset(offset: u8, cipher_text: &str) -> f32 {
    cipher_text
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| {
            let ascii_value = c.to_ascii_uppercase() as i32 - 'A' as i32;
            let offset_value = (ascii_value + i32::from(offset)) % 26;

            LETTER_FREQUENCY[offset_value as usize]
        })
        .fold(0f32, |acc, x| acc + x)
}

fn decrypt(offset: u8, cipher_text: &str) -> String {
    cipher_text
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let ascii_value = c.to_ascii_uppercase() as i32 - 'A' as i32;
                let offset_value = (ascii_value + i32::from(offset)) % 26;
                ('A' as i32 + offset_value) as u8 as char
            } else {
                c
            }
        })
        .collect()
}

pub fn solve(cipher_text: &str) -> (u8, String) {
    let mut top_score = 0f32;
    let mut top_offset = 0u8;

    for offset in 0u8..26 {
        let score = score_offset(offset, cipher_text);

        if score > top_score {
            top_score = score;
            top_offset = offset;
        }
    }

    (top_offset, decrypt(top_offset, cipher_text))
}
