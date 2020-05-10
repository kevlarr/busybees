create table page_view (
    id serial primary key,
    viewed_at timestamptz not null default (now() at time zone 'utc'),
    pathname text not null,
    ip text,
    referer text
);

create index page_view_viewed_at_idx
    on page_view using brin (viewed_at);
