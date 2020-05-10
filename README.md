# busybees

Server for [Busy Bee Life](https://www.busybee.life).

## Features

### 100% tracker and (effectively) cookie free

No Google, no Facebook. Zero trackers.

The server *does* set a cookie by default, but it is 100% emmpty for anyone who views the page
except for myself and Stacey.

**Privacy** is under-rated on the web. I respect yours.

### Bookmark-friendly article links

Posts are served on routes like `/posts/xLoGbVFoZasq/read/the-article-title-in-url-here`.

The goal was to have the post title serialized into the URL, but if it was ever changed
then the URL would change, too, and if I'm lucky enough to have someone bookmark my links,
then their links would break.

Hence the `xLoGbVFoZasq` portion. This is a random id assigned on creation and never changed.
The server actually doesn't care what the title is in the URL - one could visit
`/posts/xLoGbVFoZasq/read/blarg` or `/posts/xLoGbVFoZasq/read/you-are-the-very-model-of-a-modern-major-general`
for all that matters and the post would still load.

Best of both worlds: the title is shown in the URL because it's user-friendly, but the pages
are loaded by the random id because it's server (and bookmark) friendly.

### Basic, internal analytics

Very basic page analytics are gathered per request - path visited, referer (if any), timestamp, and IP address.
It's imperfect and faulty, but it beats cookies, fingerprinting, etc. and it's really all I need.

On storing IP addresses, from https://www.pinsentmasons.com/out-law/guides/ip-addresses-and-the-data-protection-act :

> An IP address in isolation is not personal data under the Data Protection Act, according to the Information Commissioner.
> But an IP address can become personal data when combined with other information or when used to build a profile of an individual,
> even if that individual's name is unknown

**IP addresses gathered for basic analytics are not combined with other, more personal information
and they are not used to build profiles of individuals.**
(Nothing is used to profile readers; we don't profile readers.)

### Compile-time guarantees

#### SQL

Using the `sqlx::query!` macro, I can write plain SQL (yay!) that is also verified **at compile time** against an actual database. Combined with trivially-easy serialization to user-defined structs... pure joy.

I used to like ORMs and query builders. But the weeks of my life spent trying to learn some cryptic set of poorly documented methods to accomplish something non-trivial, well, I would love to get them back. Plain SQL means that won't happen anymore.

Plus, no runtime errors from bad SQL statements.

#### HTML

`busybees` uses [horrorshow](https://docs.rs/horrorshow/0.8.3/horrorshow/) to generate HTML responses.
This effectively means that HTML is guaranteed to be valid at compile time due to the `html!` macro expansion,
unlike using traditional `.html` templates that aren't really validated until you see it in the browser.

Plus...

```rust
html! {
    dl {
        div { dt : "Stacey"; dd : "Attorney"; }
        div { dt : "Kevin"; dd : "Software Engineer"; }
    }
}
```
... isn't Haml, but it *is* so much easier to visually parse (and write and edit) than...

```html
<dl>
    <div>
        <dt>Stacey</dt><dd>Attorney</dd>
    </div>
    <div>
        <dt>Kevin</dt><dd>Software Engineer</dd>
    </div>
</dl>
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

## Setup

### HTTPS

Server requires HTTPS even locally, but a test certificate can be generated with `gencert.sh`.

### Database

The application relies on `pgcrypto` to generate random post keys.

- `create schema exts;`
- `create extensions pgcrypto with schema exts;`

## So, why Rust?

I know a lot of people say Rust is ridiculous for web development because "we don't need that level of performance". That's a fair viewpoint -
it's also a complicated language,
the compiler is harder to please than a truckload of dads,
and it can be tough to build the same kind of 'feature momentum' compared to using common web languages.

Well, *"Butts to that,"* I say - Rust is perfect for the web.
This started as a proof-of-concept, and the experience continues to be enjoyable.

**10/10 would choose again.**

### 1. we should **always** strive for 'that level of performance'

I'm busy, you're busy, most of us are busy, busy bees.
We have to wait far too long when viewing far too many sites, both from server speed and the bloat (and blight) of modern JS-heavy "apps". Between bundle fetching, parsing, network requests to build a data 'store' on the client-side, and DOM manipulation, modern apps have a lot of overhead.

We deserve better, and it's sad that we (as users) have become used to slow, bloated, memory-hogging software
despite having radically powerful hardware.

### 2. decreasing cpu and memory loads is more sustainable

All of these contribute to less energy and a lower environmental footprint.
(Because, yes, I care about that. **Small improvements are still improvements.**)

And it's about giving someone fewer of my dollars for virtual hosting.
Substantially less memory and marginally less computation time means less I have to pay.

### 3. server-rendered HTML is still good

Server-rendered HTML with minimal JavaScript is often considered antiquated or even an anti-pattern, and that's sad.
HTML is great for content-based sites, and most of the Web is content-based.

This server, rendering HTML on every new path, is still **significantly faster** than most of the load-time heavy SPAs out there. HTML size is about as small as JSON requests that would need to happen on every page,
and rendering server-side means *no subsequent requests*, unlike SPAs.

**Minimal JavaScript is maximal user experience.**

### 4. rust is more than just performance

Rust gives me some big promises that the vast majority of other languages can't.
Beyond performance, Rust is...

- ... the wonderful abstractions possible from the type/trait system
- ... a macro system that makes compile-time checks on HTML/ SQL DSLs possible
- ... immutability by default, (controlled and explicit) mutability when desired
- ... the impossibility of null-pointer exceptions in safe (ie. normal) code
- ... the requirement for me to *handle* all possible errors
- ... the compiler forcing me to write better code, because bad abstractions *are hard to write* (unlike in most other languages)
- ... knowing that if it compiles, it will run
- ... worry-free application-wide refactoring

And, honestly, it's about the community, too. From my limited experience, it's been fantastic so far.
