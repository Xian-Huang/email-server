-- Add migration script here
CREATE TABLE subscriptions IF NOT EXISTS{
    id uuid PRIMARY KEY NOT NULL,
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    subscribed_at TIMESTAMP NOT NULL
}