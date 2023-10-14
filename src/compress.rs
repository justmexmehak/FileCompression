pub fn compress_data(data: String) -> String{
    let mut i = 0;

    let mut current_char = "0";
    let mut char_count = 0;

    let mut compressed_data = String::new();

    while i < data.len() {
        if char_count == 0 {
            current_char = & data[i..i+1];
            char_count += 1;
            i += 1;
        }
        else if & data[i..i+1] == current_char {
            char_count += 1;
            i +=  1;
        }
        else {
            let s = format!("{}{}", current_char, char_count);
            compressed_data.push_str(&s);
            char_count = 0;
        }
    }
    let s = format!("{}{}", current_char, char_count);
    compressed_data.push_str(&s);

    compressed_data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress() {
        assert_eq!(compress_data("aaaabb".to_string()), "a4b2".to_string());
    }

    #[test]
    fn test_compress_empty_string() {
        assert_eq!(compress_data("".to_string()), "00".to_string());
    }

    #[test]
    fn test_compress_no_repeat() {
        assert_eq!(compress_data("ab".to_string()), "a1b1".to_string());
    }

    #[test]
    fn test_compress_all_repeat() {
        assert_eq!(compress_data("aaaa".to_string()), "a4".to_string());
    }
}
