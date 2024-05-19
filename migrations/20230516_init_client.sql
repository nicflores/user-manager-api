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
    email TEXT NOT NULL
);

-- Alter the sequence for clients.id to start from a large 8-digit number
-- ALTER SEQUENCE clients_id_seq RESTART WITH 74830193;
