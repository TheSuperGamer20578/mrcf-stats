CREATE TABLE mr_project_types
(
    id   INTEGER NOT NULL PRIMARY KEY,
    name TEXT    NOT NULL
);
INSERT INTO mr_project_types (id, name)
VALUES (0, 'Mod'),
       (1, 'Shader'),
       (2, 'Modpack'),
       (3, 'Resource Pack');

CREATE TABLE mr_projects
(
    id   VARCHAR(8) NOT NULL PRIMARY KEY,
    name TEXT       NOT NULL,
    type INTEGER    NOT NULL REFERENCES mr_project_types
);

CREATE TABLE mr_versions
(
    id           VARCHAR(8) NOT NULL PRIMARY KEY,
    project      VARCHAR(8) NOT NULL REFERENCES mr_projects,
    name         TEXT       NOT NULL,
    number       TEXT       NOT NULL,
    release_date TIMESTAMP  NOT NULL
);

CREATE TABLE mr_version_downloads
(
    timestamp TIMESTAMP  NOT NULL,
    version   VARCHAR(8) NOT NULL REFERENCES mr_versions,
    downloads INTEGER    NOT NULL,
    PRIMARY KEY (timestamp, version)
);

CREATE TABLE mr_project_downloads
(
    timestamp TIMESTAMP  NOT NULL,
    project   VARCHAR(8) NOT NULL REFERENCES mr_projects,
    downloads INTEGER    NOT NULL,
    PRIMARY KEY (timestamp, project)
);

CREATE TABLE mr_payouts
(
    timestamp TIMESTAMP NOT NULL,
    amount    REAL      NOT NULL
);
