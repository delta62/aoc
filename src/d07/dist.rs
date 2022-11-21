pub type Position = i32;

pub fn sum_of_distances(origin: Position, destinations: &[Position]) -> usize {
    destinations
        .iter()
        .map(|n| (origin - n).abs())
        .sum::<Position>() as usize
}

pub fn fuel_cost_of_distances(origin: Position, destinations: &[Position]) -> usize {
    destinations
        .iter()
        .map(|n| {
            let distance = (origin - n).abs();
            distance * (distance + 1) / 2
        })
        .sum::<Position>() as usize
}
