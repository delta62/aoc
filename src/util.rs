pub fn transpose<T, F, C, R>(input: impl Iterator<Item = C>, f: F) -> Vec<Vec<T>>
where
    R: Iterator<Item = T>,
    F: Fn(C) -> R,
{
    input.fold(Vec::new(), |mut acc, collection| {
        f(collection).enumerate().for_each(|(i, item)| {
            if acc.get(i).is_none() {
                acc.push(Vec::new());
            }

            acc[i].push(item);
        });

        acc
    })
}
