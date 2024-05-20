-- Create clients table
CREATE TABLE clients (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    bucket TEXT NOT NULL
);

-- Create vendors table with a foreign key to clients
CREATE TABLE vendors (
    id BIGSERIAL PRIMARY KEY,
    client_id BIGINT NOT NULL REFERENCES clients(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    host TEXT NOT NULL,
    port INTEGER NOT NULL,
    username TEXT,
    password TEXT,
    ssh_key TEXT,
    ssh_key_password TEXT
);

-- Create sftp table with a foreign key to clients
CREATE TABLE sftp (
    id BIGSERIAL PRIMARY KEY,
    client_id BIGINT NOT NULL REFERENCES clients(id) ON DELETE CASCADE,
    username TEXT NOT NULL,
    private_key TEXT NOT NULL,
    public_key TEXT NOT NULL,
    bucket_name TEXT NOT NULL,
    aws_role_arn TEXT NOT NULL
);

CREATE TABLE agents (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL
);

-- Create agent_clients table to represent many-to-many relationship
CREATE TABLE agent_clients (
    agent_id BIGINT NOT NULL REFERENCES agents(id) ON DELETE CASCADE,
    client_id BIGINT NOT NULL REFERENCES clients(id) ON DELETE CASCADE,
    PRIMARY KEY (agent_id, client_id)
);

-- Alter the sequence for clients.id to start from a large 8-digit number
-- ALTER SEQUENCE clients_id_seq RESTART WITH 74830193;
