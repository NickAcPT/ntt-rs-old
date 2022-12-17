alter table public.time_table_entries
    add name text not null default 'Untitled';

alter table public.time_table_entries
    add description text;

