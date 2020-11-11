#[inline]
pub(crate) fn index2d_to_index(width: u32, x: u32, y: u32) -> usize {
    width as usize * y as usize + x as usize
}

#[inline]
pub(crate) fn index_to_index2d(width: u32, index: usize) -> (u32, u32) {
    let x = index % width as usize;
    let y = (index - x) / width as usize;
    (x as u32, y as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index() {
        let width: u32 = 243;

        let index = index2d_to_index(width, 0, 0);
        assert_eq!(index, 0);
        let index = index_to_index2d(width, index);
        assert_eq!(index, (0, 0));

        let index = index2d_to_index(width, width - 1, 0);
        assert_eq!(index, width as usize - 1);
        let index = index_to_index2d(width, index);
        assert_eq!(index, (width - 1, 0));

        let index = index2d_to_index(width, 0, width - 1);
        assert_eq!(index, (width - 1) as usize * width as usize);
        let index = index_to_index2d(width, index);
        assert_eq!(index, (0, width - 1));

        let index = index2d_to_index(width, width - 1, width - 1);
        assert_eq!(index, (width as usize * width as usize) - 1);
        let index = index_to_index2d(width, index);
        assert_eq!(index, (width - 1, width - 1));
    }
}
