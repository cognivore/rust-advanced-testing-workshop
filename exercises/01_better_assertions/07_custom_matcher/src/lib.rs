use googletest::description::Description;
use googletest::matcher::{Matcher, MatcherResult};
use http::StatusCode;

struct CustomMatcher;

impl Matcher for CustomMatcher {
    type ActualT = StatusCode;

    fn matches(&self, actual: &Self::ActualT) -> MatcherResult {
        if *actual == StatusCode::MOVED_PERMANENTLY {
            MatcherResult::Match
        } else {
            MatcherResult::NoMatch
        }
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => format!("is a redirection code").into(),
            MatcherResult::NoMatch => format!("which isn't a redirection code").into(),
        }
    }
}

pub fn is_redirect() -> impl Matcher<ActualT = StatusCode> {
    CustomMatcher
}

#[cfg(test)]
mod tests {
    use crate::is_redirect;
    use googletest::assert_that;
    use http::StatusCode;

    #[test]
    fn success() {
        assert_that!(StatusCode::MOVED_PERMANENTLY, is_redirect());
    }

    #[test]
    fn failure() {
        assert_that!(StatusCode::OK, is_redirect());
    }
}
