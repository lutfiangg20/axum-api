-- Add migration script here
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    price INT NOT NULL DEFAULT 0,
    stock INT NOT NULL DEFAULT 0,
    category_id INT NOT NULL 
        REFERENCES categories(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
