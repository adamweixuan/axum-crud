-- Add migration script here
CREATE TABLE IF NOT EXISTS userinfos
(
    id      INT PRIMARY KEY     NOT NULL AUTO_INCREMENT,
    name    TEXT                NOT NULL,
    age     INT                 NOT NULL,
    email   VARCHAR(255) UNIQUE NOT NULL,
    address TEXT                NOT NULL
);