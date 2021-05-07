use conversation::Conversation;
use hdk::prelude::*;

use crate::conversation::SignalPayload;

//use holo_hash::EntryHashB64;
#[allow(unused_imports)]
#[allow(dead_code)]
#[allow(unused)]
mod conversation;

#[allow(unused_imports)]
#[allow(dead_code)]
#[allow(unused)]
mod conversation_drawing;
mod types;
mod utils;


pub fn err(reason: &str) -> WasmError {
    WasmError::Guest(String::from(reason))
}

entry_defs![
    Path::entry_def(),
    conversation::Conversation::entry_def(),
    conversation_drawing::ConversationDrawing::entry_def(),    

];

// Give unrestricted access to recv_remote_signal, which is needed for sending remote signals
#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    // grant unrestricted access to accept_cap_claim so other agents can send us claims
    let mut functions: GrantedFunctions = BTreeSet::new();
    functions.insert((zome_info()?.zome_name, "recv_remote_signal".into()));
    create_cap_grant(CapGrantEntry {
        tag: "".into(),
        // empty access converts to unrestricted
        access: ().into(),
        functions,
    })?;

    Ok(InitCallbackResult::Pass)
}

// Function required to process remote signals see hdk/src/p2p.rs
#[hdk_extern]
fn recv_remote_signal(signal: ExternIO) -> ExternResult<()> {
    tracing::debug!("remote signal received");
    let sig: SignalPayload = signal.decode()?;
    debug!("Received remote signal {:?}", sig);
    let msg_to_user = ExternIO::encode("Start round")?;
    Ok(emit_signal(&msg_to_user)?)
}
