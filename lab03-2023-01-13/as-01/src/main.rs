
use std::collections::HashSet;     //import คลาส HashSet จาก library ของ Rust
fn count_unique_words(words: &str) -> usize {
    let mut unique_words = HashSet::new(); 
    for word in words.split_whitespace() { // words.split_whitespace() คือ method ของ struct str ใน Rust คือ การแบ่ง string ตามช่องว่าง
        unique_words.insert(word); //เป็นการเพิ่มคำเข้าไปใน HashSet unique_words ซึ่งจะตรวจสอบว่าคำนั้นมีอยู่แล้วหรือไม่
    }
    unique_words.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_unique_words() {
        let result = count_unique_words("this cat this bat this rat");
        assert_eq!(result, 4);
    }
}

