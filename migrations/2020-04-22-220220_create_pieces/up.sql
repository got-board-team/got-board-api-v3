CREATE TABLE IF NOT EXISTS pieces (
  id SERIAL PRIMARY KEY,
  match_id INT NOT NULL,
  piece_type VARCHAR NOT NULL,
  x INT NOT NULL,
  y INT NOT NULL,
  location VARCHAR NOT NULL,
  house_name VARCHAR,
  spec JSONB
)
