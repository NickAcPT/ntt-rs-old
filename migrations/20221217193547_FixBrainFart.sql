alter table time_table_entries
    drop column onetime_data_id;

alter table time_table_entry_repeating_data
    add COLUMN weekly_repeating_interval week_day[] not null default array[]::week_day[];

drop table time_table_entry_onetime_data;