use alloc::vec::Vec;
// use nom::{self, IResult, bytes::complete::take_while, character::{complete::char, is_alphanumeric, is_digit}, sequence::delimited};
// use nom;

#[derive(Debug)]
pub struct Array {
    array: Vec<u8>
}

// fn array(input: &[u8]) -> IResult<&[u8], &[u8]> {
//     let foo = delimited(char('['), take_while(is_digit), char(']'));
//     foo(input)
// }

pub fn parse_array(input: &str) -> Option<Array> {
    // match array(input.as_bytes()) {
    //     Ok((_, nums)) => Some(Array {
    //         array: Vec::from(nums)
    //     }),
    //     Err(_) => None
    // }
    None
}
