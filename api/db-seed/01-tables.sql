-- ===============================================
-- 01‐tables.sql  (PostgreSQL with INT auto-increment IDs)
-- ===============================================
-- ------------------------------------------------
-- 1) users table
-- ------------------------------------------------
CREATE TABLE users (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    username VARCHAR(64) NOT NULL UNIQUE,
    email VARCHAR(128) NOT NULL
);

-- Separate index for email lookup
CREATE INDEX idx_users_email ON users(email);

-- ------------------------------------------------
-- 2) user_auth table
-- ------------------------------------------------
CREATE TABLE user_auth (
    user_id INT PRIMARY KEY,
    password_hash VARCHAR(255) NOT NULL,
    -- FK to users.id
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- ------------------------------------------------
-- 3) categories table
-- ------------------------------------------------
CREATE TABLE categories (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name VARCHAR(64) NOT NULL UNIQUE
);

-- ------------------------------------------------
-- 4) products table
-- ------------------------------------------------
create table products (
    id int generated always as identity primary key,
    name varchar(64) not null unique,
    description text not null,
    price decimal(10, 2) not null check (price >= 0),
    is_best_seller boolean not null default false,
    is_deal_of_the_day boolean not null default false,
    discount decimal(5, 2) not null default 0 check (
        discount >= 0
        and discount <= 100
    ),
    category_id int not null,
    foreign key (category_id) references categories(id) on delete cascade
);

-- Separate index for category lookup
CREATE INDEX idx_products_categories ON products(category_id);