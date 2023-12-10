macro_rules! example_str {
    ($id:literal) => {{
        let path = format!("examples/{}", $id);
        ::std::fs::read_to_string(&path)
            .inspect_err(|_| eprintln!("Unable to read from {path}"))
            .unwrap()
    }};
}
