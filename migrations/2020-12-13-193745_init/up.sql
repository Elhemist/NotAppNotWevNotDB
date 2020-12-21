CREATE TYPE user_role AS ENUM ('client', 'courier', 'admin');

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    phone TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    role user_role NOT NULL DEFAULT 'client',
    first_name TEXT,
    middle_name TEXT,
    last_name TEXT,
    CHECK (
        role = 'client'
        OR role != 'client'
        AND first_name IS NOT NULL
        AND last_name IS NOT NULL
    )
);

CREATE TYPE transport_colors AS ENUM ('black', 'gray', 'white', 'yellow', 'red', 'blue', 'brown');

CREATE TABLE transport (
    id SERIAL PRIMARY KEY,
    number TEXT NOT NULL,
    color transport_colors NOT NULL,
    model TEXT NOT NULL
);

CREATE TYPE courier_status AS ENUM ('not_working', 'free', 'delivering', 'returning');

CREATE TABLE courier (
    user_id INTEGER PRIMARY KEY REFERENCES users (id) ON DELETE CASCADE,
    transport_id INTEGER REFERENCES transport (id) ON DELETE CASCADE,
    status courier_status NOT NULL DEFAULT 'not_working'
);

CREATE TABLE product_category (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    category_id INTEGER REFERENCES product_category(id) ON DELETE CASCADE NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    price NUMERIC NOT NULL,
    image_url TEXT NOT NULL,
    available INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE addresses (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users (id) ON DELETE CASCADE,
    street TEXT NOT NULL,
    home TEXT NOT NULL,
    apartment TEXT
);

CREATE TYPE order_status AS ENUM ('shopping', 'processing', 'preparing', 'delivering', 'completed');

CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users (id) ON DELETE RESTRICT,
    courier_id INTEGER REFERENCES courier (user_id) ON DELETE RESTRICT,
    address_id INTEGER REFERENCES addresses (id) ON DELETE RESTRICT,
    status order_status NOT NULL DEFAULT 'processing',
    total_sum NUMERIC NOT NULL,
    comment TEXT
);

CREATE TABLE products_in_orders (
    id SERIAL PRIMARY KEY,
    order_id INTEGER REFERENCES orders (id) ON DELETE RESTRICT,
    product_id INTEGER REFERENCES products (id) ON DELETE RESTRICT,
    quantity INTEGER NOT NULL DEFAULT 1
);