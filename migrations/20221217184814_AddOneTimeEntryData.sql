create table public.time_table_entry_onetime_data
(
    id                        uuid primary key not null default gen_random_uuid(),
    weekly_repeating_interval week_day[] -- The week days in which this entry repeats weekly
);
comment on table public.time_table_entry_onetime_data is 'One-time data for a time table entry';
comment on column public.time_table_entry_onetime_data.weekly_repeating_interval is 'The week days in which this entry repeats weekly';

