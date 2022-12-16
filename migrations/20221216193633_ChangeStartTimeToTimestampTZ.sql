alter table time_table_entries
    drop column start_time,
    add column start_time timestamptz;

comment on column time_table_entries.start_time is 'The start time of this event';