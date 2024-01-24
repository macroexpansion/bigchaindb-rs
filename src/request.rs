use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use serde::de::DeserializeOwned;

use crate::{error::Error, transaction::TransactionTemplate};

#[derive(Clone, Debug)]
pub struct NormalizedNode<'a> {
    pub endpoint: String,
    pub headers: HashMap<&'a str, &'a str>,
}

impl<'a> NormalizedNode<'a> {
    pub fn new(endpoint: String, headers: HashMap<&'a str, &'a str>) -> Self {
        Self { endpoint, headers }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum RequestMethod {
    Get,
    Post,
}

#[derive(Clone, Debug)]
pub struct RequestOption<'a> {
    pub method: Option<RequestMethod>,
    pub query: Option<&'a HashMap<&'a str, &'a str>>,
    pub headers: Option<&'a HashMap<&'a str, &'a str>>,
    pub json_body: Option<TransactionTemplate>,
}

impl<'a> RequestOption<'a> {
    pub fn new() -> Self {
        Self {
            method: None,
            query: None,
            headers: None,
            json_body: None,
        }
    }

    pub fn method(mut self, method: RequestMethod) -> Self {
        self.method = Some(method);
        self
    }

    pub fn body(mut self, body: TransactionTemplate) -> Self {
        self.json_body = Some(body);
        self
    }
}

#[derive(Clone, Debug)]
pub struct Request<'a> {
    pub node: NormalizedNode<'a>,
    pub backoff_time: Option<Instant>,
    pub retries: usize,
    pub connection_error: Option<String>,
}

impl<'a> Request<'a> {
    pub fn new(node: NormalizedNode<'a>) -> Self {
        Self {
            node,
            backoff_time: Some(Instant::now()),
            retries: 0,
            connection_error: None,
        }
    }

    pub async fn request<T: DeserializeOwned>(
        &mut self,
        url_path: &str,
        config: &RequestOption<'_>,
        timeout: Duration,
        max_backoff_time: Duration,
    ) -> Result<T, Error> {
        let mut request_headers: HashMap<&str, &str> = HashMap::new();
        request_headers.extend(&self.node.headers);
        request_headers.insert("Accept", "application/json");

        if config.json_body.is_some() {
            request_headers.insert("Content-Type", "application/json");
        }

        if let Some(headers) = config.headers {
            request_headers.extend(headers);
        }

        let mut request_config = config.clone();
        request_config.headers = Some(&request_headers);

        let api_url = format!(
            "{node_endpoint}{url_path}",
            node_endpoint = self.node.endpoint
        );

        // If connectionError occurs, a timestamp equal to now +
        // `backoffTimedelta` is assigned to the object.
        // Next time the function is called, it either
        // waits till the timestamp is passed or raises `TimeoutError`.
        // If `ConnectionError` occurs two or more times in a row,
        // the retry count is incremented and the new timestamp is calculated
        // as now + the `backoffTimedelta`
        // The `backoffTimedelta` is the minimum between the default delay
        // multiplied by two to the power of the
        // number of retries or timeout/2 or 10. See Transport class for that
        // If a request is successful, the backoff timestamp is removed,
        // the retry count is back to zero.

        let backoff_time_delta = self.get_backoff_time_delta();

        if timeout < backoff_time_delta {
            return Err(Error::RequestTimeout);
        }

        if backoff_time_delta.as_millis() > 0 {
            tokio::time::sleep(backoff_time_delta).await;
        }

        let request_timeout = timeout
            .checked_sub(backoff_time_delta)
            .unwrap_or(Duration::new(0, 0));

        let resp: T = base_request(&api_url, request_config, Some(request_timeout)).await?;

        self.update_backoff_time(max_backoff_time);

        Ok(resp)
    }

    fn get_backoff_time_delta(&self) -> Duration {
        if let Some(value) = self.backoff_time {
            value.duration_since(Instant::now())
        } else {
            Duration::new(0, 0)
        }
    }

    fn update_backoff_time(&mut self, _max_backoff_time: Duration) {
        if self.connection_error.is_none() {
            self.backoff_time = None;
            self.retries = 0;
        }
    }
}

pub async fn base_request<T: DeserializeOwned>(
    api_url: &str,
    request_config: RequestOption<'_>,
    request_timeout: Option<Duration>,
) -> Result<T, Error> {
    let mut client = if request_config.method.unwrap_or(RequestMethod::Get) == RequestMethod::Post {
        let body = serde_json::to_string(
            &request_config
                .json_body
                .ok_or_else(|| Error::RequestNoBodyProvided)?,
        )
        .map_err(|_| Error::SerdeError)?;

        reqwest::Client::new().post(api_url).body(body)
    } else {
        reqwest::Client::new().get(api_url)
    };

    if let Some(_query) = request_config.query {
        todo!()
    }

    if let Some(headers) = request_config.headers {
        for (key, value) in headers.iter() {
            client = client.header(*key, *value);
        }
    }

    if let Some(timeout) = request_timeout {
        client = client.timeout(timeout);
    }

    let resp = client.send().await.map_err(|_| Error::RequestError)?;

    match resp.error_for_status() {
        Ok(data) => Ok(data.json::<T>().await.map_err(|_| Error::SerdeError)?),
        Err(err) => {
            if err.is_timeout() {
                return Err(Error::RequestTimeout);
            }

            // TODO: handle all errors from reqwest's response

            Err(Error::ResponseError)
        }
    }
}
