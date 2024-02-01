CREATE TABLE service_checks (
    id              SERIAL PRIMARY KEY,
    team            TEXT NOT NULL,
    vm              TEXT NOT NULL, 
    service         TEXT NOT NULL,

    up              BOOLEAN NOT NULL,
    short_error     TEXT,
    long_error      TEXT,
    time            TIMESTAMPTZ NOT NULL
);

CREATE TABLE team_snapshots (
    id              SERIAL PRIMARY KEY,
    team            TEXT NOT NULL,

    points          INT NOT NULL,
    services        JSONB NOT NULL,
    time            TIMESTAMPTZ NOT NULL
);

CREATE TABLE sla_violations (
    id              SERIAL PRIMARY KEY,
    team            TEXT NOT NULL,
    vm              TEXT NOT NULL,
    service         TEXT NOT NULL,

    time            TIMESTAMPTZ NOT NULL
);
