create type week_day as enum ('MONDAY', 'TUESDAY', 'WEDNESDAY', 'THURSDAY', 'FRIDAY', 'SATURDAY', 'SUNDAY');

alter table time_table_entries
    drop column start_date,
    drop column end_date,
    drop column recurrence_interval;

create table time_table_entry_repeating_data
(
    id               uuid primary key not null default gen_random_uuid(),
    entry_start_date date             not null, -- The date where this event starts happening
    entry_end_date   date             not null  -- The date where this entry stops happening
);
comment on table time_table_entry_repeating_data is 'Repeating data for a time table entry';
comment on column time_table_entry_repeating_data.entry_start_date is 'The date where this event starts happening';
comment on column time_table_entry_repeating_data.entry_end_date is 'The date where this entry stops happening';

