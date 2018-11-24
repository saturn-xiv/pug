CREATE TABLE IF NOT EXISTS schema_migrations(
  id BIGSERIAL PRIMARY KEY,
  version CHAR(17) NOT NULL,
  name VARCHAR(255) NOT NULL,
  up TEXT NOT NULL,
  down TEXT NOT NULL,
  run_at DATETIME
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_schema_migrations ON schema_migrations(version, name);
