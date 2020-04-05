CREATE TABLE IF NOT EXISTS matches_users (
  id SERIAL PRIMARY KEY,
  match_id INT NOT NULL,
  user_id INT NOT NULL,
  created_at TIMESTAMP NOT NULL
)
