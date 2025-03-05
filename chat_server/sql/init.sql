-- 创建用户表
CREATE TABLE users if not exists (
    id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

create table chats if not exists (
    id INT AUTO_INCREMENT PRIMARY KEY,
    user_ids INT[] NOT NULL,
    chat_type INT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 创建消息表
CREATE TABLE messages if not exists (
    id INT AUTO_INCREMENT PRIMARY KEY,
    chat_id INT NOT NULL,
    sender_id INT NOT NULL,
    content TEXT NOT NULL,
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 创建群组表
CREATE TABLE groups if not exists (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

create index idx_user_ids on chats(user_ids);
create index idx_chat_id on messages(chat_id);
create index idx_sender_id on messages(sender_id);
create index idx_group_id on messages(group_id);
