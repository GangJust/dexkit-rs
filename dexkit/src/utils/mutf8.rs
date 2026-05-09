use crate::errors::Error;

pub struct MUtf8;

impl MUtf8 {
    /// Decodes bytes from the slice until a delimiter 0x00 is encountered.
    /// Returns a new string containing the decoded characters.
    /// # Arguments
    /// * `bytes` - A byte slice containing the MUTF-8 encoded data.
    /// # Returns
    /// * `Ok(String)` - The decoded string if successful.
    /// * `Err(Error)` - If an error occurs during decoding.
    pub fn decode(bytes: &[u8]) -> Result<String, Error> {
        let mut result = String::new();
        let mut i = 0;

        while i < bytes.len() {
            let a = bytes[i] as u8;

            // if encounter 0x00, stop decoding
            if a == 0 {
                return Ok(result);
            }

            if a < 0x80 {
                // if single byte character
                result.push(a as char);
                i += 1;
            } else if (a & 0xe0) == 0xc0 {
                // if two byte character
                if i + 1 >= bytes.len() {
                    return Err(Error::MUtf8DecodeError("bad second byte".to_string()));
                }

                let b = bytes[i + 1] as u8;
                if (b & 0xc0) != 0x80 {
                    return Err(Error::MUtf8DecodeError("bad second byte".to_string()));
                }

                let code_point = ((a & 0x1f) as u16) << 6 | ((b & 0x3f) as u16);
                if let Some(ch) = char::from_u32(code_point as u32) {
                    result.push(ch);
                } else {
                    return Err(Error::MUtf8DecodeError("bad byte".to_string()));
                }
                i += 2;
            } else if (a & 0xf0) == 0xe0 {
                // if three byte character
                if i + 2 >= bytes.len() {
                    return Err(Error::MUtf8DecodeError(
                        "bad second or third byte".to_string(),
                    ));
                }

                let b = bytes[i + 1] as u8;
                let c = bytes[i + 2] as u8;

                if (b & 0xc0) != 0x80 || (c & 0xc0) != 0x80 {
                    return Err(Error::MUtf8DecodeError(
                        "bad second or third byte".to_string(),
                    ));
                }

                let code_point =
                    ((a & 0x0f) as u16) << 12 | ((b & 0x3f) as u16) << 6 | ((c & 0x3f) as u16);
                if let Some(ch) = char::from_u32(code_point as u32) {
                    result.push(ch);
                } else {
                    return Err(Error::MUtf8DecodeError("bad byte".to_string()));
                }
                i += 3;
            } else {
                return Err(Error::MUtf8DecodeError("bad byte".to_string()));
            }
        }

        // if reach here, means no delimiter found, return the whole decoded string
        Ok(result)
    }

    /// Returns the number of bytes the modified UTF8 representation of string would take.
    /// # Arguments
    /// * `s` - The string to count bytes for.
    /// * `short_length` - Whether to enforce 16-bit length limit (65535 bytes).
    /// # Returns
    /// * `Ok(u64)` - The number of bytes required.
    /// * `Err(Error)` - If string is too long and short_length is true.
    fn count_bytes(s: &str, short_length: bool) -> Result<u64, Error> {
        let mut result = 0u64;

        for ch in s.chars() {
            let code = ch as u32;
            if code != 0 && code <= 127 {
                // U+0000 uses two bytes in MUTF-8
                result += 1;
            } else if code <= 2047 {
                result += 2;
            } else {
                result += 3;
            }

            if short_length && result > 65535 {
                return Err(Error::MUtf8DecodeError(
                    "String more than 65535 UTF bytes long".to_string(),
                ));
            }
        }

        Ok(result)
    }

    /// Encodes the modified UTF-8 bytes corresponding to string into destination buffer.
    /// # Arguments
    /// * `dst` - The destination buffer to write encoded bytes.
    /// * `offset` - The starting offset in the destination buffer.
    /// * `s` - The string to encode.
    /// # Returns
    /// * The number of bytes written.
    pub fn encode_to_buffer(dst: &mut [u8], offset: usize, s: &str) -> usize {
        let mut current_offset = offset;

        for ch in s.chars() {
            let code = ch as u32;

            if code != 0 && code <= 127 {
                // Single byte (ASCII, but U+0000 uses two bytes in MUTF-8)
                dst[current_offset] = code as u8;
                current_offset += 1;
            } else if code <= 2047 {
                // Two bytes
                dst[current_offset] = (0xc0 | (0x1f & (code >> 6))) as u8;
                dst[current_offset + 1] = (0x80 | (0x3f & code)) as u8;
                current_offset += 2;
            } else {
                // Three bytes
                dst[current_offset] = (0xe0 | (0x0f & (code >> 12))) as u8;
                dst[current_offset + 1] = (0x80 | (0x3f & (code >> 6))) as u8;
                dst[current_offset + 2] = (0x80 | (0x3f & code)) as u8;
                current_offset += 3;
            }
        }

        current_offset - offset
    }

    /// Returns a vector containing the modified UTF-8 form of the string.
    /// # Arguments
    /// * `s` - The string to encode.
    /// # Returns
    /// * `Ok(Vec<u8>)` - The encoded bytes if successful.
    /// * `Err(Error)` - If the string is too long (> 65535 bytes).
    pub fn encode(s: &str) -> Result<Vec<u8>, Error> {
        let utf_count = Self::count_bytes(s, true)? as usize;
        let mut result = vec![0u8; utf_count];
        Self::encode_to_buffer(&mut result, 0, s);
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_ascii() {
        let bytes = b"Hello\0";
        let result = MUtf8::decode(bytes).unwrap();
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_decode_with_null_terminator() {
        let bytes = b"Hello\0World";
        let result = MUtf8::decode(bytes).unwrap();
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_decode_two_byte_sequence() {
        // UTF-8 for "ñ" (U+00F1)
        let bytes = [0xc3, 0xb1, 0x00];
        let result = MUtf8::decode(&bytes).unwrap();
        assert_eq!(result, "ñ");
    }

    #[test]
    fn test_decode_three_byte_sequence() {
        // UTF-8 for "中" (U+4E2D)
        let bytes = [0xe4, 0xb8, 0xad, 0x00];
        let result = MUtf8::decode(&bytes).unwrap();
        assert_eq!(result, "中");
    }

    #[test]
    fn test_decode_bad_second_byte() {
        let bytes = [0xc3, 0x30]; // Invalid second byte
        let result = MUtf8::decode(&bytes);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("bad second byte"));
    }

    #[test]
    fn test_encode_ascii() {
        let result = MUtf8::encode("Hello").unwrap();
        assert_eq!(result, b"Hello");
    }

    #[test]
    fn test_encode_two_byte() {
        let result = MUtf8::encode("ñ").unwrap();
        assert_eq!(result, [0xc3, 0xb1]);
    }

    #[test]
    fn test_encode_three_byte() {
        let result = MUtf8::encode("中").unwrap();
        assert_eq!(result, [0xe4, 0xb8, 0xad]);
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let original = "Hello, 世界! ñoël";
        let encoded = MUtf8::encode(original).unwrap();
        let mut encoded_with_null = encoded.clone();
        encoded_with_null.push(0); // Add null terminator
        let decoded = MUtf8::decode(&encoded_with_null).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_encode_to_buffer() {
        let mut buffer = [0u8; 10];
        let bytes_written = MUtf8::encode_to_buffer(&mut buffer, 2, "Hi");
        assert_eq!(bytes_written, 2);
        assert_eq!(&buffer[2..4], b"Hi");
        assert_eq!(&buffer[0..2], &[0, 0]); // untouched
        assert_eq!(&buffer[4..], &[0, 0, 0, 0, 0, 0]); // untouched
    }
}
