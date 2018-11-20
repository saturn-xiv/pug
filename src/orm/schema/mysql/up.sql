CREATE TABLE schema_migrations(
  id BIGSERIAL PRIMARY KEY,
  version CHAR(17) NOT NULL,
  name VARCHAR(255) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_schema_migrations ON schema_migrations(version, name);
