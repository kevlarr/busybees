create function first_image(content text) returns text as $$
    select substring(content, 'src="([a-zA-Z0-9\.\-_~:\/%\?#=]+)"');
$$ language sql immutable;
