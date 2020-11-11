use super::math::index2d_to_index;

pub(crate) fn rotate90_square<T>(size: u32, buffer: &mut Vec<T>) {
    for y in 0..size - 1 {
        for x in y + 1..size {
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
