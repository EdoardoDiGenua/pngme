#![allow(unused_variables)]

use crate::Error;
use core::fmt;
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType([u8; 4]);

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        return self.0;
    }

    pub fn is_critical(&self) -> bool {
        let mask = 1 << 5;
        (mask & self.0[0]) == 0
    }

    pub fn is_public(&self) -> bool {
        let mask = 1 << 5;
        (mask & self.0[1]) == 0
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        let mask = 1 << 5;
        (mask & self.0[2]) == 0
    }

    pub fn is_safe_to_copy(&self) -> bool {
        let mask = 1 << 5;
        (mask & self.0[3]) != 0
    }

    pub fn is_valid(&self) -> bool {
        self.0.iter().all(|&el| ChunkType::is_valid_byte(el))
            & ChunkType::is_reserved_bit_valid(&self)
    }

    pub fn is_valid_byte(byte: u8) -> bool {
        (byte >= b'A' && byte <= b'Z') || (byte >= b'a' && byte <= b'z')
    }
}

#[derive(Debug)]
pub struct InvalidChunkTypeError;

impl Display for InvalidChunkTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ChunkType must be 4 ASCII alphabetic characters")
    }
}

impl std::error::Error for InvalidChunkTypeError {}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let is_valid = value.iter().all(|&byte| ChunkType::is_valid_byte(byte));
        if is_valid {
            Ok(ChunkType(value))
        } else {
            Err(Box::new(InvalidChunkTypeError))
        }
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let binding = self.bytes();
        let t = String::from_utf8_lossy(&binding);
        write!(f, "{}", t)
    }
}

#[derive(Debug)]
struct ParseChunkTypeError;

impl Display for ParseChunkTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Can't parse string to chunk type, string must contain 4 characters"
        )
    }
}

impl std::error::Error for ParseChunkTypeError {}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        if bytes.len() == 4 {
            let bytes_fixed_length: [u8; 4] = bytes.try_into()?;
            ChunkType::try_from(bytes_fixed_length)
        } else {
            Err(Box::new(ParseChunkTypeError))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
