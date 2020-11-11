use super::math::index2d_to_index;

pub(crate) fn transpose_square<T>(size: u32, buffer: &mut [T]) {
    for y in 0..size - 1 {
        for x in y + 1..size {
            let xy = index2d_to_index(size, x, y);
            let yx = index2d_to_index(size, y, x);
            buffer.swap(xy, yx);
        }
    }
}

pub(crate) fn transpose<T>(width: u32, buffer: &mut [T]) {
    let mut visited = vec![false; buffer.len()];
    let mn1 = buffer.len() - 1;
    let n = buffer.len() / width as usize;

    for i in 0..buffer.len() {
        if visited[i] {
            continue;
        }
        let mut a = i;
        loop {
            a = if a == mn1 { mn1 } else { (n * a) % mn1 };
            buffer.swap(a, i);
            visited[a] = true;
            if i == a {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn transpose_square() {
        let mut buffer: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        super::transpose_square(3, &mut buffer);

        assert_eq!(buffer[0], 1);
        assert_eq!(buffer[1], 4);
        assert_eq!(buffer[2], 7);

        assert_eq!(buffer[3], 2);
        assert_eq!(buffer[4], 5);
        assert_eq!(buffer[5], 8);

        assert_eq!(buffer[6], 3);
        assert_eq!(buffer[7], 6);
        assert_eq!(buffer[8], 9);
    }

    #[test]
    fn transpose() {
        let mut buffer: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        super::transpose(2, &mut buffer);

        assert_eq!(buffer[0], 1); // 1 2        1 3 5 7
        assert_eq!(buffer[1], 3); // 3 4  -->   2 4 6 8
        assert_eq!(buffer[2], 5); // 5 6
        assert_eq!(buffer[3], 7); // 7 8

        assert_eq!(buffer[4], 2);
        assert_eq!(buffer[5], 4);
        assert_eq!(buffer[6], 6);
        assert_eq!(buffer[7], 8);
    }
}
