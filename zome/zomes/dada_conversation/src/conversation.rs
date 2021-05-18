use hdk::prelude::*;
use holo_hash::EntryHashB64;
use holo_hash::HeaderHashB64;
use std::time::SystemTime;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConversationInfo {
    // TODO: Define the needed metadata
    description: String
}

#[hdk_entry(id = "conversation", visibility = "public")]
pub struct Conversation {
    pub author: AgentPubKey,
    // pub created_at: Timestamp,    
    pub conversation_info: ConversationInfo,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConversationInput {
    pub conversation_info: ConversationInfo,
    // pub cocreators: Vec<AgentPubKey>,
}


impl Conversation {
    
    pub fn add_drawing() {
    
    }
}


// External function that can be called from UI/test
#[hdk_extern]
pub fn start_new_conversation(/* cocreators:Vec<AgentPubKey> */) -> ExternResult<HeaderHashB64> {
    let input = ConversationInput {
    conversation_info: ConversationInfo {
            description: "[desc]".to_string()
        },
        // cocreators: cocreators,
    };
    new_conversation(input)
}

pub fn new_conversation(input: ConversationInput) -> ExternResult<HeaderHashB64> {
    
    let agent_info: AgentInfo = agent_info()?; // Agent that starts the conversation

    let latest_pubkey = agent_info.agent_latest_pubkey;

    // Create conversation entry
    let conversation_entry = Conversation {
        author: latest_pubkey.clone(),        
        conversation_info: input.conversation_info,
    };

    let header_hash = create_entry(&gs)?;
    let entry_hash_conversation = hash_entry(&gs)?;

    // Link to creators conversations
    create_link(latest_pubkey.clone().into(), entry_hash_conversation, LinkTag::new("my_conversations"))?;

    // Send a signal of conversation creation

    Ok(HeaderHashB64::from(header_hash))
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(tag = "signal_name", content = "signal_payload")]
pub enum SignalPayload {
    ConversationStarted(Conversation),
    ConversationFinished,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::fixt::prelude::*;
    use hdk::prelude::*;
    use std::vec;

    #[test]
    fn test_new_conversation() {
     
        }
    }
