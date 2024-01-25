#![allow(dead_code)]

use std::{collections::HashMap, time::Duration};

use crate::{
    error::Error,
    request::{NormalizedNode, RequestMethod, RequestOption},
    transaction::TransactionTemplate,
    transport::Transport,
};

const BLOCKS: &'static str = "blocks";
const BLOCKS_DETAIL: &'static str = "blocks/%(blockHeight)s";
const OUTPUTS: &'static str = "outputs";
const TRANSACTIONS: &'static str = "transactions";
const TRANSACTIONS_SYNC: &'static str = "transactions?mode=sync";
const TRANSACTIONS_ASYNC: &'static str = "transactions?mode=async";
const TRANSACTIONS_COMMIT: &'static str = "transactions?mode=commit";
const TRANSACTIONS_DETAIL: &'static str = "transactions/%(transactionId)s";
const ASSETS: &'static str = "assets";
const METADATA: &'static str = "metadata";

const DEFAULT_NODE: &'static str = "http://localhost:9984/api/v1/";
const DEFAULT_TIMEOUT: u64 = 20; // default timeout is 20 seconds

#[derive(Debug, Clone)]
pub struct Connection<'a> {
    // Common headers for every request
    pub headers: Option<HashMap<&'a str, &'a str>>,
    pub transport: Transport<'a>,
}

impl<'a> Connection<'a> {
    pub fn new(
        nodes: Vec<Option<String>>,
        headers: HashMap<&'a str, &'a str>,
        timeout: Option<Duration>,
    ) -> Self {
        let mut normalized_nodes = Vec::new();

        for node in nodes {
            normalized_nodes.push(NormalizedNode::new(
                node.unwrap_or(DEFAULT_NODE.to_string()),
                headers.clone(),
            ));
        }

        Self {
            headers: Some(headers),
            transport: Transport::new(
                normalized_nodes,
                timeout.unwrap_or(Duration::new(DEFAULT_TIMEOUT, 0)),
            ),
        }
    }

    pub async fn post_transaction_commit(
        &mut self,
        transaction: TransactionTemplate,
    ) -> Result<TransactionTemplate, Error> {
        let options = RequestOption::new()
            .method(RequestMethod::Post)
            .body(transaction);
        let resp: TransactionTemplate = self
            .transport
            .forward_request(TRANSACTIONS_COMMIT, &options)
            .await?;
        Ok(resp)
    }
}
