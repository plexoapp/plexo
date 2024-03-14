-- create table _sqlx_migrations
-- (
--     version        bigint                                 not null
--         primary key,
--     description    text                                   not null,
--     installed_on   timestamp with time zone default now() not null,
--     success        boolean                                not null,
--     checksum       bytea                                  not null,
--     execution_time bigint                                 not null
-- );

-- alter table _sqlx_migrations
--     owner to bregydoc;

create table labels
(
    id          uuid                     default gen_random_uuid() not null
        primary key,
    created_at  timestamp with time zone default now()             not null,
    updated_at  timestamp with time zone default now()             not null,
    name        text                                               not null
        unique,
    description text,
    color       varchar,
    owner_id    uuid                                               not null
);

-- alter table labels
--     owner to bregydoc;

create table labels_by_tasks
(
    label_id uuid not null,
    task_id  uuid not null,
    primary key (label_id, task_id)
);

-- alter table labels_by_tasks
--     owner to bregydoc;

create table members
(
    id            uuid                     default gen_random_uuid() not null
        primary key,
    created_at    timestamp with time zone default now()             not null,
    updated_at    timestamp with time zone default now()             not null,
    name          text                                               not null,
    email         varchar                                            not null,
    password_hash varchar,
    github_id     varchar
        unique,
    google_id     varchar
        unique,
    photo_url     varchar,
    role          varchar
);

-- alter table members
--     owner to bregydoc;

create table members_by_projects
(
    member_id  uuid not null,
    project_id uuid not null,
    primary key (member_id, project_id)
);

-- alter table members_by_projects
--     owner to bregydoc;

create table members_by_teams
(
    team_id   uuid not null,
    member_id uuid not null,
    role      varchar default 'Member'::character varying,
    primary key (team_id, member_id)
);

-- alter table members_by_teams
--     owner to bregydoc;

create table projects
(
    id          uuid                     default gen_random_uuid() not null
        primary key,
    created_at  timestamp with time zone default now()             not null,
    updated_at  timestamp with time zone default now()             not null,
    name        text                                               not null,
    prefix      varchar,
    owner_id    uuid                                               not null
        references members
            on update cascade on delete set null,
    description text,
    lead_id     uuid,
    start_date  timestamp with time zone,
    due_date    timestamp with time zone,
    status      varchar,
    visibility  varchar
);

-- alter table projects
--     owner to bregydoc;

create table self
(
    id         uuid                     default gen_random_uuid() not null
        primary key,
    created_at timestamp with time zone default now()             not null,
    updated_at timestamp with time zone default now()             not null,
    name       text                                               not null
);

-- alter table self
--     owner to bregydoc;

create table tasks
(
    id          uuid                     default gen_random_uuid() not null
        primary key,
    created_at  timestamp with time zone default now()             not null,
    updated_at  timestamp with time zone default now()             not null,
    title       text                                               not null,
    description text,
    owner_id    uuid                                               not null
        references members
            on update cascade on delete set null,
    status      varchar,
    priority    varchar,
    due_date    timestamp with time zone,
    project_id  uuid,
    lead_id     uuid,
    labels      jsonb,
    count       serial,
    parent_id   uuid
);

-- alter table tasks
--     owner to bregydoc;

create table tasks_by_assignees
(
    task_id     uuid not null
        references tasks
            on update cascade on delete cascade,
    assignee_id uuid not null
        references members
            on update cascade on delete cascade,
    primary key (task_id, assignee_id)
);

-- alter table tasks_by_assignees
--     owner to bregydoc;

create table tasks_by_projects
(
    task_id    uuid not null
        constraint tasks_by_projects_task_fkey
            references tasks
            on update cascade on delete cascade,
    project_id uuid not null
        constraint tasks_by_projects_project_fkey
            references projects
            on update cascade on delete cascade,
    primary key (task_id, project_id)
);

-- alter table tasks_by_projects
--     owner to bregydoc;

create table teams
(
    id         uuid                     default gen_random_uuid() not null
        primary key,
    created_at timestamp with time zone default now()             not null,
    updated_at timestamp with time zone default now()             not null,
    name       varchar                                            not null,
    owner_id   uuid                                               not null,
    visibility varchar,
    prefix     text
        unique
);

-- alter table teams
--     owner to bregydoc;

create table teams_by_projects
(
    team_id    uuid not null,
    project_id uuid not null,
    primary key (team_id, project_id)
);

-- alter table teams_by_projects
--     owner to bregydoc;

create table activity
(
    id            uuid                     default gen_random_uuid() not null
        primary key,
    created_at    timestamp with time zone default now()             not null,
    updated_at    timestamp with time zone default now()             not null,
    member_id     uuid                                               not null
        references members
            on delete cascade,
    resource_id   uuid                                               not null,
    operation     text                                               not null,
    resource_type text                                               not null
);

-- alter table activity
--     owner to bregydoc;

create index activity_member_id_idx
    on activity (member_id);

create index activity_resource_id_idx
    on activity (resource_id);

create table assets
(
    id         uuid                     default gen_random_uuid() not null
        primary key,
    created_at timestamp with time zone default now()             not null,
    updated_at timestamp with time zone default now()             not null,
    name       text                                               not null,
    owner_id   uuid                                               not null
        references members
            on update cascade on delete set null,
    kind       varchar,
    project_id uuid
);

-- alter table assets
--     owner to bregydoc;

create table changes
(
    id            uuid                     default gen_random_uuid() not null
        primary key,
    created_at    timestamp with time zone default now()             not null,
    updated_at    timestamp with time zone default now()             not null,
    owner_id      uuid                                               not null
        references members
            on delete cascade,
    resource_id   uuid                                               not null,
    operation     text                                               not null,
    resource_type text                                               not null,
    diff_json     text                                               not null
);

-- alter table changes
--     owner to bregydoc;

create index changes_member_id_idx
    on changes (owner_id);

create index changes_resource_id_idx
    on changes (resource_id);

create function set_current_timestamp_updated_at() returns trigger
    language plpgsql
as
$$
DECLARE
  _new record;
BEGIN
  _new := NEW;
  _new."updated_at" = NOW();
  RETURN _new;
END;
$$;

ALTER FUNCTION set_current_timestamp_updated_at() OWNER TO CURRENT_USER;

create trigger set_public_labels_updated_at
    before update
    on labels
    for each row
execute procedure set_current_timestamp_updated_at();

comment on trigger set_public_labels_updated_at on labels is 'trigger to set value of column "updated_at" to current timestamp on row update';

create trigger set_public_members_updated_at
    before update
    on members
    for each row
execute procedure set_current_timestamp_updated_at();

comment on trigger set_public_members_updated_at on members is 'trigger to set value of column "updated_at" to current timestamp on row update';

create trigger set_public_projects_updated_at
    before update
    on projects
    for each row
execute procedure set_current_timestamp_updated_at();

comment on trigger set_public_projects_updated_at on projects is 'trigger to set value of column "updated_at" to current timestamp on row update';

create trigger set_public_self_updated_at
    before update
    on self
    for each row
execute procedure set_current_timestamp_updated_at();

comment on trigger set_public_self_updated_at on self is 'trigger to set value of column "updated_at" to current timestamp on row update';

create trigger set_public_tasks_updated_at
    before update
    on tasks
    for each row
execute procedure set_current_timestamp_updated_at();

comment on trigger set_public_tasks_updated_at on tasks is 'trigger to set value of column "updated_at" to current timestamp on row update';

create trigger set_public_teams_updated_at
    before update
    on teams
    for each row
execute procedure set_current_timestamp_updated_at();

comment on trigger set_public_teams_updated_at on teams is 'trigger to set value of column "updated_at" to current timestamp on row update';

create trigger set_public_assets_updated_at
    before update
    on assets
    for each row
execute procedure set_current_timestamp_updated_at();

