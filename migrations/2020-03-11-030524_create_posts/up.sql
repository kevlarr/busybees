create table post (
    id serial primary key,
    key text unique not null check (length(key) = 12),
    title text not null check (length(title) > 0 and length(title) <= 128),
    content text not null check (length(content) > 0),
    published boolean default false,
    created_at timestamptz not null,
    updated_at timestamptz not null,

    check(created_at <= updated_at)
);

create function gen_random_key(int) returns text as
$$
declare
    key text;

begin
    key := encode(exts.gen_random_bytes($1 * 2), 'base64');
    key := replace(key, '+', 'A');
    key := replace(key, '/', 'l');
    key := replace(key, '=', '5');

    return substring(key from 0 for $1 + 1);
end;
$$ language plpgsql;


create function gen_unique_key_trigger() returns trigger as
$$
declare
    counter int := 0;
    key text;
    query text;
    found text;

begin
    query := 'select key from ' || quote_ident(tg_table_name) || ' where key=';

    -- Looping 5 times should absolutely be enough to avoid conflicts,
    -- unless a meteor strikes our basement.
    loop
        exit when counter = 5;

        key := gen_random_key(12);

        execute query || quote_literal(key) into found;

        if found is null then
            exit;
        end if;

        counter := counter + 1;
    end loop;

    new.key = key;
    return new;
end;
$$ language plpgsql;


create trigger gen_post_key
    before insert on post
    for each row execute
        procedure gen_unique_key_trigger();
