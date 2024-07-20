/// Builder for MatchConfiguration
#[derive(Debug, Default)]
pub struct MatchConfigurationBuilder {
    strict_case: Option<bool>,
    fuzzy: Option<bool>
}

impl MatchConfigurationBuilder {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct MatchConfiguration {}

impl MatchConfiguration {
    fn builder() -> MatchConfigurationBuilder {
        MatchConfigurationBuilder::default()
    }
}
