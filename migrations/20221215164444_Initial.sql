create table users
(
    id          uuid primary key not null default gen_random_uuid(),
    external_id bigint           not null,               -- The id of the user account from the external login service (Which is currently GitHub)
    name        text             not null,               -- The username of this user
    is_admin    boolean          not null default false, -- Whether this user is an administrator of the system.
    is_approved boolean          not null default false  -- Whether this used is approved by an administrator to use this application
);
create unique index users_external_id_uindex on users (external_id);
comment on table users is 'The users of the timetable application';
comment on column users.external_id is 'The id of the user account from the external login service (Which is currently GitHub)';
comment on column users.name is 'The username of this user';
comment on column users.is_admin is 'Whether this user is an administrator of the system.';
comment on column users.is_approved is 'Whether this used is approved by an administrator to use this application';

create table time_tables
(
    id       uuid primary key not null default gen_random_uuid(),
    owner_id uuid             not null, -- The user who owns this time table
    name     text             not null, -- The name of this time table
    foreign key (owner_id) references users (id)
        on delete cascade
);
comment on table time_tables is 'User created time tables';
comment on column time_tables.owner_id is 'The user who owns this time table';
comment on column time_tables.name is 'The name of this time table';

-- Time Table permissions
create table time_tables_permissions
(
    time_table_id uuid    not null,               -- The time table
    user_id       uuid    not null,               -- The user who got extra permissions
    can_edit      boolean not null default false, -- Whether the user can edit the time table or not
    primary key (time_table_id, user_id),
    foreign key (time_table_id) references time_tables (id)
        on delete cascade,
    foreign key (user_id) references users (id)
        on delete cascade
);
comment on table time_tables_permissions is 'External access permissions for time tables';
comment on column time_tables_permissions.time_table_id is 'The time table';
comment on column time_tables_permissions.user_id is 'The user who got extra permissions';
comment on column time_tables_permissions.can_edit is 'Whether the user can edit the time table or not';

create type time_table_entry_type as enum ('RECURRING', 'ONE_TIME');

-- Entries
create table time_table_entries
(
    id                  uuid primary key      not null default gen_random_uuid(), -- The id of this entry
    time_table_id       uuid                  not null,                           -- The time table
    type                time_table_entry_type not null,                           -- The type of this time table entry
    author_id           uuid                  not null,                           -- The user who created/last edited this entry
    duration            interval              not null,                           -- The duration of this event
    start_date          date                  not null,                           -- The starting date of this event (inclusive)
    end_date            date                  not null,                           -- End date of this entry (inclusive)
    recurrence_interval interval,                                                 -- The interval of recurrence of this event (null when ONE_TIME)
    start_time          time with time zone   not null,                           -- The start time of this event
    foreign key (time_table_id) references time_tables (id)
        on delete cascade,
    foreign key (author_id) references users (id)
        on delete cascade
);
create index time_table_entries_time_table_id_index on time_table_entries (time_table_id);
comment on table time_table_entries is 'The entries of a specific time table';
comment on column time_table_entries.id is 'The id of this entry';
comment on column time_table_entries.time_table_id is 'The time table';
comment on column time_table_entries.type is 'The type of this time table entry';
comment on column time_table_entries.author_id is 'The user who created/last edited this entry';
comment on column time_table_entries.duration is 'The duration of this event';
comment on column time_table_entries.start_date is 'The starting date of this event (inclusive)';
comment on column time_table_entries.end_date is 'End date of this entry (inclusive)';
comment on column time_table_entries.recurrence_interval is 'The interval of recurrence of this event (null when ONE_TIME)';
comment on column time_table_entries.start_time is 'The start time of this event';

-- Entry change history
create table time_table_entries_history
(
    time_table_entry_id uuid                     not null,
    time                timestamp with time zone not null default now(), -- The time of this edit
    old_record          jsonb                    not null,               -- The old table entry data
    author_id           uuid                     not null,               -- The user who authored this change
    foreign key (time_table_entry_id) references time_table_entries (id)
        on delete cascade,
    foreign key (author_id) references users (id)
        on delete cascade
);
comment on table time_table_entries_history is 'The history of time table entry edits';
comment on column time_table_entries_history.time is 'The time of this edit';
comment on column time_table_entries_history.old_record is 'The old table entry data';
comment on column time_table_entries_history.author_id is 'The user who authored this change';

