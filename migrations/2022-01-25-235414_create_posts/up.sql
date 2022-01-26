CREATE TABLE documents (
    id varchar(255) PRIMARY KEY,
    title varchar(500) NOT NULL,
    body text NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;