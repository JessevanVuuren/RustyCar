pub fn rotate<T: Copy>(array: &[T], size: i32, step: usize) -> Vec<T> {
    let mut rotated = Vec::with_capacity((size * size) as usize);

    for y in 0..size {
        for x in 0..size {
            let i = match step {
                0 => y * size + x,
                1 => (size - 1 - x) * size + y,
                2 => (size - y) * size - 1 - x,
                3 => (size - 1 - y) + size * x,
                _ => panic!("Unreachable step: {}", step),
            };
            rotated.push(array[i as usize].clone());
        }
    }

    rotated
}
