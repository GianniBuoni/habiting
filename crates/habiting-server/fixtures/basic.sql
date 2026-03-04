-- Insert 'reading' and 'writing' tags
INSERT INTO tags (name) VALUES ('reading'), ('writing'), ('grading');
-- Insert a session for each tag
INSERT INTO sessions
    (tag_id)
SELECT
    uuid AS tag_id
FROM tags WHERE name = 'reading';

INSERT INTO sessions
    (tag_id)
SELECT
    uuid AS tag_id
FROM tags WHERE name = 'writing';
