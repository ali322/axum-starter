-- Add migration script here
DROP TABLE IF EXISTS `users`;

CREATE TABLE `users` (
  `id` varchar(100) NOT NULL,
  `username` varchar(50) NOT NULL,
  `password` varchar(100) NOT NULL,
  `email` varchar(200) DEFAULT NULL,
  `is_actived` int(1) NOT NULL DEFAULT '1',
  `last_logined_at` datetime NOT NULL,
  `created_at` datetime NOT NULL,
  PRIMARY KEY (`id`)
);

DROP TABLE IF EXISTS `posts`;

CREATE TABLE `posts` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `title` VARCHAR(100) NOT NULL,
  `content` TEXT DEFAULT NULL,
  `created_at` DATETIME NOT NULL,
  `updated_at` DATETIME NOT NULL,
  `user_id` VARCHAR(100) NOT NULL,
  PRIMARY key(`id`),
  CONSTRAINT `fk-posts-users` FOREIGN KEY (`user_id`) REFERENCES `users`(`id`)
);

DROP TABLE IF EXISTS `tags`;

CREATE TABLE `tags` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `name` VARCHAR(100) NOT NULL,
  PRIMARY KEY(`id`)
);

DROP TABLE IF EXISTS `post_has_tags`;

CREATE TABLE `post_has_tags`(
  `post_id` INT NOT NULL,
  `tag_id` INT NOT NULL,
  PRIMARY KEY (`post_id`, `tag_id`),
  CONSTRAINT `fk-post_has_tags-posts` FOREIGN KEY (`post_id`) REFERENCES `posts`(`id`),
  CONSTRAINT `fk-post_has_tags-tags` FOREIGN KEY (`tag_id`) REFERENCES `tags`(`id`)
);
