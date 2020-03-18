# busybees

Server to support the (upcoming) blog at http://busybee.life

## Why Rust?

I know a lot of people say Rust is ridiculous for web development because "we don't need that level of performance".
My response would be...

### 1. We should **always** strive for that level of performance

I'm busy, you're busy, most of us are busy - we are all busy bees.
We don't have time to wait for the slow-butt servers that infest the modern web.

We deserve better, and it's sad that we (as users) have become used to slow, bloated, memory-hogging software
despite having radically powerful hardware.

### 2. Lightening cpu and memory loads as well as decreasing processing time

All of these contribute to less energy and a lower environmental footprint.
(Because, yes, I care about that. **Small improvements are still improvements.**)

And it's about giving someone fewer of my dollars to for virtual hosting.
Less memory required means less I have to pay.

### 3. Server-rendered HTML is still good

Server-rendered HTML with minimal JavaScript is often considered antiquated. And that's sad.

This server, rendering HTML on every new path, is still **significantly faster** than most of the load-time heavy SPAs out there
that then need to make dozens of slow requests to some slow API to render a page.

*Minimal JavaScript is maximal user experience.*

### 4. It's more than just performance

Rust gives me some big promises that the vast majority of other languages can't.

It isn't just performance, it's about the **type and trait system**.

It's about the absolute *impossibility of null-pointer exceptions*,
and it's about me being *forced to write better code*,
because it's *hard* to write the kind of garbage abstractions we so quickly throw together in other languages.

It's about **the compiler** that, while slow and very mean, forces me to be a better programmer.
It's about knowing that *if it compiles, it will run*.

And, honestly, it's about the community, too.
