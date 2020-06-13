# busybees

Utilities to share across different applications that rely on the same database model,
encryption, image processing, etc.

Also included are common third-party dependencies, eg. `busybees::deps::sqlx`.
Exporting dependencies enables cleaner `Cargo.toml` files as it prevents the need
to specify the same dependencies across applications, which makes it easier for
dependency versions to fall out of sync across different apps.
