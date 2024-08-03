//! # Search Module
//!
//! This module provides a builder pattern implementation for creating search queries with options for strict case matching and fuzzy matching.
//!
//! ## Design
//!
//! The module uses a type-safe builder pattern to ensure that the search query is always provided before building the `Search` object. The design employs a `SearchMarker` trait to restrict the types that can be used in the builder, preventing the creation of invalid `Search` objects.
//!
//! ## Example
//!
//! ```
//! use you_autocomplete_me::search::Search;
//! let search = Search::builder()
//!     .search("example")
//!     .strict(true)
//!     .fuzzy(true)
//!     .build();
//!
//! assert_eq!(search.search, "example");
//! assert!(search.strict_case);
//! assert!(search.fuzzy);
//! ```
//!
//! ## Flow Chart
//!
//! ```text
//! SearchBuilder<NoSearch> --search(&str)--> SearchBuilder<WithSearch> --build()--> Search
//! ```
//!
//! The flow chart shows the state transitions of the `SearchBuilder` from `NoSearch` to `WithSearch` and finally to the `Search` struct when `build` is called.

use std::fmt::Debug;

/// A TypeState builder for [Search].
#[derive(Debug, Default)]
pub struct SearchBuilder<S: Debug + SearchMarker> {
    search: S,
    strict_case: Option<bool>,
    fuzzy: Option<bool>,
}

/// A marker trait to ensure the builder state.
pub trait SearchMarker {}

impl SearchMarker for NoSearch {}

impl<'a> SearchMarker for WithSearch<'a> {}

/// Represents the initial state with no search value.
#[derive(Default, Debug)]
pub struct NoSearch;

/// Represents the state with a search value.
#[derive(Debug)]
pub struct WithSearch<'a>(&'a str);

impl<S: Debug + SearchMarker> SearchBuilder<S> {
    /// Sets the search value.
    ///
    /// # Examples
    ///
    /// ```
    /// use you_autocomplete_me::search::Search;
    /// let builder = Search::builder().search("example");
    /// ```
    pub fn search(self, search: &str) -> SearchBuilder<WithSearch<'_>> {
        let Self {
            strict_case, fuzzy, ..
        } = self;
        SearchBuilder {
            search: WithSearch(search),
            strict_case,
            fuzzy,
        }
    }

    /// Sets strict case matching for the search.
    ///
    /// # Examples
    ///
    /// ```
    /// use you_autocomplete_me::search::Search;
    /// let mut builder = Search::builder().search("example");
    /// builder.strict(true);
    /// ```
    pub fn strict(mut self, strict: bool) -> Self {
        self.strict_case = Some(strict);
        self
    }

    /// Sets fuzzy matching.
    ///
    /// # Examples
    ///
    /// ```
    /// use you_autocomplete_me::search::Search;
    /// let search = Search::builder().search("example").fuzzy(true).build();
    /// assert!(search.fuzzy);
    /// ```
    pub fn fuzzy(mut self, fuzzy: bool) -> Self {
        self.fuzzy = Some(fuzzy);
        self
    }
}

impl<'a> SearchBuilder<WithSearch<'a>> {
    /// Builds the `Search` object.
    ///
    /// # Examples
    ///
    /// ```
    /// use you_autocomplete_me::search::Search;
    /// let search = Search::builder()
    ///     .search("example")
    ///     .strict(true)
    ///     .fuzzy(true)
    ///     .build();
    /// ```
    pub fn build(self) -> Search<'a> {
        Search {
            search: self.search.0,
            strict_case: self.strict_case.unwrap_or(false),
            fuzzy: self.fuzzy.unwrap_or(false),
        }
    }
}

impl SearchBuilder<NoSearch> {
    /// Creates a new `SearchBuilder` using default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use you_autocomplete_me::search::Search;
    /// let builder = Search::builder();
    /// ```
    pub fn new() -> Self {
        Default::default()
    }
}

/// Represents the search query.
#[derive(Debug, Default)]
pub struct Search<'a> {
    /// The search query string.
    pub search: &'a str,
    /// Use strict casing.
    pub strict_case: bool,
    /// Allow fuzzy search.
    pub fuzzy: bool,
}

impl<'a> Search<'a> {
    /// Creates a new `Search` object with the provided search string.
    ///
    /// # Examples
    ///
    /// ```
    /// use you_autocomplete_me::search::Search;
    /// let search = Search::new("example");
    /// ```
    pub fn new(search: &'a str) -> Self {
        Self {
            search,
            ..Default::default()
        }
    }

    /// Checks if smart case matching should be used.
    ///
    /// # Examples
    ///
    /// ```
    /// use you_autocomplete_me::search::Search;
    /// let search = Search::new("Example");
    /// assert!(search.is_smart_case());
    /// ```
    #[inline]
    pub fn is_smart_case(&self) -> bool {
        !self.strict_case && self.search.chars().any(|c| c.is_uppercase())
    }

    /// Returns a vector of capitalized characters in the search string.
    ///
    /// # Examples
    ///
    /// ```
    /// use you_autocomplete_me::search::Search;
    /// let search = Search::new("Example");
    /// assert_eq!(search.capitlized_chars(), vec!['E']);
    /// ```
    pub fn capitlized_chars(&self) -> Vec<char> {
        self.search.chars().filter(|c| c.is_uppercase()).collect()
    }

    /// Creates a new `SearchBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use you_autocomplete_me::search::Search;
    /// let builder = Search::builder();
    /// ```
    pub fn builder() -> SearchBuilder<NoSearch> {
        SearchBuilder::default()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn building_a_search_with_builder_uses_value() {
        let search_value = "test";
        let search = Search::builder().search(search_value).build();
        assert_eq!(search_value, search.search)
    }
}
