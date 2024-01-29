#![allow(dead_code)]

use std::{collections::HashMap, time::Duration};

use crate::{
    error::Error,
    output::Output,
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
    pub fn new(nodes: Vec<&'a str>) -> Self {
        let mut normalized_nodes = Vec::new();

        for node in nodes {
            normalized_nodes.push(NormalizedNode::new(node, None));
        }

        Self {
            headers: None,
            transport: Transport::new(normalized_nodes, Duration::new(DEFAULT_TIMEOUT, 0)),
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

    pub async fn list_outputs(
        &mut self,
        public_key: &'a str,
        spent: Option<bool>,
    ) -> Result<Vec<Output>, Error> {
        let mut query = HashMap::new();
        query.insert("public_key", public_key);

        if let Some(value) = spent {
            let spent = if value { "true" } else { "false" };
            query.insert("spent", spent);
        }

        let options = RequestOption::new()
            .method(RequestMethod::Get)
            .query(&query);

        let resp: Vec<Output> = self.transport.forward_request(OUTPUTS, &options).await?;
        Ok(resp)
    }
}
