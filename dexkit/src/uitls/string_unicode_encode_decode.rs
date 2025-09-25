/// This struct provides Unicode conversion utility methods that allow to convert
/// a string into Unicode sequence and vice-versa.
/// 
/// Port of Michael Gantman's Java StringUnicodeEncoderDecoder class.
pub struct StringUnicodeEncoderDecoder;

impl StringUnicodeEncoderDecoder {
    const UNICODE_PREFIX: &'static str = "\\u";
    const UPPER_CASE_UNICODE_PREFIX: &'static str = "\\U";
    const DELIMITER: &'static str = "\\\\u";

    /// This method converts a String of characters in any language into a String
    /// that contains a sequence of Unicode codes corresponding to characters in the original String.
    /// For Example String "Hello" will be converted into a String 
    /// "\u005c\u00750048\u005c\u00750065\u005c\u0075006c\u005c\u0075006c\u005c\u0075006f"
    /// Empty String conversion will return an empty String.
    /// 
    /// # Arguments
    /// 
    /// * `txt` - String that contains a sequence of characters to convert
    /// 
    /// # Returns
    /// 
    /// String that contains a sequence of unicode codes corresponding to the characters
    /// in the original String. Each code will be in hexadecimal format preceded by
    /// prefix "\u005c\u0075" with no spaces between them.
    pub fn encode_string_to_unicode_sequence(txt: &str) -> String {
        let mut result = String::new();
        
        if !txt.is_empty() {
            let chars: Vec<char> = txt.chars().collect();
            let mut i = 0;
            
            while i < chars.len() {
                let ch = chars[i];
                let code_point = ch as u32;
                result.push_str(&Self::convert_code_point_to_unicode_string(code_point));
                
                // Handle surrogate pairs (similar to Java's isHighSurrogate logic)
                if Self::is_high_surrogate(ch) && i + 1 < chars.len() {
                    i += 1;
                }
                i += 1;
            }
        }
        
        result
    }

    /// This method converts String that contains a sequence of Unicode codes 
    /// onto a String of corresponding characters. For example a String
    /// "\u0048\u0065\u006c\u006c\u006f" 
    /// will be converted into String "Hello" by this method.
    /// 
    /// # Arguments
    /// 
    /// * `unicode_sequence` - String that contains sequence of Unicode codes.
    ///   Each code must be in hexadecimal format and must be preceded by
    ///   "'backslash' + 'u'" prefix. This method allows leading and trailing
    ///   whitespaces for the whole String as well as spaces between codes.
    /// 
    /// # Returns
    /// 
    /// String that contains sequence of characters that correspond to the
    /// respective Unicode codes in the original String
    /// 
    /// # Errors
    /// 
    /// Returns an error if input String is in invalid format, for example
    /// if any code is not in hexadecimal format or the code is not a valid
    /// Unicode code (not valid code point).
    pub fn decode_unicode_sequence_to_string(unicode_sequence: &str) -> Result<String, String> {
        let mut result = String::new();
        
        if unicode_sequence.is_empty() {
            return Ok(result);
        }
        
        let processed_sequence = Self::replace_upper_case_u_with_lower_case(unicode_sequence);
        let trimmed = processed_sequence.trim();
        
        if !trimmed.starts_with(Self::UNICODE_PREFIX) {
            return Err("Unicode sequence must start with \\u prefix".to_string());
        }
        
        // Split by \u and process each part
        let parts: Vec<&str> = trimmed.split("\\u").collect();
        
        // Skip the first empty part (before the first \u)
        for code_point_str in parts.iter().skip(1) {
            let trimmed_code = code_point_str.trim();
            if trimmed_code.is_empty() {
                continue;
            }
            
            match u32::from_str_radix(trimmed_code, 16) {
                Ok(code_point) => {
                    match char::from_u32(code_point) {
                        Some(ch) => result.push(ch),
                        None => return Err(format!("Invalid Unicode code point: {}", code_point)),
                    }
                },
                Err(_) => return Err(format!("Invalid hexadecimal format: {}", trimmed_code)),
            }
        }
        
        Ok(result)
    }

    /// Replaces upper case 'U' prefix with lower case 'u' prefix
    fn replace_upper_case_u_with_lower_case(unicode_sequence: &str) -> String {
        if unicode_sequence.contains(Self::UPPER_CASE_UNICODE_PREFIX) {
            unicode_sequence.replace(Self::UPPER_CASE_UNICODE_PREFIX, Self::UNICODE_PREFIX)
        } else {
            unicode_sequence.to_string()
        }
    }

    /// This method converts an integer that holds a unicode code value into a String
    /// 
    /// # Arguments
    /// 
    /// * `code_point` - a unicode code value
    /// 
    /// # Returns
    /// 
    /// String that starts with prefix "'backslash' + 'u'" that follows with
    /// hexadecimal value of an integer. If the hexadecimal value of an integer
    /// is less then four digits the value is padded with preceding zeros.
    /// For example if the integer has value 32 (decimal) it will be converted
    /// into String "\u0020"
    fn convert_code_point_to_unicode_string(code_point: u32) -> String {
        let mut result = String::from(Self::UNICODE_PREFIX);
        let mut code_point_hex_str = format!("{:x}", code_point);
        
        // Remove leading zero if present (similar to Java logic)
        if code_point_hex_str.starts_with('0') && code_point_hex_str.len() > 1 {
            code_point_hex_str = code_point_hex_str[1..].to_string();
        }
        
        if code_point_hex_str.len() <= 4 {
            result.push_str(&Self::get_preceding_zeros_str(code_point_hex_str.len()));
        }
        
        result.push_str(&code_point_hex_str);
        result
    }

    /// This method receives a length of a String and if it is less then 4 it generates
    /// a padding String of zeros that can be appended to the String to make it of length 4.
    /// I.e. if parameter passed is 1 the returned String will be "000".
    /// If the parameter passed is 4 or greater empty String is returned.
    /// 
    /// # Arguments
    /// 
    /// * `code_point_str_length` - Length of a String to be padded by preceding zeros to the length of 4
    /// 
    /// # Returns
    /// 
    /// padding String
    fn get_preceding_zeros_str(code_point_str_length: usize) -> String {
        if code_point_str_length >= 4 {
            String::new()
        } else {
            "0".repeat(4 - code_point_str_length)
        }
    }

    /// Check if a character is a high surrogate (similar to Java's Character.isHighSurrogate)
    fn is_high_surrogate(ch: char) -> bool {
        let code = ch as u32;
        code >= 0xD800 && code <= 0xDBFF
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_string_to_unicode_sequence() {
        let result = StringUnicodeEncoderDecoder::encode_string_to_unicode_sequence("Hello");
        println!("Encoded 'Hello': {}", result);
        
        let empty_result = StringUnicodeEncoderDecoder::encode_string_to_unicode_sequence("");
        assert_eq!(empty_result, "");
    }

    #[test]
    fn test_decode_unicode_sequence_to_string() {
        // Test basic decoding
        let unicode_seq = "\\u0048\\u0065\\u006c\\u006c\\u006f";
        let result = StringUnicodeEncoderDecoder::decode_unicode_sequence_to_string(unicode_seq);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello");
        
        // Test empty string
        let empty_result = StringUnicodeEncoderDecoder::decode_unicode_sequence_to_string("");
        assert!(empty_result.is_ok());
        assert_eq!(empty_result.unwrap(), "");
    }

    #[test]
    fn test_roundtrip_conversion() {
        let original = "Hello, 世界! 🌍";
        let encoded = StringUnicodeEncoderDecoder::encode_string_to_unicode_sequence(original);
        let decoded = StringUnicodeEncoderDecoder::decode_unicode_sequence_to_string(&encoded);
        
        assert!(decoded.is_ok());
        assert_eq!(decoded.unwrap(), original);
    }

    #[test]
    fn test_error_handling() {
        // Test invalid format
        let invalid_seq = "invalid";
        let result = StringUnicodeEncoderDecoder::decode_unicode_sequence_to_string(invalid_seq);
        assert!(result.is_err());
        
        // Test invalid hex
        let invalid_hex = "\\uGGGG";
        let result2 = StringUnicodeEncoderDecoder::decode_unicode_sequence_to_string(invalid_hex);
        assert!(result2.is_err());
    }

    #[test]
    fn test_upper_case_u_replacement() {
        let unicode_seq = "\\U0048\\U0065\\U006c\\U006c\\U006f";
        let result = StringUnicodeEncoderDecoder::decode_unicode_sequence_to_string(unicode_seq);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello");
    }
}