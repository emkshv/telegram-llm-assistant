CREATE TABLE chat_bots (
          id INTEGER PRIMARY KEY NOT NULL,
          behavior TEXT NOT NULL
      );
CREATE UNIQUE INDEX unique_index_chat_bot_ids
      ON chat_bots (id);

CREATE TABLE chat_threads (
          id INTEGER PRIMARY KEY NOT NULL,
          is_current BOOLEAN,
          chat_id INTEGER NOT NULL
      );

CREATE UNIQUE INDEX IF NOT EXISTS idx_one_current_thread_per_chat ON chat_threads(chat_id) WHERE is_current;

CREATE TABLE IF NOT EXISTS chat_messages (
    id INTEGER PRIMARY KEY NOT NULL,
    content TEXT NOT NULL,
    chat_id INTEGER NOT NULL,
    chat_thread_id INTEGER NOT NULL,
    user_role TEXT NOT NULL
);
