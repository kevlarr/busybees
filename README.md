# Busybees

Hobby blog engine including basic WYSIWYG editor & image uploads/processing.

This project is written in Rust because the language offers [more than just performance](https://blog.rocketinsights.com/rust-is-more-than-performance/)
and is a great choice for server-side applications,
with its modern feature set, fantastic developer experience, and ease of deployment.

Beyond raw performance, Rust offers...

- ... wonderful abstractions possible from the combination of structs, traits, and enums
- ... immutability by default, (controlled and explicit) mutability when desired
- ... the requirement for me to *rigourously handle* all possible errors (or explicitly panic on them)
- ... a macro system that makes compile-time checks on HTML/SQL possible
- ... no null-pointer exceptions
- ... worry-free, application-wide refactoring

And, honestly, it's about the community, too - few seem so friendly, supportive, and humble.

## Features

### Bookmark-friendly article links

Posts are served on routes like `/posts/xLoGbVFoZasq/read/the-article-title-in-url-here`.

The goal was to have the post title serialized into the URL, but if it was ever changed
then the URL would change, too, and if I'm lucky enough to have someone bookmark my links,
then their links would break.

Hence the random id, eg. `xLoGbVFoZasq`.
The server actually doesn't care what the title is in the URL - one could visit
`/posts/xLoGbVFoZasq/read/blarg` or `/posts/xLoGbVFoZasq/read/you-are-the-very-model-of-a-modern-major-general`
for all that matters and the post would still load.

Best of both worlds: the title is shown in the URL because it's user-friendly, but the pages
are loaded by the random id because it's server (and bookmark) friendly.

### Compiler-verified SQL

Using the `sqlx::query!` macro, I can write plain SQL (yay!) that is also verified **at compile time** against an actual database.
Combined with trivially-easy serialization to user-defined structs, interacting with the database is pure joy.


### Compiler-verified HTML

HTML is generated via [horrorshow](https://docs.rs/horrorshow/0.8.3/horrorshow/),
which (like using `sqlx`) means that HTML is guaranteed to be valid at compile time due to the `html!` macro expansion,
unlike using traditional `.html` templates that aren't really validated until you see it in the browser.

Plus...

```rust
let title = "Some title";

html! {
    div {
        h1 : title;
        div {
          h2 : "A subtitle";
          p : "Some text";
        }
    }
}
```
... feels easier to visually parse (and write and edit) than...

```html
<div>
  <h1>{{title}}</h1>
  <div>
    <h2>A subtitle</h2>
    <p>Some text</p>
  </div>
</div>
```

### JS/CSS cache-busting without new filenames

JS and CSS files are served on paths like `/assets/1588378111/app.css`.
These do not correspond to actual paths, though, as the server will discard the timestamp
and instead load `/assets/app.css`. The paths will only change upon server restarts.

A big advantage to this is that the files themselves do not need to be 'built' or have
hashes appended to them, HTML files do not need to be updated to match the new asset hashes, etc.

Everything happens at **run time**: the server generates HTML that links to these
virtual paths and then requests for them get stripped and matched to actual paths.
There are no build systems to coordinate and there is a single source of truth.

**Note:** This is primarily only an advantage when not already using a preprocessor like SCSS.

## Setup

### File uploads

You will need to make a directory to store uploaded images.
(While a real blog engine should use external storage like S3,
this server was meant to run as simply - and cheaply - as possible
and stores files to disk.)

Examples will use an `./uploads/` directory in the same location as the server.

```bash
mkdir uploads
```

### Database

The application relies on `pgcrypto` to generate random post keys.

```sql
create schema exts;
create extension pgcrypto with schema exts;
```

### Run the server

You will need to populate a `.env` file with the following variables,
substituting them with whatever values match your desired setup:

| | |
|-|-|
|ADDRESS|localhost:3000|
|DATABASE_URL|postgresql://user:password@host:port/dbname|
HASH_SECRET|'some secret here >= 32 bytes'|
UPLOAD_PATH|uploads/|

With your `.env` populated, you should be good to build and run the server:

```bash
cargo run -q --bin server
```

Visiting localhost:3000 at this point should show you a basic page with no posts - how exciting!

#### Add an admin user

Since the blog was meant only for my wife and I to write on, there is no
"user sign-up" flow.
Currently, users must be inserted into the database directly.

First, hash your password:

```bash
cargo run -q --bin encrypter 'my super secret password here'

$argon2id$v...snip...Tt5err5U
```

Then, insert into the database:

```sql
insert into author (name, email, password_hash) values
    ('your name', 'your.email@example.com', '$argon2id$v...snip...Tt5err5U');
```

Finally, log in by visiting the `/auth` route.
(There are no links to log in on the page - again, only the two of us do, so login shouldn't be a visible route.)
