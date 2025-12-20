/// Calculate the UTF-8 byte length of a character as a `u32`.
///
/// This is a convenience wrapper around [`char::len_utf8`] that returns
/// the result as a `u32` instead of `usize`, which is more convenient
/// for byte offset calculations in parsers.
///
/// # Example
///
/// ```
/// use umc_parser::char::len_utf8_u32;
///
/// assert_eq!(len_utf8_u32('a'), 1);
/// assert_eq!(len_utf8_u32('ñ'), 2);
/// assert_eq!(len_utf8_u32('中'), 3);
/// assert_eq!(len_utf8_u32('🦀'), 4);
/// ```
pub fn len_utf8_u32(c: char) -> u32 {
  c.len_utf8() as u32
}
