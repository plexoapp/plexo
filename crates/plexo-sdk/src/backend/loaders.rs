use std::sync::Arc;

use async_graphql::dataloader::DataLoader;

use crate::resources::{
    assets::loader::AssetLoader, changes::loader::ChangeLoader, chats::loader::ChatLoader, labels::loader::LabelLoader,
    members::loader::MemberLoader, messages::loader::MessageLoader, projects::loader::ProjectLoader,
    tasks::loader::TaskLoader, teams::loader::TeamLoader,
};

use super::engine::SDKEngine;

// #[derive(Clone)]
pub struct SDKLoaders {
    pub task_loader: DataLoader<TaskLoader>,
    pub member_loader: DataLoader<MemberLoader>,
    pub project_loader: DataLoader<ProjectLoader>,
    pub team_loader: DataLoader<TeamLoader>,
    pub asset_loader: DataLoader<AssetLoader>,
    pub label_loader: DataLoader<LabelLoader>,
    pub change_loader: DataLoader<ChangeLoader>,
    pub chat_loader: DataLoader<ChatLoader>,
    pub message_loader: DataLoader<MessageLoader>,

    pub engine: Arc<SDKEngine>,
}

impl SDKLoaders {
    pub fn new(engine: Arc<SDKEngine>) -> Self {
        Self {
            task_loader: DataLoader::new(TaskLoader::new(engine.clone()), tokio::spawn),
            member_loader: DataLoader::new(MemberLoader::new(engine.clone()), tokio::spawn),
            project_loader: DataLoader::new(ProjectLoader::new(engine.clone()), tokio::spawn),
            team_loader: DataLoader::new(TeamLoader::new(engine.clone()), tokio::spawn),
            asset_loader: DataLoader::new(AssetLoader::new(engine.clone()), tokio::spawn),
            label_loader: DataLoader::new(LabelLoader::new(engine.clone()), tokio::spawn),
            change_loader: DataLoader::new(ChangeLoader::new(engine.clone()), tokio::spawn),
            chat_loader: DataLoader::new(ChatLoader::new(engine.clone()), tokio::spawn),
            message_loader: DataLoader::new(MessageLoader::new(engine.clone()), tokio::spawn),

            engine,
        }
    }
}
