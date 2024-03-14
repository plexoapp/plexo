create table organization
(
    id            uuid                     default gen_random_uuid() not null
        primary key,
    created_at    timestamp with time zone default now()             not null,
    updated_at    timestamp with time zone default now()             not null,
    owner_id      uuid                                               not null
        references members
            on update cascade on delete set null,
    name          text                                               not null,
    value         text                                               not null
);

create trigger set_public_organization_updated_at
    before update
    on organization
    for each row
execute procedure set_current_timestamp_updated_at();

comment on trigger set_public_organization_updated_at on organization is 'trigger to set value of column "updated_at" to current timestamp on row update';