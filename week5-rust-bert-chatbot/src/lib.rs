use rust_bert::pipelines::conversation::{ConversationModel, ConversationManager};

pub fn dialogue(text: String) -> String {
    let conversation_model = ConversationModel::new(Default::default()).unwrap();
    let mut conversation_manager = ConversationManager::new();

    let conversation_id = conversation_manager.create(&text);
    let output = conversation_model.generate_responses(&mut conversation_manager);
    let response = output.get(&conversation_id).unwrap();
    response.to_string()
}