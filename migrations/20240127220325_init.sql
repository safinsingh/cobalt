-- Add migration script here

-- The following has been generated with the help of ChatGPT

-- Create the 'team_snapshots' table
CREATE TABLE team_snapshots (
    alias VARCHAR(255) NOT NULL,
    points INT NOT NULL,
    time DATETIME NOT NULL
);

-- Create the 'services' table
CREATE TABLE services (
    team VARCHAR(255) NOT NULL,
    service VARCHAR(255) NOT NULL,
    consecutive_downs INT UNSIGNED NOT NULL,
    UNIQUE (team, service)
);

-- Create the 'uptime' table
CREATE TABLE uptime (
    team VARCHAR(255) NOT NULL,
    service VARCHAR(255) NOT NULL,
    up BOOLEAN NOT NULL,
    error VARCHAR(255),
    UNIQUE (team, service)
);

-- Create the 'sla_violations' table
CREATE TABLE sla_violations (
    team VARCHAR(255) NOT NULL,
    service VARCHAR(255) NOT NULL,
    time DATETIME NOT NULL,
    UNIQUE (team, service)
);
