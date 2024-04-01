-- Add migration script here

create table chats
(
    id            uuid                     default gen_random_uuid() not null
        primary key,
    created_at    timestamp with time zone default now()             not null,
    updated_at    timestamp with time zone default now()             not null,
    owner_id      uuid                                               not null
        references members
            on update cascade on delete set null,
    resource_id          uuid                                        not null,
    resource_type        text                                        not null,
    status               varchar
);

create trigger set_public_chats_updated_at
    before update
    on chats
    for each row
execute procedure set_current_timestamp_updated_at();

comment on trigger set_public_chats_updated_at on chats is 'trigger to set value of column "updated_at" to current timestamp on row update';

create table messages
(
    id            uuid                     default gen_random_uuid() not null
        primary key,
    created_at    timestamp with time zone default now()             not null,
    updated_at    timestamp with time zone default now()             not null,
    owner_id      uuid                                               not null
        references members
            on update cascade on delete set null,
    chat_id      uuid                                                not null
        references chats
            on update cascade on delete set null,
    parent_id            uuid                                        not null
        references messages
            on update cascade on delete set null,
    resource_id          uuid                                        not null,
    resource_type        text                                        not null,
    content              text                                        not null,
    status               varchar
);

create trigger set_public_messages_updated_at
    before update
    on messages
    for each row
execute procedure set_current_timestamp_updated_at();

comment on trigger set_public_messages_updated_at on messages is 'trigger to set value of column "updated_at" to current timestamp on row update';