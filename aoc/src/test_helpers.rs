macro_rules! example_str {
    ($id:literal) => {{
        let path = format!("examples/{}", $id);
        std::fs::read_to_string(path).unwrap()
    }};
}
