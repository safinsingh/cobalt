-- The following has been generated with the help of ChatGPT

-- Create the 'teams' table
CREATE TABLE teams (
    id 			SERIAL PRIMARY KEY,
    alias 		TEXT NOT NULL,
    points 		INT NOT NULL,

    UNIQUE 		(alias)
);

-- Create the 'services' table
CREATE TABLE services (
    id 					SERIAL PRIMARY KEY,
    team_id 			INT NOT NULL,
    alias 				TEXT NOT NULL,

    consecutive_downs	INT NOT NULL,
    up 					BOOLEAN NOT NULL,

    short_error			TEXT,
    verbose_error 		TEXT,

    UNIQUE (team_id, alias),
    FOREIGN KEY (team_id) REFERENCES teams (id)
);

-- Create the 'team_snapshots' table
CREATE TABLE team_snapshots (
    id 			SERIAL PRIMARY KEY,
    team_id 	INT NOT NULL,

    points 		INT NOT NULL,
    services 	JSONB NOT NULL,
    time 		TIMESTAMPTZ NOT NULL,

    FOREIGN KEY (team_id) REFERENCES teams (id)
);

-- Create the 'sla_violations' table
CREATE TABLE sla_violations (
    id 			SERIAL PRIMARY KEY,
    team_id 	INT NOT NULL,
    service_id 	INT NOT NULL,
    time 		TIMESTAMPTZ NOT NULL,

    FOREIGN KEY (team_id) REFERENCES teams (id),
    FOREIGN KEY (service_id) REFERENCES services (id)
);
