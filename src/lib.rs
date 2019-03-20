mod response;

use std::collections::HashMap;

const NEW_YORK_TIMES: &str = "https://api.nytimes.com/svc";

struct NewYorkTimes {
    api_key: String,
}
impl NewYorkTimes {
    #[allow(dead_code)]
    pub fn new<S: Into<String>>(api_key: S) -> Self {
        Self {
            api_key: api_key.into(),
        }
    }

    #[allow(dead_code)]
    pub fn most_popular_emailed(
        &self,
        days: u32,
    ) -> Result<response::Response, reqwest::Error> {
        let url = format!(
            "{}/mostpopular/v2/emailed/{}.json?api-key={}",
            NEW_YORK_TIMES, days, self.api_key
        );

        reqwest::get(&url)?.json()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn mostpopular_emailed() {
        let api_key = env::var("NYT_API_KEY").unwrap();

        let new_york_times = NewYorkTimes::new(api_key);

        let mpe = new_york_times.most_popular_emailed(1);
        assert!(mpe.is_ok())
    }
}
