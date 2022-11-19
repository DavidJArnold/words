fn order_str(str_in: &str) -> String {
    let mut cvec: Vec<char> = str_in.clone().chars().collect::<Vec<char>>();
    cvec.sort_unstable();
    let sor = cvec.iter().collect::<String>();
    return sor;
}

pub fn anagram(words: Vec<&str>) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    for word in words {
        map.insert(order_str(word).to_owned(), word.to_owned());
    }
    return map;
}

pub fn solve_anagram(
    an_map: &std::collections::HashMap<String, String>,
    anagrist: String,
) -> Option<&String> {
    return an_map.get(&order_str(anagrist.as_str()));
}
