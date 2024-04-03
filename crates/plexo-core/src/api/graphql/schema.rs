use async_graphql::{
    extensions::{Analyzer, Tracing}, // extensions::OpenTelemetry,
    MergedObject,
    MergedSubscription,
    Schema,
};

use crate::core::app::Core;

use super::{
    operations::{
        assets::{AssetsGraphQLMutation, AssetsGraphQLQuery, AssetsGraphQLSubscription},
        auth::AuthMutation,
        changes::ChangesGraphQLQuery,
        chats::{ChatsGraphQLMutation, ChatsGraphQLQuery, ChatsGraphQLSubscription},
        labels::{LabelsGraphQLMutation, LabelsGraphQLQuery, LabelsGraphQLSubscription},
        members::{MembersGraphQLMutation, MembersGraphQLQuery, MembersGraphQLSubscription},
        messages::{MessagesGraphQLMutation, MessagesGraphQLQuery, MessagesGraphQLSubscription},
        profile::{ProfileGraphQLMutation, ProfileGraphQLQuery},
        projects::{ProjectsGraphQLMutation, ProjectsGraphQLQuery, ProjectsGraphQLSubscription},
        tasks::{TasksGraphQLMutation, TasksGraphQLQuery, TasksGraphQLSubscription},
        teams::{TeamsGraphQLMutation, TeamsGraphQLQuery, TeamsGraphQLSubscription},
    },
    processors::ai::{AIProcessorGraphQLMutation, AIProcessorGraphQLQuery, AIProcessorGraphQLSubscription},
};

#[derive(MergedObject, Default)]
pub struct QueryRoot(
    TasksGraphQLQuery,
    AssetsGraphQLQuery,
    LabelsGraphQLQuery,
    ProjectsGraphQLQuery,
    TeamsGraphQLQuery,
    MembersGraphQLQuery,
    MessagesGraphQLQuery,
    ChangesGraphQLQuery,
    AIProcessorGraphQLQuery,
    ProfileGraphQLQuery,
    ChatsGraphQLQuery,
);

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    TasksGraphQLMutation,
    AuthMutation,
    AssetsGraphQLMutation,
    LabelsGraphQLMutation,
    ProjectsGraphQLMutation,
    TeamsGraphQLMutation,
    MembersGraphQLMutation,
    MessagesGraphQLMutation,
    ProfileGraphQLMutation,
    AIProcessorGraphQLMutation,
    ChatsGraphQLMutation,
    // ChangesGraphQLMutation,
);

#[derive(MergedSubscription, Default)]
pub struct SubscriptionRoot(
    TasksGraphQLSubscription,
    ProjectsGraphQLSubscription,
    AssetsGraphQLSubscription,
    LabelsGraphQLSubscription,
    MembersGraphQLSubscription,
    MessagesGraphQLSubscription,
    TeamsGraphQLSubscription,
    AIProcessorGraphQLSubscription,
    ChatsGraphQLSubscription,
);

pub trait GraphQLSchema {
    fn graphql_api_schema(&self) -> Schema<QueryRoot, MutationRoot, SubscriptionRoot>;
}

impl GraphQLSchema for Core {
    fn graphql_api_schema(&self) -> Schema<QueryRoot, MutationRoot, SubscriptionRoot> {
        Schema::build(
            QueryRoot::default(),
            MutationRoot::default(),
            SubscriptionRoot::default(),
        )
        .data(self.clone()) // TODO: Optimize this
        .extension(Tracing)
        .extension(Analyzer)
        // .extension(open_telemetry)
        .finish()
    }
}
