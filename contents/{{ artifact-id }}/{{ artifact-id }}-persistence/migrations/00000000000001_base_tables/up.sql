CREATE TABLE {{ prefix_name }} (
    id SERIAL PRIMARY KEY,
    name VARCHAR,
    created TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
 );

SELECT diesel_manage_updated_at('{{ prefix_name }}');
