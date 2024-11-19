-- Add migration script here
CREATE TABLE IF NOT EXISTS `subscriptions` (
    `id` INT PRIMARY KEY NOT NULL,
    `email` VARCHAR(50) NOT NULL UNIQUE,
    `name` VARCHAR(30) NOT NULL,
    `subscribed_at` TIMESTAMP NOT NULL
);