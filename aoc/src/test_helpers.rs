macro_rules! example_str {
    ($id:literal) => {{
        let path = format!("examples/{}", $id);
        std::fs::read_to_string(path).unwrap()
    }};
}

macro_rules! example_bytes {
    ($id:literal) => {{
        let path = format!("examples/{}", $id);
        std::fs::read(path).unwrap()
    }};
}
