#![allow(dead_code)]

pub mod first;
pub mod second;

pub mod third;
pub mod fourth;
pub mod fifth;

pub mod sixth;


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
