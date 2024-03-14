use plexo_sdk::resources::assets::asset::Asset;
use plexo_sdk::resources::assets::operations::{AssetCrudOperations, CreateAssetInput, GetAssetsInput, UpdateAssetInput};
// use plexo_sdk::resources::changes::change::Change;
// use plexo_sdk::resources::changes::operations::{ChangeCrudOperations, CreateChangeInput, GetChangesInput, UpdateChangeInput};
use plexo_sdk::resources::labels::label::Label;
use plexo_sdk::resources::labels::operations::{CreateLabelInput, GetLabelsInput, LabelCrudOperations, UpdateLabelInput};
use plexo_sdk::resources::members::member::Member;
use plexo_sdk::resources::members::operations::{CreateMemberInput, GetMembersInput, MemberCrudOperations, UpdateMemberInput};
use plexo_sdk::resources::projects::operations::{CreateProjectInput, GetProjectsInput, ProjectCrudOperations, UpdateProjectInput};
use plexo_sdk::resources::tasks::operations::{CreateTaskInput, TaskCrudOperations, UpdateTaskInput};
use plexo_sdk::resources::teams::operations::{CreateTeamInput, GetTeamsInput, TeamCrudOperations, UpdateTeamInput};
use plexo_sdk::resources::teams::team::Team;
use plexo_sdk::resources::{projects::project::Project, tasks::task::Task};
use poem::Result;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use poem_openapi::{ApiResponse, OpenApi};
use uuid::Uuid;

use crate::core::app::Core;
use crate::errors::app::PlexoAppError;

use super::{auth::PlexoAPIKeyAuthorization, commons::PlexoAPITags};

pub struct PlexoOpenAPI {
    pub core: Core,
}

impl PlexoOpenAPI {
    pub fn new(core: Core) -> Self {
        Self { core }
    }
}

#[OpenApi]
impl PlexoOpenAPI {
    #[oai(path = "/tasks", method = "post", tag = "PlexoAPITags::Task", operation_id = "create_task")]
    /// Creates a new task leveraging Plexo's AI-powered autonomous task generation.
    /// This function streamlines the planning process by intelligently considering project requirements and team capabilities.
    async fn create_task(&self, input: Json<CreateTaskInput>, _auth: PlexoAPIKeyAuthorization) -> Result<CreateTaskResponse> {
        let task = self.core.engine.create_task(input.0).await.map_err(PlexoAppError::SDKError)?;

        Ok(CreateTaskResponse::Ok(Json(task)))
    }

    #[oai(path = "/tasks/:id", method = "get", tag = "PlexoAPITags::Task", operation_id = "get_task")]
    /// Retrieves a specific task, utilizing Plexo's real-time task tracking feature.
    /// This function aids in monitoring the progress of individual tasks within a project.
    async fn get_task(&self, id: Path<Uuid>, _auth: PlexoAPIKeyAuthorization) -> Result<GetTaskResponse> {
        let task = self.core.engine.get_task(id.0).await.map_err(PlexoAppError::SDKError)?;

        Ok(GetTaskResponse::Ok(Json(task)))
    }

    #[oai(path = "/tasks", method = "get", tag = "PlexoAPITags::Task", operation_id = "get_tasks")]
    /// Retrieves a list of tasks using Plexo's real-time task tracking feature filtered by the input provided.
    /// This function helps in monitoring the progress of a specific group of tasks.
    async fn get_tasks(&self, _auth: PlexoAPIKeyAuthorization) -> Result<GetTasksResponse> {
        let tasks = self.core.engine.get_tasks(None).await.map_err(PlexoAppError::SDKError)?;

        Ok(GetTasksResponse::Ok(Json(tasks)))
    }

    #[oai(
        path = "/tasks/:id",
        method = "put",
        tag = "PlexoAPITags::Task",
        operation_id = "update_task"
    )]
    /// Updates existing information or fields from already created task.
    /// This function helps to modify the characteristics and update properties of a tasks in relation to the project progress.
    async fn update_task(
        &self,
        id: Path<Uuid>,
        input: Json<UpdateTaskInput>,
        _auth: PlexoAPIKeyAuthorization,
    ) -> Result<UpdateTaskResponse> {
        let task = self
            .core
            .engine
            .update_task(id.0, input.0)
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(UpdateTaskResponse::Ok(Json(task)))
    }

    #[oai(
        path = "/tasks/:id",
        method = "delete",
        tag = "PlexoAPITags::Task",
        operation_id = "delete_task"
    )]
    /// Deletes an existing task in a certain project.
    /// This function helps in removing a task that is no longer needed within a project.
    async fn delete_task(&self, id: Path<Uuid>, _auth: PlexoAPIKeyAuthorization) -> Result<DeleteTaskResponse> {
        let task = self.core.engine.delete_task(id.0).await.map_err(PlexoAppError::SDKError)?;

        Ok(DeleteTaskResponse::Ok(Json(task)))
    }

    #[oai(
        path = "/projects",
        method = "post",
        tag = "PlexoAPITags::Project",
        operation_id = "create_project"
    )]
    /// Creates a new project in Plexo, utilizing the platform's AI-powered tools for efficient project initialization.
    /// This function allows for setting up the framework of a project, aligning it with team goals and resources.
    async fn create_project(
        &self,
        input: Json<CreateProjectInput>,
        _auth: PlexoAPIKeyAuthorization,
    ) -> Result<CreateProjectResponse> {
        let project = self
            .core
            .engine
            .create_project(input.0)
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(CreateProjectResponse::Ok(Json(project)))
    }

    #[oai(
        path = "/projects/:id",
        method = "get",
        tag = "PlexoAPITags::Project",
        operation_id = "get_project"
    )]
    /// Retrieves detailed information about a specific project, employing Plexo's real-time tracking and AI analytics.
    /// This function facilitates in-depth insight into project progress and dynamics.
    async fn get_project(&self, id: Path<Uuid>, _auth: PlexoAPIKeyAuthorization) -> Result<GetProjectResponse> {
        let project = self.core.engine.get_project(id.0).await.map_err(PlexoAppError::SDKError)?;

        Ok(GetProjectResponse::Ok(Json(project)))
    }

    #[oai(
        path = "/projects",
        method = "get",
        tag = "PlexoAPITags::Project",
        operation_id = "get_projects"
    )]
    /// Gathers a list of all projects, leveraging Plexo's comprehensive data management and AI insights.
    /// This function aids in overseeing multiple projects, enhancing strategic decision-making.
    async fn get_projects(&self, _auth: PlexoAPIKeyAuthorization) -> GetProjectsResponse {
        let projects = self
            .core
            .engine
            .get_projects(GetProjectsInput::default())
            .await
            .map_err(PlexoAppError::SDKError)
            .unwrap();

        GetProjectsResponse::Ok(Json(projects))
    }

    #[oai(
        path = "/projects/:id",
        method = "put",
        tag = "PlexoAPITags::Project",
        operation_id = "update_project"
    )]
    /// Updates an existing project's information, aligning with changes in project scope or team structure.
    /// This function ensures that project details stay current and relevant.
    async fn update_project(
        &self,
        id: Path<Uuid>,
        input: Json<UpdateProjectInput>,
        _auth: PlexoAPIKeyAuthorization,
    ) -> Result<UpdateProjectResponse> {
        let project = self
            .core
            .engine
            .update_project(id.0, input.0)
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(UpdateProjectResponse::Ok(Json(project)))
    }

    #[oai(
        path = "/projects/:id",
        method = "delete",
        tag = "PlexoAPITags::Project",
        operation_id = "delete_project"
    )]
    /// Removes a project from Plexo's system, maintaining the platform's focus on current and active projects.
    /// This function is crucial for project lifecycle management and resource allocation optimization.
    async fn delete_project(&self, id: Path<Uuid>, _auth: PlexoAPIKeyAuthorization) -> Result<DeleteProjectResponse> {
        let project = self.core.engine.delete_project(id.0).await.map_err(PlexoAppError::SDKError)?;

        Ok(DeleteProjectResponse::Ok(Json(project)))
    }

    #[oai(
        path = "/members",
        method = "post",
        tag = "PlexoAPITags::Member",
        operation_id = "create_member"
    )]
    /// Registers a new member in the Plexo system, harnessing the AI capabilities for optimal team integration.
    /// This function is essential for expanding the team and managing member roles effectively.
    async fn create_member(&self, input: Json<CreateMemberInput>, _auth: PlexoAPIKeyAuthorization) -> Result<CreateMemberResponse> {
        let member = self
            .core
            .engine
            .create_member(input.0)
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(CreateMemberResponse::Ok(Json(member)))
    }

    #[oai(
        path = "/members/:id",
        method = "get",
        tag = "PlexoAPITags::Member",
        operation_id = "get_member"
    )]
    /// Retrieves detailed information about a specific member, utilizing Plexo's efficient member management system.
    /// This function aids in understanding individual member contributions and roles within a project.
    async fn get_member(&self, id: Path<Uuid>, _auth: PlexoAPIKeyAuthorization) -> Result<GetMemberResponse> {
        let member = self.core.engine.get_member(id.0).await.map_err(PlexoAppError::SDKError)?;

        Ok(GetMemberResponse::Ok(Json(member)))
    }

    #[oai(
        path = "/members",
        method = "get",
        tag = "PlexoAPITags::Member",
        operation_id = "get_members"
    )]
    /// Gathers a comprehensive list of all members, leveraging Plexo's robust data management capabilities.
    /// This function enables effective oversight of team composition and individual member roles.
    async fn get_members(&self, _auth: PlexoAPIKeyAuthorization) -> Result<GetMembersResponse> {
        let members = self
            .core
            .engine
            .get_members(GetMembersInput::default())
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(GetMembersResponse::Ok(Json(members)))
    }

    #[oai(
        path = "/members/:id",
        method = "put",
        tag = "PlexoAPITags::Member",
        operation_id = "update_member"
    )]
    /// Updates the information of an existing member, aligning their profile with current roles and responsibilities.
    /// This function ensures member data remains up-to-date and relevant.
    async fn update_member(
        &self,
        id: Path<Uuid>,
        input: Json<UpdateMemberInput>,
        _auth: PlexoAPIKeyAuthorization,
    ) -> Result<UpdateMemberResponse> {
        let member = self
            .core
            .engine
            .update_member(id.0, input.0)
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(UpdateMemberResponse::Ok(Json(member)))
    }

    #[oai(
        path = "/members/:id",
        method = "delete",
        tag = "PlexoAPITags::Member",
        operation_id = "delete_member"
    )]
    /// Deletes a member's profile from Plexo, maintaining the accuracy of team composition and project alignment.
    /// This function is key for managing team dynamics and project resources.
    async fn delete_member(&self, id: Path<Uuid>, _auth: PlexoAPIKeyAuthorization) -> Result<DeleteMemberResponse> {
        let member = self.core.engine.delete_member(id.0).await.map_err(PlexoAppError::SDKError)?;

        Ok(DeleteMemberResponse::Ok(Json(member)))
    }

    #[oai(path = "/teams", method = "post", tag = "PlexoAPITags::Team", operation_id = "create_team")]
    /// Creates a new team within Plexo, employing AI-driven insights for optimal team formation and project alignment.
    /// This function is crucial for structuring teams based on project needs and member skills.
    async fn create_team(&self, input: Json<CreateTeamInput>, _auth: PlexoAPIKeyAuthorization) -> Result<CreateTeamResponse> {
        let team = self.core.engine.create_team(input.0).await.map_err(PlexoAppError::SDKError)?;

        Ok(CreateTeamResponse::Ok(Json(team)))
    }

    #[oai(path = "/teams/:id", method = "get", tag = "PlexoAPITags::Team", operation_id = "get_team")]
    /// Retrieves detailed information about a specific team, showcasing Plexo's capability in team management and analytics.
    /// This function is vital for understanding team dynamics and project involvement.
    async fn get_team(&self, id: Path<Uuid>, _auth: PlexoAPIKeyAuthorization) -> Result<GetTeamResponse> {
        let team = self.core.engine.get_team(id.0).await.map_err(PlexoAppError::SDKError)?;

        Ok(GetTeamResponse::Ok(Json(team)))
    }

    #[oai(path = "/teams", method = "get", tag = "PlexoAPITags::Team", operation_id = "get_teams")]
    /// Compiles a list of all teams, demonstrating Plexo's comprehensive approach to team oversight and project distribution.
    /// This function is essential for managing multiple teams across various projects.
    async fn get_teams(&self, _auth: PlexoAPIKeyAuthorization) -> Result<GetTeamsResponse> {
        let teams = self
            .core
            .engine
            .get_teams(GetTeamsInput::default())
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(GetTeamsResponse::Ok(Json(teams)))
    }

    #[oai(
        path = "/teams/:id",
        method = "put",
        tag = "PlexoAPITags::Team",
        operation_id = "update_team"
    )]
    /// Updates existing team details, reflecting changes in team structure or project alignment.
    /// This function plays a key role in keeping team information current and aligned with ongoing projects.
    async fn update_team(
        &self,
        id: Path<Uuid>,
        input: Json<UpdateTeamInput>,
        _auth: PlexoAPIKeyAuthorization,
    ) -> Result<UpdateTeamResponse> {
        let team = self
            .core
            .engine
            .update_team(id.0, input.0)
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(UpdateTeamResponse::Ok(Json(team)))
    }

    #[oai(
        path = "/teams/:id",
        method = "delete",
        tag = "PlexoAPITags::Team",
        operation_id = "delete_team"
    )]
    /// Removes a team from the Plexo system, ensuring that the platform's focus remains on active and relevant teams.
    /// This function is critical for effective project management and resource allocation.
    async fn delete_team(&self, id: Path<Uuid>, _auth: PlexoAPIKeyAuthorization) -> Result<DeleteTeamResponse> {
        let team = self.core.engine.delete_team(id.0).await.map_err(PlexoAppError::SDKError)?;

        Ok(DeleteTeamResponse::Ok(Json(team)))
    }

    #[oai(
        path = "/labels",
        method = "post",
        tag = "PlexoAPITags::Label",
        operation_id = "create_label"
    )]
    /// Introduces a new label to Plexo, enhancing project categorization and task prioritization.
    /// This function is essential for maintaining organized and efficient project workflows.
    async fn create_label(&self, input: Json<CreateLabelInput>, _auth: PlexoAPIKeyAuthorization) -> Result<CreateLabelResponse> {
        let label = self
            .core
            .engine
            .create_label(input.0)
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(CreateLabelResponse::Ok(Json(label)))
    }

    #[oai(
        path = "/labels/:id",
        method = "get",
        tag = "PlexoAPITags::Label",
        operation_id = "get_label"
    )]
    /// Retrieves specific details about a label, utilizing Plexo's structured approach to task and project organization.
    /// This function aids in understanding the role and impact of labels within project management.
    async fn get_label(&self, id: Path<Uuid>, _auth: PlexoAPIKeyAuthorization) -> Result<GetLabelResponse> {
        let label = self.core.engine.get_label(id.0).await.map_err(PlexoAppError::SDKError)?;

        Ok(GetLabelResponse::Ok(Json(label)))
    }

    #[oai(path = "/labels", method = "get", tag = "PlexoAPITags::Label", operation_id = "get_labels")]
    /// Compiles a list of all labels in Plexo, showcasing the platform's comprehensive categorization capabilities.
    /// This function is vital for overseeing task organization and project prioritization.
    async fn get_labels(&self, _auth: PlexoAPIKeyAuthorization) -> Result<GetLabelsResponse> {
        let labels = self
            .core
            .engine
            .get_labels(GetLabelsInput::default())
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(GetLabelsResponse::Ok(Json(labels)))
    }

    #[oai(
        path = "/labels/:id",
        method = "put",
        tag = "PlexoAPITags::Label",
        operation_id = "update_label"
    )]
    /// Updates an existing label's information, ensuring its relevance and effectiveness in project categorization.
    /// This function is key to maintaining an organized and efficient project management system.
    async fn update_label(
        &self,
        id: Path<Uuid>,
        input: Json<UpdateLabelInput>,
        _auth: PlexoAPIKeyAuthorization,
    ) -> Result<UpdateLabelResponse> {
        let label = self
            .core
            .engine
            .update_label(id.0, input.0)
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(UpdateLabelResponse::Ok(Json(label)))
    }

    #[oai(
        path = "/labels/:id",
        method = "delete",
        tag = "PlexoAPITags::Label",
        operation_id = "delete_label"
    )]
    /// Deletes a label from Plexo, streamlining the categorization system to focus on current and active labels.
    /// This function is important for maintaining clarity and efficiency in project organization.
    async fn delete_label(&self, id: Path<Uuid>, _auth: PlexoAPIKeyAuthorization) -> Result<DeleteLabelResponse> {
        let label = self.core.engine.delete_label(id.0).await.map_err(PlexoAppError::SDKError)?;

        Ok(DeleteLabelResponse::Ok(Json(label)))
    }

    #[oai(
        path = "/assets",
        method = "post",
        tag = "PlexoAPITags::Asset",
        operation_id = "create_asset"
    )]
    /// Introduces a new asset to Plexo, enhancing project categorization and task prioritization.
    /// This function is essential for maintaining organized and efficient project workflows.
    async fn create_asset(&self, input: Json<CreateAssetInput>, _auth: PlexoAPIKeyAuthorization) -> Result<CreateAssetResponse> {
        let asset = self
            .core
            .engine
            .create_asset(input.0)
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(CreateAssetResponse::Ok(Json(asset)))
    }

    #[oai(
        path = "/assets/:id",
        method = "get",
        tag = "PlexoAPITags::Asset",
        operation_id = "get_asset"
    )]
    /// Retrieves specific details about an asset, utilizing Plexo's structured approach to task and project organization.
    /// This function aids in understanding the role and impact of assets within project management.
    async fn get_asset(&self, id: Path<Uuid>, _auth: PlexoAPIKeyAuthorization) -> Result<GetAssetResponse> {
        let asset = self.core.engine.get_asset(id.0).await.map_err(PlexoAppError::SDKError)?;

        Ok(GetAssetResponse::Ok(Json(asset)))
    }

    #[oai(path = "/assets", method = "get", tag = "PlexoAPITags::Asset", operation_id = "get_assets")]
    /// Compiles a list of all assets in Plexo, showcasing the platform's comprehensive categorization capabilities.
    /// This function is vital for overseeing task organization and project prioritization.
    async fn get_assets(&self, _auth: PlexoAPIKeyAuthorization) -> Result<GetAssetsResponse> {
        let assets = self
            .core
            .engine
            .get_assets(GetAssetsInput::default())
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(GetAssetsResponse::Ok(Json(assets)))
    }

    #[oai(
        path = "/assets/:id",
        method = "put",
        tag = "PlexoAPITags::Asset",
        operation_id = "update_asset"
    )]
    /// Updates an existing asset's information, ensuring its relevance and effectiveness in project categorization.
    /// This function is key to maintaining an organized and efficient project management system.
    async fn update_asset(
        &self,
        id: Path<Uuid>,
        input: Json<UpdateAssetInput>,
        _auth: PlexoAPIKeyAuthorization,
    ) -> Result<UpdateAssetResponse> {
        let asset = self
            .core
            .engine
            .update_asset(id.0, input.0)
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(UpdateAssetResponse::Ok(Json(asset)))
    }

    #[oai(
        path = "/assets/:id",
        method = "delete",
        tag = "PlexoAPITags::Asset",
        operation_id = "delete_asset"
    )]
    /// Deletes an asset from Plexo, streamlining the categorization system to focus on current and active assets.
    /// This function is important for maintaining clarity and efficiency in project organization.
    async fn delete_asset(&self, id: Path<Uuid>, _auth: PlexoAPIKeyAuthorization) -> Result<DeleteAssetResponse> {
        let asset = self.core.engine.delete_asset(id.0).await.map_err(PlexoAppError::SDKError)?;

        Ok(DeleteAssetResponse::Ok(Json(asset)))
    }

    // #[oai(
    //     path = "/changes",
    //     method = "post",
    //     tag = "PlexoAPITags::Change",
    //     operation_id = "create_change"
    // )]
    // /// Introduces a new change to Plexo, enhancing project categorization and task prioritization.
    // /// This function is essential for maintaining organized and efficient project workflows.
    // async fn create_change(&self, input: Json<CreateChangeInput>, _auth: PlexoAPIKeyAuthorization) -> Result<CreateChangeResponse> {
    //     let change = self
    //         .core
    //         .engine
    //         .create_change(input.0)
    //         .await
    //         .map_err(PlexoAppError::SDKError)?;

    //     Ok(CreateChangeResponse::Ok(Json(change)))
    // }

    // #[oai(
    //     path = "/changes/:id",
    //     method = "get",
    //     tag = "PlexoAPITags::Change",
    //     operation_id = "get_change"
    // )]
    // /// Retrieves specific details about a change, utilizing Plexo's structured approach to task and project organization.
    // /// This function aids in understanding the role and impact of changes within project management.
    // async fn get_change(&self, id: Path<Uuid>, _auth: PlexoAPIKeyAuthorization) -> Result<GetChangeResponse> {
    //     let change = self.core.engine.get_change(id.0).await.map_err(PlexoAppError::SDKError)?;

    //     Ok(GetChangeResponse::Ok(Json(change)))
    // }

    // #[oai(
    //     path = "/changes",
    //     method = "get",
    //     tag = "PlexoAPITags::Change",
    //     operation_id = "get_changes"
    // )]
    // /// Compiles a list of all changes in Plexo, showcasing the platform's comprehensive categorization capabilities.
    // /// This function is vital for overseeing task organization and project prioritization.
    // async fn get_changes(&self, _auth: PlexoAPIKeyAuthorization) -> Result<GetChangesResponse> {
    //     let changes = self
    //         .core
    //         .engine
    //         .get_changes(GetChangesInput::default())
    //         .await
    //         .map_err(PlexoAppError::SDKError)?;

    //     Ok(GetChangesResponse::Ok(Json(changes)))
    // }

    // #[oai(
    //     path = "/changes/:id",
    //     method = "put",
    //     tag = "PlexoAPITags::Change",
    //     operation_id = "update_change"
    // )]
    // /// Updates an existing change's information, ensuring its relevance and effectiveness in project categorization.
    // /// This function is key to maintaining an organized and efficient project management system.
    // async fn update_change(
    //     &self,
    //     id: Path<Uuid>,
    //     input: Json<UpdateChangeInput>,
    //     _auth: PlexoAPIKeyAuthorization,
    // ) -> Result<UpdateChangeResponse> {
    //     let change = self
    //         .core
    //         .engine
    //         .update_change(id.0, input.0)
    //         .await
    //         .map_err(PlexoAppError::SDKError)?;

    //     Ok(UpdateChangeResponse::Ok(Json(change)))
    // }

    // #[oai(
    //     path = "/changes/:id",
    //     method = "delete",
    //     tag = "PlexoAPITags::Change",
    //     operation_id = "delete_change"
    // )]
    // /// Deletes a change from Plexo, streamlining the categorization system to focus on current and active changes.
    // /// This function is important for maintaining clarity and efficiency in project organization.
    // async fn delete_change(&self, id: Path<Uuid>, _auth: PlexoAPIKeyAuthorization) -> Result<DeleteChangeResponse> {
    //     let change = self.core.engine.delete_change(id.0).await.map_err(PlexoAppError::SDKError)?;

    //     Ok(DeleteChangeResponse::Ok(Json(change)))
    // }
}

//
//
//

//
//
//

#[derive(ApiResponse)]
enum CreateProjectResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Project>),
}

#[derive(ApiResponse)]
enum GetProjectsResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Vec<Project>>),
}

#[derive(ApiResponse)]
enum GetProjectResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Project>),
}

#[derive(ApiResponse)]
enum UpdateProjectResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Project>),
}

#[derive(ApiResponse)]
enum DeleteProjectResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Project>),
}

//
//
//

//
//
//

#[derive(ApiResponse)]
enum CreateTaskResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Task>),
}

#[derive(ApiResponse)]
enum GetTasksResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Vec<Task>>),
}

#[derive(ApiResponse)]
enum GetTaskResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Task>),
    // #[oai(status = 404)]
    // NotFound,
}

#[derive(ApiResponse)]
enum UpdateTaskResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Task>),
    // #[oai(status = 404)]
    // NotFound,
}

#[derive(ApiResponse)]
enum DeleteTaskResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Task>),
    // #[oai(status = 404)]
    // NotFound,
}

//
//
//

//
//
//

#[derive(ApiResponse)]
enum CreateMemberResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Member>),
}

#[derive(ApiResponse)]
enum GetMembersResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Vec<Member>>),
}

#[derive(ApiResponse)]
enum GetMemberResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Member>),
    // #[oai(status = 404)]
    // NotFound,
}

#[derive(ApiResponse)]
enum UpdateMemberResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Member>),
    // #[oai(status = 404)]
    // NotFound,
}

#[derive(ApiResponse)]
enum DeleteMemberResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Member>),
    // #[oai(status = 404)]
    // NotFound,
}

//
//
//

//
//
//

#[derive(ApiResponse)]
enum CreateTeamResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Team>),
}

#[derive(ApiResponse)]
enum GetTeamsResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Vec<Team>>),
}

#[derive(ApiResponse)]
enum GetTeamResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Team>),
    // #[oai(status = 404)]
    // NotFound,
}

#[derive(ApiResponse)]
enum UpdateTeamResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Team>),
    // #[oai(status = 404)]
    // NotFound,
}

#[derive(ApiResponse)]
enum DeleteTeamResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Team>),
    // #[oai(status = 404)]
    // NotFound,
}

//
//
//

//
//
//

#[derive(ApiResponse)]
enum CreateLabelResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Label>),
}

#[derive(ApiResponse)]
enum GetLabelsResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Vec<Label>>),
}

#[derive(ApiResponse)]
enum GetLabelResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Label>),
    // #[oai(status = 404)]
    // NotFound,
}

#[derive(ApiResponse)]
enum UpdateLabelResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Label>),
    // #[oai(status = 404)]
    // NotFound,
}

#[derive(ApiResponse)]
enum DeleteLabelResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Label>),
    // #[oai(status = 404)]
    // NotFound,
}

//
//
//

//
//
//

#[derive(ApiResponse)]
enum CreateAssetResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Asset>),
}

#[derive(ApiResponse)]
enum GetAssetsResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Vec<Asset>>),
}

#[derive(ApiResponse)]
enum GetAssetResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Asset>),
    // #[oai(status = 404)]
    // NotFound,
}

#[derive(ApiResponse)]
enum UpdateAssetResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Asset>),
    // #[oai(status = 404)]
    // NotFound,
}

#[derive(ApiResponse)]
enum DeleteAssetResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Asset>),
    // #[oai(status = 404)]
    // NotFound,
}

//
//
//

//
//
//

// #[derive(ApiResponse)]
// enum CreateChangeResponse {
//     /// Returns when the user is successfully created.
//     #[oai(status = 200)]
//     Ok(Json<Change>),
// }

// #[derive(ApiResponse)]
// enum GetChangesResponse {
//     /// Returns when the user is successfully created.
//     #[oai(status = 200)]
//     Ok(Json<Vec<Change>>),
// }

// #[derive(ApiResponse)]
// enum GetChangeResponse {
//     /// Returns when the user is successfully created.
//     #[oai(status = 200)]
//     Ok(Json<Change>),
//     // #[oai(status = 404)]
//     // NotFound,
// }

// #[derive(ApiResponse)]
// enum UpdateChangeResponse {
//     /// Returns when the user is successfully created.
//     #[oai(status = 200)]
//     Ok(Json<Change>),
//     // #[oai(status = 404)]
//     // NotFound,
// }

// #[derive(ApiResponse)]
// enum DeleteChangeResponse {
//     /// Returns when the user is successfully created.
//     #[oai(status = 200)]
//     Ok(Json<Change>),
//     // #[oai(status = 404)]
//     // NotFound,
// }
