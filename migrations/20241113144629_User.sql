-- Add migration script here
CREATE TABLE IF NOT EXISTS `User` (
    id INT PRIMARY KEY,
    username VARCHAR ( 50 ) NOT NULL,
    email VARCHAR ( 100 ) UNIQUE,
    password VARCHAR ( 255 ) NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
  )