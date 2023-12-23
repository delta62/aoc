macro_rules! example_str {
    ($id:literal) => {{
        let path = format!("examples/{}", $id);
        let res = ::std::fs::read_to_string(&path);

        if res.is_err() {
            eprintln!("Unable to read from {path}");
        }

        res.unwrap()
    }};
}
