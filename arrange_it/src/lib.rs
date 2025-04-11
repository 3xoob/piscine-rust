pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<_> = phrase
        .split_whitespace()
        .map(|word| {
            let position = word
                .chars()
                .filter_map(|c| c.to_digit(10))
                .next()
                .unwrap_or(0);
            (position, word)
        })
        .collect();
    words.sort_by_key(|&(position, _)| position);
    words
        .into_iter()
        .map(|(_, word)| word)
        .collect::<Vec<_>>()
        .join(" ")
}
