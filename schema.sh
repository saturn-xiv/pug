#!/bin/sh

set -e

cargo test --test orm_test -- --nocapture
diesel print-schema --database-url "postgres://postgres:@localhost:5432/pug" -o -- schema_migrations > src/orm/schema/postgresql/schema.rs
diesel print-schema --database-url "postgres://postgres:@localhost:5432/pug" -o -- settings > src/settings/postgresql/schema.rs
diesel print-schema --database-url "postgres://postgres:@localhost:5432/pug" -o -- locales > src/i18n/db/postgresql/schema.rs
diesel print-schema --database-url "postgres://postgres:@localhost:5432/pug" -o -- users logs policies attachments notifications > src/nut/auth/postgresql/schema.rs
diesel print-schema --database-url "postgres://postgres:@localhost:5432/pug" -o -- cards links friend_links leave_words votes tags categories > src/nut/site/postgresql/schema.rs

cargo test --test orm_test --no-default-features --features "mysql" -- --nocapture
diesel print-schema --database-url "mysql://root:@localhost:3306/pug" -o -- schema_migrations > src/orm/schema/mysql/schema.rs
diesel print-schema --database-url "mysql://root:@localhost:3306/pug" -o -- settings > src/settings/mysql/schema.rs
diesel print-schema --database-url "mysql://root:@localhost:3306/pug" -o -- locales > src/i18n/db/mysql/schema.rs
diesel print-schema --database-url "mysql://root:@localhost:3306/pug" -o -- users logs policies attachments notifications > src/nut/auth/mysql/schema.rs
diesel print-schema --database-url "mysql://root:@localhost:3306/pug" -o -- cards links friend_links leave_words votes tags categories > src/nut/site/mysql/schema.rs

cargo test --test orm_test --no-default-features --features "sqlite" -- --nocapture
diesel print-schema --database-url "tmp/db" -o -- schema_migrations > src/orm/schema/sqlite/schema.rs
diesel print-schema --database-url "tmp/db" -o -- settings > src/settings/sqlite/schema.rs
diesel print-schema --database-url "tmp/db" -o -- locales > src/i18n/db/sqlite/schema.rs
diesel print-schema --database-url "tmp/db" -o -- users logs policies attachments notifications > src/nut/auth/sqlite/schema.rs
diesel print-schema --database-url "tmp/db" -o -- cards links friend_links leave_words votes tags categories > src/nut/site/sqlite/schema.rs
