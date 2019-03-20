pub mod error;
pub mod request;
pub mod response;

use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;

use error::{Error, Result};
use request::{MostPopularPeriod, ShareType};
use response::{
    Article, EmailedArticle, Response, SharedArticle, ViewedArticle,
};

const NEW_YORK_TIMES: &str = "https://api.nytimes.com/svc";

struct NewYorkTimes {
    api_key: String,
}
impl NewYorkTimes {
    fn request_and_check_status<A>(url: &String) -> Result<Response<A>>
    where
        A: Article + for<'a> Deserialize<'a>,
    {
        let mut response = reqwest::get(url)?;

        match response.status() {
            StatusCode::BAD_REQUEST => {
                let body: Response<A> = response.json()?;
                Err(Error::api_error(body.errors.unwrap_or(vec![])))
            }
            StatusCode::OK => Ok(response.json()?),

            status_code => Err(Error::unexpected_status(status_code)),
        }
    }

    #[allow(dead_code)]
    pub fn new<S: Into<String>>(api_key: S) -> Self {
        Self {
            api_key: api_key.into(),
        }
    }

    #[allow(dead_code)]
    pub fn most_popular_emailed(
        &self,
        period: MostPopularPeriod,
    ) -> Result<Response<EmailedArticle>> {
        let url = format!(
            "{}/mostpopular/v2/emailed/{}.json?api-key={}",
            NEW_YORK_TIMES,
            json!(period).as_str().unwrap(),
            self.api_key
        );
        println!("url {}", url);

        NewYorkTimes::request_and_check_status(&url)
    }

    #[allow(dead_code)]
    pub fn most_popular_viewed(
        &self,
        period: MostPopularPeriod,
    ) -> Result<Response<ViewedArticle>> {
        let url = format!(
            "{}/mostpopular/v2/viewed/{}.json?api-key={}",
            NEW_YORK_TIMES,
            json!(period).as_str().unwrap(),
            self.api_key
        );

        NewYorkTimes::request_and_check_status(&url)
    }

    #[allow(dead_code)]
    pub fn most_popular_shared(
        &self,
        period: MostPopularPeriod,
        share_type: ShareType,
    ) -> Result<Response<SharedArticle>> {
        let url = format!(
            "{}/mostpopular/v2/shared/{}/{}.json?api-key={}",
            NEW_YORK_TIMES,
            json!(period).as_str().unwrap(),
            json!(share_type).as_str().unwrap(),
            self.api_key
        );

        NewYorkTimes::request_and_check_status(&url)
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

        let mpe =
            new_york_times.most_popular_emailed(MostPopularPeriod::OneDay);
        match mpe {
            Ok(_) => assert!(true),
            Err(e) => panic!("{}", e.kind()),
        }
    }

    #[test]
    fn mostpopular_viewed() {
        let api_key = env::var("NYT_API_KEY").unwrap();

        let new_york_times = NewYorkTimes::new(api_key);

        let mpv = new_york_times.most_popular_viewed(MostPopularPeriod::OneDay);
        match mpv {
            Ok(_) => assert!(true),
            Err(e) => panic!("{}", e.kind()),
        }
    }

    #[test]
    fn mostpopular_shared() {
        let api_key = env::var("NYT_API_KEY").unwrap();

        let new_york_times = NewYorkTimes::new(api_key);

        let mps = new_york_times.most_popular_shared(
            MostPopularPeriod::OneDay,
            ShareType::Facebook,
        );
        match mps {
            Ok(_) => assert!(true),
            Err(e) => panic!("{}", e.kind()),
        }
    }
}
