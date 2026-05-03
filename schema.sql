CREATE TABLE IF NOT EXISTS link (
    id SERIAL PRIMARY KEY,
    short_code VARCHAR(10) NOT NULL UNIQUE,
    long_url TEXT NOT NULL,
    clicks INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP

);

CREAT INDEX IF NOT EXISTS idx_short_code ON links(short_code);
