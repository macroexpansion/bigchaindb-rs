use std::time::{Duration, Instant};

use serde::de::DeserializeOwned;

use crate::{
    error::Error,
    request::{NormalizedNode, Request, RequestOption},
};

#[derive(Debug, Clone)]
pub struct Transport<'a> {
    pub connection_pool: Vec<Request<'a>>,
    pub timeout: Option<Duration>,
    // the maximum backoff time is 10 seconds
    pub max_backoff_time: Duration,
}

impl<'a> Transport<'a> {
    pub fn new(nodes: Vec<NormalizedNode<'a>>, timeout: Duration) -> Self {
        let mut connection_pool = Vec::new();
        for node in nodes {
            connection_pool.push(Request::new(node));
        }

        Self {
            connection_pool,
            timeout: Some(timeout),
            max_backoff_time: timeout / 2,
        }
    }

    pub async fn forward_request<T: DeserializeOwned>(
        &mut self,
        path: &str,
        options: &RequestOption<'_>,
    ) -> Result<T, Error> {
        while self.timeout.is_some() {
            let start_time = Instant::now();
            let connection = self.pick_connection();
            let response: T = connection
                .request(path, options, self.timeout.unwrap(), self.max_backoff_time)
                .await?;
            let elapsed = Instant::now().duration_since(start_time);

            if connection.backoff_time.get().is_some() && self.timeout.unwrap().as_millis() > 0 {
                self.timeout = self.timeout.unwrap().checked_sub(elapsed);
            } else {
                // No connection error, the response is valid
                return Ok(response);
            }
        }

        Err(Error::RequestTimeout)
    }

    fn pick_connection(&self) -> &Request {
        let mut connection = &self.connection_pool[0];
        for conn in self.connection_pool.iter() {
            if conn.backoff_time.get().unwrap() < connection.backoff_time.get().unwrap() {
                connection = conn;
            }
        }
        connection
    }
}
