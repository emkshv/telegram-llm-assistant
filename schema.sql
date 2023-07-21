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
