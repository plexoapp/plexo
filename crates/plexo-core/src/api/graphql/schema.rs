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
        labels::{LabelsGraphQLMutation, LabelsGraphQLQuery, LabelsGraphQLSubscription},
        members::{MembersGraphQLMutation, MembersGraphQLQuery, MembersGraphQLSubscription},
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
    ChangesGraphQLQuery,
    AIProcessorGraphQLQuery,
    ProfileGraphQLQuery,
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
    ProfileGraphQLMutation,
    AIProcessorGraphQLMutation,
    // ChangesGraphQLMutation,
);

#[derive(MergedSubscription, Default)]
pub struct SubscriptionRoot(
    TasksGraphQLSubscription,
    ProjectsGraphQLSubscription,
    AssetsGraphQLSubscription,
    LabelsGraphQLSubscription,
    MembersGraphQLSubscription,
    TeamsGraphQLSubscription,
    AIProcessorGraphQLSubscription,
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
