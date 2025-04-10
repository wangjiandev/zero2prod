-- Add migration script here

-- 创建subscriptions表
CREATE TABLE subscriptions (
    id uuid PRIMARY KEY NOT NULL,
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    subscribed_at timestamptz NOT NULL
);
