#![deny(clippy::all)]
#![forbid(unsafe_code)]
mod engine;
mod pixel;

pub type Color = [u8; 4];

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
