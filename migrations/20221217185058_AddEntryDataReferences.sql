alter table time_table_entries
    add column repeating_data_id uuid references time_table_entry_repeating_data (id) on delete cascade,
    add column onetime_data_id uuid references time_table_entry_onetime_data (id) on delete cascade;