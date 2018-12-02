CREATE TABLE users(
  id BIGINT NOT NULL AUTO_INCREMENT,
  real_name VARCHAR(32) NOT NULL,
  nick_name VARCHAR(32) NOT NULL,
  email VARCHAR(255) NOT NULL,
  password BLOB,
  uid VARCHAR(36) NOT NULL,
  provider_type VARCHAR(16) NOT NULL,
  provider_id VARCHAR(255) NOT NULL,
  logo VARCHAR(255) NOT NULL,
  sign_in_count BIGINT NOT NULL,
  current_sign_in_at DATETIME,
  current_sign_in_ip VARCHAR(39),
  last_sign_in_at DATETIME,
  last_sign_in_ip VARCHAR(39),
  confirmed_at DATETIME,
  locked_at DATETIME,
  deleted_at DATETIME,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL
);
CREATE INDEX idx_users_real_name ON users(real_name);
CREATE UNIQUE INDEX idx_users_nick_name ON users(nick_name);
CREATE UNIQUE INDEX idx_users_email ON users(email);
CREATE UNIQUE INDEX idx_users_uid ON users(uid);
CREATE UNIQUE INDEX idx_users_provider ON users(provider_type, provider_id);

CREATE TABLE logs(
  id BIGINT NOT NULL AUTO_INCREMENT,
  user_id BIGINT NOT NULL,
  ip VARCHAR(39) NOT NULL,
  message VARCHAR(255) NOT NULL,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE policies(
  id BIGINT NOT NULL AUTO_INCREMENT,
  user_id BIGINT NOT NULL,
  role VARCHAR(32) NOT NULL,
  resource_name VARCHAR(255),
  resource_id BIGINT,
  nbf DATE NOT NULL,
  exp DATE NOT NULL,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL
);
CREATE INDEX idx_policies_role ON policies(role);

CREATE TABLE attachments(
  id BIGINT NOT NULL AUTO_INCREMENT,
  user_id BIGINT NOT NULL,
  title VARCHAR(255) NOT NULL,
  size BIGINT NOT NULL,
  mime_type VARCHAR(255) NOT NULL,
  url VARCHAR(255) NOT NULL,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL
);
CREATE INDEX idx_attachments ON attachments(title);

CREATE TABLE notifications(
  id BIGINT NOT NULL AUTO_INCREMENT,
  user_id BIGINT NOT NULL,
  url VARCHAR(255) NOT NULL,
  body TEXT NOT NULL,
  media_type VARCHAR(8) NOT NULL,
  level VARCHAR(8) NOT NULL,
  read BOOLEAN NOT NULL,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL
);
CREATE INDEX idx_notifications ON notifications(level);
