use super::{
    app::Core,
    config::{
        ADMIN_EMAIL, ADMIN_NAME, ADMIN_PASSWORD, ADMIN_PHOTO_URL, ORGANIZATION_EMAIL, ORGANIZATION_HUB_ID,
        ORGANIZATION_NAME, ORGANIZATION_PHOTO_URL, ORGANIZATION_PLAN_ID, ORGANIZATION_URL,
    },
    email::FirstWelcomeTemplate,
};

use askama::Template;

use plexo_sdk::{
    common::commons::SortOrder,
    organization::operations::{Organization, OrganizationCrudOperations, OrganizationInitializationInputBuilder},
    resources::{
        changes::change::ChangeResourceType,
        members::{
            extensions::{CreateMemberFromEmailInputBuilder, MembersExtensionOperations},
            member::MemberRole,
            operations::{GetMembersInput, GetMembersInputBuilder, MemberCrudOperations},
        },
    },
};
use tokio::task;
use tokio_stream::StreamExt;
use tracing::info;

impl Core {
    pub async fn prelude(&self) -> Result<Organization, Box<dyn std::error::Error>> {
        self.normalize_admin_user().await?;

        let org = match self.engine.get_organization().await? {
            Some(organization) => Ok(organization),
            None => self.initialize_organization().await,
        }?;

        let engine = self.engine.clone();

        task::spawn(async move {
            while let Some(not) = engine.listen(ChangeResourceType::Tasks).await.unwrap().next().await {
                info!("task change: {:?}", not);
            }
        });

        Ok(org)
    }

    async fn normalize_admin_user(&self) -> Result<(), Box<dyn std::error::Error>> {
        let default_admin_email = (*ADMIN_EMAIL).clone();
        let default_admin_password = (*ADMIN_PASSWORD).clone();
        let default_admin_name = (*ADMIN_NAME).clone();
        let default_admin_photo_url = (*ADMIN_PHOTO_URL).clone();

        let hashed_password = self.auth.hash_password(default_admin_password.as_str());
        let default_admin_role = MemberRole::Admin;

        match self.engine.get_member_by_email(default_admin_email.clone()).await {
            Ok(Some(_admin)) => {
                info!("default admin user already exists: {}", default_admin_email);
                return Ok(());
            }
            Err(e) => {
                info!("error checking for default admin user: {}", e);
                return Err(Box::new(e));
            }
            _ => {}
        }

        if !self.engine.get_members(GetMembersInput::default()).await?.is_empty() {
            info!("members already exist, skipping default admin user creation");
            return Ok(());
        }

        info!("creating default admin user: {}", default_admin_email);

        self.engine
            .create_member_from_email(
                CreateMemberFromEmailInputBuilder::default()
                    .email(default_admin_email)
                    .name(default_admin_name)
                    .photo_url(default_admin_photo_url)
                    .role(default_admin_role)
                    .password_hash(hashed_password)
                    .build()?,
            )
            .await?;
        Ok(())
    }

    async fn initialize_organization(&self) -> Result<Organization, Box<dyn std::error::Error>> {
        let members = self
            .engine
            .get_members(
                GetMembersInputBuilder::default()
                    .limit(1)
                    .sort_by("created_at".to_string())
                    .sort_order(SortOrder::Asc)
                    .build()?,
            )
            .await?;

        let first_member = members.first().unwrap();

        let mut org_data = OrganizationInitializationInputBuilder::default()
            .name((*ORGANIZATION_NAME).to_owned())
            .email((*ORGANIZATION_EMAIL).to_owned())
            .photo_url((*ORGANIZATION_PHOTO_URL).to_owned())
            .owner_id(first_member.id);

        if let Some(org_hub_id) = (*ORGANIZATION_HUB_ID).to_owned() {
            org_data = org_data.hub_id(org_hub_id);
        }

        if let Some(org_plan_id) = (*ORGANIZATION_PLAN_ID).to_owned() {
            org_data = org_data.plan_id(org_plan_id);
        }

        let org = self
            .engine
            .initialize_organization(first_member.id, org_data.build()?)
            .await
            .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?;

        let org_email = org.email.clone();

        self.first_time_welcome_email(org_email)?;

        Ok(org)
    }

    fn first_time_welcome_email(&self, organization_owner_email: String) -> Result<(), Box<dyn std::error::Error>> {
        let from = "onboarding@plexo.app";
        let to = organization_owner_email.as_str();
        let subject = "Welcome to Plexo!";
        // let html = "<h1>Welcome to Plexo!</h1>";

        let welcome = FirstWelcomeTemplate {
            admin_email: (*ADMIN_EMAIL).to_owned(),
            admin_password: (*ADMIN_PASSWORD).to_owned(),
            plexo_url: (*ORGANIZATION_URL).to_owned(),
            // organization_name: (*ORGANIZATION_NAME).to_owned(),
            // organization_email: (*ORGANIZATION_EMAIL).to_owned(),
        };

        let html = welcome.render().unwrap();

        self.send_email(from, to, subject, html.as_str())
            .map_err(|err| err.into())
    }
}
