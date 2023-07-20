CREATE TABLE kittnes (name text);
CREATE TABLE chat_bots (
          id INTEGER PRIMARY KEY NOT NULL,
          description TEXT NOT NULL
      );
CREATE UNIQUE INDEX unique_index_chat_bot_ids
      ON chat_bots (id);
