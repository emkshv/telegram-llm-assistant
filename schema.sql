CREATE TABLE IF NOT EXISTS chat_bots (
          id INTEGER PRIMARY KEY NOT NULL,
          behavior TEXT NOT NULL,
          openai_model TEXT NOT NULL,
          mock_model TEXT NOT NULL
      );

CREATE UNIQUE INDEX IF NOT EXISTS unique_index_chat_bot_ids
      ON chat_bots (id);

CREATE TABLE IF NOT EXISTS chat_threads (
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
    user_role TEXT NOT NULL,
    inserted_at DATETIME DEFAULT(STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW'))
);

CREATE INDEX IF NOT EXISTS idx_chat_messages_inserted_at ON chat_messages (inserted_at);
