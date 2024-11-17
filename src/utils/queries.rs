pub fn find_query(val: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    if let Some((index, _)) = val.iter().enumerate().rev().find(|&(_, line)| {
        line == "==============================================================================="
    }) {
        result = val.into_iter().skip(index + 1).collect();
    }
    result
}
