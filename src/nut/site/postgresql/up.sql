CREATE TABLE cards(
  id BIGSERIAL PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  body TEXT NOT NULL,
  media_type VARCHAR(8) NOT NULL,
  action VARCHAR(32) NOT NULL,
  href VARCHAR(255) NOT NULL,
  logo VARCHAR(255) NOT NULL,
  loc VARCHAR(16) NOT NULL,
  lang VARCHAR(8) NOT NULL,
  position SMALLINT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL
);
CREATE INDEX idx_cards_lang ON cards(lang);
CREATE INDEX idx_cards_loc ON cards(loc);

CREATE TABLE links(
  id BIGSERIAL PRIMARY KEY,
  href VARCHAR(255) NOT NULL,
  label VARCHAR(255) NOT NULL,
  loc VARCHAR(16) NOT NULL,
  lang VARCHAR(8) NOT NULL,
  x SMALLINT NOT NULL,
  y SMALLINT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL
);
CREATE INDEX idx_links_lang ON links(lang);
CREATE INDEX idx_links_loc ON links(loc);

CREATE TABLE friend_links(
  id BIGSERIAL PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  home VARCHAR(255) NOT NULL,
  logo VARCHAR(255) NOT NULL,
  lang VARCHAR(8) NOT NULL,
  position SMALLINT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL
);
CREATE INDEX idx_friend_links_lang ON friend_links(lang);

CREATE TABLE leave_words(
  id BIGSERIAL PRIMARY KEY,
  body TEXT NOT NULL,
  media_type VARCHAR(8) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL
);

CREATE TABLE votes(
  id BIGSERIAL PRIMARY KEY,
  point BIGINT NOT NULL,
  resource_type VARCHAR(255) NOT NULL,
  resource_id BIGINT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL
);
CREATE INDEX idx_votes_resource_type ON votes(resource_type);
CREATE UNIQUE INDEX idx_votes_resource ON votes(resource_type, resource_id);
