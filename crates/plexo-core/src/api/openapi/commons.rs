use poem_openapi::Tags;

#[derive(Clone)]
pub struct PlexoOpenAPISpecs(pub String);

#[derive(Tags)]
pub enum PlexoAPITags {
    /// Operations about tasks
    Task,
    /// Operations about projects
    Project,
    /// Operations about members
    Member,
    /// Operations about teams
    Team,
    /// Operations about labels
    Label,
    /// Operations about assets
    Asset,
    /// Operations about changes
    Change,
}
