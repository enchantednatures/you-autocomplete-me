struct Match<'a> {
    search: String,
    matching_characters: Vec<char>,
    result: &'a str,
}

impl<'a> Match<'a> {
    fn new(search: String, matching_characters: Vec<char>, result: &'a str) -> Self {
        Self {
            search,
            matching_characters,
            result,
        }
    }
}
