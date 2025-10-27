DROP TABLE short_urls;
CREATE TABLE IF NOT EXISTS short_urls (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  short_code varchar(10) NOT NULL UNIQUE,
  target_url text NOT NULL,
  clicks bigint NOT NULL DEFAULT 0,
  created_at timestamptz NOT NULL DEFAULT now(),
  expires_at timestamptz
);

CREATE INDEX IF NOT EXISTS idx_short_urls_code ON short_urls(short_code);
