fn main() {
    let input = include_str!("../input");
    let (width, height) = (25, 6);
    let layers = get_layers(input, width, height);

    let part1_sol = part1(&layers);
    let part2_sol = part2(&layers, width, height);

    println!("Part 1: {}", part1_sol);
    println!("Part 2:\n{}", part2_sol);
}

fn part2(layers: &[Vec<u32>], width: usize, height: usize) -> String {
    let mut pixels = vec![2u32; width * height];

    for layer in layers.iter().rev() {
        for (dest, source) in pixels.iter_mut().zip(layer) {
            if *source != 2 {
                *dest = *source;
            }
        }
    }

    let mut image = String::new();

    for pixel_line in pixels.chunks(width) {
        for pixel in pixel_line {
            let symbol = match *pixel {
                0 => '\u{25A0}',
                1 => '\u{25A1}',
                2 => '\u{2205}',
                _ => panic!("unknown pixel")
            };

            image.push(symbol);
        }

        image.push('\n');
    }

    image
}

fn part1(layers: &[Vec<u32>]) -> usize {
    let layer = layers
        .iter()
        .min_by_key(|v| v.iter().filter(|v| **v == 0).count())
        .unwrap();

    let ones = layer.iter().filter(|v| **v == 1).count();
    let twos = layer.iter().filter(|v| **v == 2).count();

    ones * twos
}

fn get_layers(input: &str, width: usize, height: usize) -> Vec<Vec<u32>> {
    let data: Vec<u32> = input.chars().map(|v| v.to_digit(10).unwrap()).collect();
    let mut layers = Vec::new();
    for chunk in data.chunks(width * height) {
        layers.push(chunk.to_vec());
    }

    layers
}
