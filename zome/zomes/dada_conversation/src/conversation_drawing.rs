use crate::types::ResourceAmount;
use hdk::prelude::*;
use holo_hash::EntryHashB64;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConversationDrawingInfo {
    // TODO: Define the needed metadata
    description: String
}


#[hdk_entry(id = "conversation_drawing", visibility = "public")]
pub struct ConversationDrawing {
    pub creator: AgentPubKey,    
    pub previous_drawing: Option<EntryHashB64>,
    // pub first_picture_hash: Option<EntryHashB64>,
    pub drawing_info: ConversationDrawingInfo
    
    /*
        pub left: Option<EntryHashB64>,
        pub center: Option<EntryHashB64>,
        pub right: Option<EntryHashB64>,
    */
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConversationDrawingInput {    
    pub previous_drawing: Option<EntryHashB64>,
    pub drawing_info: ConversationDrawingInfo
}

fn new_conversation_drawing(input: ConversationDrawingInput) -> () {

}