-- XJTU-ApiHub MySQL schema
-- 当前生产库：阿里云 MySQL 8.0.36，库 smart_care（临时，与 SmartCare 共用）
-- user / user_token 已创建；posts / comments / chat_* 为下一阶段计划表
-- 字符集：utf8mb4_general_ci；学号与 token 字段使用 ascii_general_ci

CREATE TABLE IF NOT EXISTS `user` (
  `stu_id` VARCHAR(16) CHARACTER SET ascii COLLATE ascii_general_ci NOT NULL COMMENT '学号，唯一账号 ID',
  `nick_name` VARCHAR(32) NOT NULL COMMENT '社区昵称',
  `real_name` VARCHAR(64) NULL COMMENT '真实姓名',
  `grade_name` VARCHAR(64) NULL COMMENT '年级/行政班级名称',
  `province_code` CHAR(2) CHARACTER SET ascii COLLATE ascii_general_ci NULL COMMENT '生源省份代码',
  `gender` TINYINT NULL COMMENT '性别：1 男，2 女',
  `is_deleted` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '逻辑删除：0 正常，1 删除',
  `degree_type` TINYINT NULL COMMENT '学历类型：2 本科，3 研究生',
  `enrollment_year` SMALLINT NULL COMMENT '入学年份',
  `privacy_agreed_at` DATETIME(6) NULL COMMENT '同意隐私协议时间',
  `last_login_at` DATETIME(6) NULL COMMENT '最后登录时间',
  `created_at` DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6) COMMENT '创建时间',
  `updated_at` DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6) ON UPDATE CURRENT_TIMESTAMP(6) COMMENT '更新时间',
  PRIMARY KEY (`stu_id`),
  KEY `idx_user_nick_name` (`nick_name`),
  CONSTRAINT `chk_user_stu_id` CHECK (`stu_id` <> ''),
  CONSTRAINT `chk_user_nick_name` CHECK (`nick_name` <> '')
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='社区用户';

CREATE TABLE IF NOT EXISTS `user_token` (
  `token_hash` CHAR(64) CHARACTER SET ascii COLLATE ascii_general_ci NOT NULL COMMENT '明文 token 的 SHA-256',
  `stu_id` VARCHAR(16) CHARACTER SET ascii COLLATE ascii_general_ci NOT NULL COMMENT '所属用户学号',
  `created_at` DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6) COMMENT '签发时间',
  `expires_at` DATETIME(6) NOT NULL COMMENT '过期时间',
  `revoked_at` DATETIME(6) NULL COMMENT '吊销时间',
  `last_used_at` DATETIME(6) NULL COMMENT '最后使用时间',
  PRIMARY KEY (`token_hash`),
  KEY `idx_user_token_stu_id` (`stu_id`),
  KEY `idx_user_token_expires_at` (`expires_at`),
  CONSTRAINT `fk_user_token_user`
    FOREIGN KEY (`stu_id`) REFERENCES `user` (`stu_id`)
    ON UPDATE CASCADE ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='社区登录凭证';

CREATE TABLE IF NOT EXISTS `posts` (
  `id` BIGINT NOT NULL AUTO_INCREMENT COMMENT '帖子 ID',
  `section_id` VARCHAR(24) NOT NULL COMMENT '板块 ID',
  `author_id` VARCHAR(16) CHARACTER SET ascii COLLATE ascii_general_ci NOT NULL COMMENT '发帖人学号',
  `title` VARCHAR(120) NOT NULL COMMENT '标题',
  `body` TEXT NOT NULL COMMENT '正文',
  `is_deleted` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '逻辑删除',
  `created_at` DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6) COMMENT '创建时间',
  `updated_at` DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6) ON UPDATE CURRENT_TIMESTAMP(6) COMMENT '更新时间',
  PRIMARY KEY (`id`),
  KEY `idx_posts_section_id` (`section_id`, `id`),
  KEY `idx_posts_author_id` (`author_id`),
  CONSTRAINT `fk_posts_author`
    FOREIGN KEY (`author_id`) REFERENCES `user` (`stu_id`)
    ON UPDATE CASCADE ON DELETE RESTRICT
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='论坛帖子';

CREATE TABLE IF NOT EXISTS `comments` (
  `id` BIGINT NOT NULL AUTO_INCREMENT COMMENT '评论 ID',
  `post_id` BIGINT NOT NULL COMMENT '所属帖子 ID',
  `author_id` VARCHAR(16) CHARACTER SET ascii COLLATE ascii_general_ci NOT NULL COMMENT '评论人学号',
  `body` VARCHAR(2000) NOT NULL COMMENT '评论内容',
  `is_deleted` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '逻辑删除',
  `created_at` DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6) COMMENT '创建时间',
  `updated_at` DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6) ON UPDATE CURRENT_TIMESTAMP(6) COMMENT '更新时间',
  PRIMARY KEY (`id`),
  KEY `idx_comments_post_id` (`post_id`, `id`),
  KEY `idx_comments_author_id` (`author_id`),
  CONSTRAINT `fk_comments_post`
    FOREIGN KEY (`post_id`) REFERENCES `posts` (`id`)
    ON DELETE CASCADE,
  CONSTRAINT `fk_comments_author`
    FOREIGN KEY (`author_id`) REFERENCES `user` (`stu_id`)
    ON UPDATE CASCADE ON DELETE RESTRICT
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='帖子评论';

CREATE TABLE IF NOT EXISTS `chat_rooms` (
  `id` VARCHAR(24) NOT NULL COMMENT '房间标识',
  `name` VARCHAR(64) NOT NULL COMMENT '房间显示名',
  `created_at` DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6) COMMENT '创建时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='聊天室';

INSERT IGNORE INTO `chat_rooms` (`id`, `name`) VALUES ('general', '校园闲聊');

CREATE TABLE IF NOT EXISTS `chat_messages` (
  `id` BIGINT NOT NULL AUTO_INCREMENT COMMENT '消息 ID，兼轮询游标',
  `room_id` VARCHAR(24) NOT NULL COMMENT '所属房间',
  `author_id` VARCHAR(16) CHARACTER SET ascii COLLATE ascii_general_ci NOT NULL COMMENT '发送人学号',
  `client_message_id` CHAR(36) CHARACTER SET ascii COLLATE ascii_general_ci NOT NULL COMMENT '客户端消息 ID',
  `body` VARCHAR(2000) NOT NULL COMMENT '消息内容',
  `is_deleted` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '逻辑删除',
  `created_at` DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6) COMMENT '创建时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_chat_messages_client` (`room_id`, `author_id`, `client_message_id`),
  KEY `idx_chat_messages_room_id` (`room_id`, `id`),
  CONSTRAINT `fk_chat_messages_room`
    FOREIGN KEY (`room_id`) REFERENCES `chat_rooms` (`id`)
    ON DELETE RESTRICT,
  CONSTRAINT `fk_chat_messages_author`
    FOREIGN KEY (`author_id`) REFERENCES `user` (`stu_id`)
    ON UPDATE CASCADE ON DELETE RESTRICT
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='聊天室消息';
