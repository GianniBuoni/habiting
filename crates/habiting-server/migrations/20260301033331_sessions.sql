-- Up
CREATE TABLE sessions (
    uuid UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    active BOOLEAN NOT NULL DEFAULT false,
    tag_id UUID NOT NULL,
    CONSTRAINT fk_tag
        FOREIGN KEY(tag_id)
        REFERENCES tags(uuid)
        ON DELETE CASCADE
);
