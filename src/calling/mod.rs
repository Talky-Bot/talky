use std::collections::HashMap;

pub struct Calling {
    waiting_server: Option<u64>,
    current_conversations: HashMap<u64, u64>
}

impl Calling {
    pub async fn new() -> Self {
        Self {
            waiting_server: None,
            current_conversations: HashMap::new()
        }
    }

    pub async fn add_to_queue(&mut self, channel_id: u64) -> Result<(), ()> {
        if self.channel_in_queue(&channel_id).await {
            return Err(());
        } else if self.waiting_server.is_some() {
            tokio::task::block_in_place(|| {
                self.current_conversations.insert(channel_id.to_owned(), self.waiting_server.as_ref().unwrap().to_owned());
                self.current_conversations.insert(self.waiting_server.as_ref().unwrap().to_owned(), channel_id);
            });
        } else {
            self.waiting_server = Some(channel_id);
        }
        Ok(())
    }

    pub async fn channel_in_queue(&self, &channel_id: &u64) -> bool {
        tokio::task::block_in_place(|| {
            self.current_conversations.contains_key(&channel_id)
        })
    }


    pub async fn retrieve_channel_id(&self, channel_id: &u64) -> Option<u64> {
        tokio::task::block_in_place(|| {
            self.current_conversations.get(&channel_id)
        }).copied()
    }
}