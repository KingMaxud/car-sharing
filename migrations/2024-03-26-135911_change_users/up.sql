ALTER TABLE users
ADD COLUMN status       VARCHAR(20) DEFAULT 'active',
ADD COLUMN telegram_id  integer NOT NULL;
