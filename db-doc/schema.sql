CREATE TABLE IF NOT EXISTS posts (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    section_id VARCHAR(24) NOT NULL,
    author_id VARCHAR(32) NOT NULL,
    title VARCHAR(120) NOT NULL,
    body VARCHAR(10000) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE INDEX IF NOT EXISTS posts_section_id ON posts(section_id, id);

CREATE TABLE IF NOT EXISTS comments (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    post_id BIGINT NOT NULL REFERENCES posts(id),
    author_id VARCHAR(32) NOT NULL,
    body VARCHAR(2000) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE INDEX IF NOT EXISTS comments_post_id ON comments(post_id, id);

CREATE TABLE IF NOT EXISTS chat_rooms (
    id VARCHAR(24) PRIMARY KEY,
    name VARCHAR(64) NOT NULL
);

INSERT INTO chat_rooms (id, name)
SELECT 'general', '校园闲聊'
WHERE NOT EXISTS (SELECT 1 FROM chat_rooms WHERE id = 'general');

CREATE TABLE IF NOT EXISTS chat_messages (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    room_id VARCHAR(24) NOT NULL REFERENCES chat_rooms(id),
    author_id VARCHAR(32) NOT NULL,
    client_message_id UUID NOT NULL,
    body VARCHAR(2000) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    UNIQUE (room_id, author_id, client_message_id)
);

CREATE INDEX IF NOT EXISTS chat_messages_room_id ON chat_messages(room_id, id);
