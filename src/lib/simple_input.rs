pub fn get_user_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.remove(input.len() - 1);
    input.remove(input.len() - 1);
    input
}
