-- Your SQL goes here
CREATE TABLE `users` (
    `id` int NOT NULL AUTO_INCREMENT,
    `first_name` varchar(250) NOT NULL,
    `last_name` varchar(50) NOT NULL,
    `age` int NOT NULL,
    `gender` varchar(10) NOT NULL DEFAULT 'MALE',
    `email` varchar(50) NOT NULL,
    `phone` varchar(20),
    `address` varchar(255),
    `username` varchar(50) NOT NULL,
    `password` varchar(255) NOT NULL,
    `created_at` TIMESTAMP  NOT NULL DEFAULT CURRENT_TIMESTAMP ,
    `created_by` int DEFAULT NULL,
    `updated_at` TIMESTAMP  DEFAULT NULL ON UPDATE CURRENT_TIMESTAMP ,
    `updated_by` int DEFAULT NULL,
    `deleted_at` TIMESTAMP DEFAULT NULL,
    `deleted_by` int DEFAULT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `email` (`email`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

CREATE TABLE `posts` (
    `id` int NOT NULL AUTO_INCREMENT,
    `title` varchar(255) NOT NULL,
    `content` text NOT NULL,
    `user_id` int NOT NULL,
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `created_by` int DEFAULT NULL,
    `updated_at` TIMESTAMP DEFAULT NULL ON UPDATE CURRENT_TIMESTAMP,
    `updated_by` int DEFAULT NULL,
    `deleted_at` TIMESTAMP DEFAULT NULL,
    `deleted_by` int DEFAULT NULL,
    PRIMARY KEY (`id`),
    KEY `FK_gn0hp001asdqwev1` (`user_id`),

    CONSTRAINT `FK_gn0hp001asdqwev1` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`)
                     ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

LOCK TABLES `users` WRITE;
/*!40000 ALTER TABLE `users` DISABLE KEYS */;
insert into `users` values (
                               1, "Phong", "Tran", 22, "MALE", "phongtk@mail.com", "0123456789", "Dong Da, Ha Noi", "gn0hp", "111", now(), null, null, null, null, null
                           );
/*!40000 ALTER TABLE `users` ENABLE KEYS */;
insert into `users` values (
                               2, "Phuonng", "Tran", 60, "MALE", "phuongtk@mail.com", "083124567895", "Nam Truc, Nam Dinh", "phuong", "222", now(), null, null, null, null, null
                           );
UNLOCK TABLES;