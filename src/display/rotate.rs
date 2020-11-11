use super::math::index2d_to_index;

pub(crate) fn rotate90_square<T>(size: u32, buffer: &mut Vec<T>) {
    for y in 0..size - 2 {
        for x in y + 1..size - 1 {
            let xy = index2d_to_index(size, x, y);
            let yx = index2d_to_index(size, y, x);
            buffer.swap(xy, yx);
        }
    }
}

pub(crate) fn rotate90<T>(width: u32, buffer: &mut Vec<T>) {
    let mut visited = vec![false; buffer.len()];
    let mn1 = buffer.len() - 1;
    let n = buffer.len() / width as usize;

    for i in 1..buffer.len() {
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
    use super::*;

    #[test]
    fn rotate90_square_test() {
        let width: u32 = 5;
        let height: u32 = 5;

        let mut buffer: Vec<i128> = vec![255; width as usize * height as usize];
        let lt = 192;
        let rt = 128;
        let lb = 64;
        let rb = 0;
        let c = 111;
        buffer[index2d_to_index(width, 0, 0)] = lt;
        // buffer[index2d_to_index(width, width - 2, 2)] = rt;
        // buffer[index2d_to_index(width, 0, height - 1)] = lb;
        // buffer[index2d_to_index(width, width - 1, height - 1)] = rb;
        // buffer[index2d_to_index(width, width / 2, height / 2)] = c;

        rotate90_square(width, &mut buffer);
        rotate90_square(width, &mut buffer);

        assert_eq!(
            lt,
            buffer[index2d_to_index(width, width - 1, 0)],
            "{:#?}",
            buffer
        );
    }
}
