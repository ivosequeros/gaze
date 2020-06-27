use super::filter::{Constraint, Filter};
use crate::client::Client;
use crate::codec::{Model::*, Type};
use crate::router::Router;
use std::collections::HashMap;

pub type Selector = HashMap<String, HashMap<usize, HashMap<Vec<u8>, Vec<Vec<u8>>>>>;

pub trait Selection {
    async fn relay(client_ids: &Vec<Vec<u8>>, message: &[u8]);
    async fn select(router: &Router, message_type: &Vec<u8>, model: &Type<'_>, message: &[u8]);
}

impl Selection for Selector {
    async fn relay(client_ids: &Vec<Vec<u8>>, message: &[u8]) {
        let client: Client = router.clients.get_mut(client_id).unwrap();
        let writer = client.writer.lock().await;
        writer.write(message).await;
    }

    pub async fn select(router: &Router, message_type: &Vec<u8>, model: &Type<'_>, message: &[u8]) {
        let mut position = 0;
        let mut start = 0;

        match *model {
            Literal(Type::Int) => {
                let number_size = message[start..];

                match router.subscriptions.get(position) {
                    Ok(subscriptions) => {
                        match subscriptions.get(message[start..start + number_size]) {
                            Ok(subscriptions) => {
                                let client_ids = subscription(message);
                                tokio::spawn(relay(client_ids, message));
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }

                start = start + number_size;
            }
            Literal(Type::String) => {}
            Record(record) => {}
        }
    }
}
