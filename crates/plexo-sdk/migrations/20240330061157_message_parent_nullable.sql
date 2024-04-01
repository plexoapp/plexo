-- Add migration script here
ALTER TABLE messages ALTER COLUMN parent_id DROP NOT NULL;