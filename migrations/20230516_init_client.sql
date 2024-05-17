CREATE TABLE clients (
    id BIGSERIAL PRIMARY KEY,
    vendors TEXT[] NOT NULL,
    sftp TEXT NOT NULL,
    name TEXT NOT NULL,
    email TEXT NOT NULL
);
