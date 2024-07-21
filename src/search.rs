use std::fmt::Debug;

/// Builder for MatchConfiguration
#[derive(Debug, Default)]
pub struct SearchBuilder<S: Debug> {
    search: S,
    strict_case: Option<bool>,
    fuzzy: Option<bool>,
}

#[derive(Default, Debug)]
pub struct NoSearch;

#[derive(Debug)]
pub struct WithSearch<'a>(&'a str);

impl<S: Debug> SearchBuilder<S> {
    fn search(self, search: &str) -> SearchBuilder<WithSearch<'_>> {
        let Self {
            strict_case, fuzzy, ..
        } = self;
        SearchBuilder {
            search: WithSearch(search),
            strict_case,
            fuzzy,
        }
    }

    fn fuzzy(&mut self, fuzzy: bool) -> &mut Self {
        self.fuzzy = Some(fuzzy);
        self
    }
}

impl<'a> SearchBuilder<WithSearch<'a>> {
    fn build(self) -> Search<'a> {
        Search {
            search: self.search.0,
            strict_case: self.strict_case.unwrap_or(false),
            fuzzy: self.fuzzy.unwrap_or(false),
        }
    }
}

impl SearchBuilder<NoSearch> {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct Search<'a> {
    search: &'a str,
    strict_case: bool,
    fuzzy: bool,
}

impl<'a> Search<'a> {
    fn builder() -> SearchBuilder<NoSearch> {
        SearchBuilder::default()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn search_builder_value_is_search() {
        let search_value = "test";
        let search = Search::builder().search(search_value).build();
        assert_eq!(search_value, search.search)
    }
}
