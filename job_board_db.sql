-- MySQL dump 10.13  Distrib 8.0.42, for Linux (x86_64)
--
-- Host: localhost    Database: job_board_db
-- ------------------------------------------------------
-- Server version	8.0.42-0ubuntu0.20.04.1

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!50503 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `applications`
--

DROP TABLE IF EXISTS `applications`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `applications` (
  `id` int NOT NULL AUTO_INCREMENT,
  `user_id` int NOT NULL,
  `job_id` int NOT NULL,
  `full_name` varchar(255) DEFAULT NULL,
  `email` varchar(255) DEFAULT NULL,
  `applied_at` datetime DEFAULT CURRENT_TIMESTAMP,
  `resume_link` text,
  `cover_letter` text,
  PRIMARY KEY (`id`),
  UNIQUE KEY `unique_application` (`user_id`,`job_id`),
  KEY `job_id` (`job_id`),
  CONSTRAINT `applications_ibfk_1` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`),
  CONSTRAINT `applications_ibfk_2` FOREIGN KEY (`job_id`) REFERENCES `jobs` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=11 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `applications`
--

LOCK TABLES `applications` WRITE;
/*!40000 ALTER TABLE `applications` DISABLE KEYS */;
INSERT INTO `applications` VALUES (9,2,10,'John Doe','john@example.com','2025-06-26 05:49:20','https://example.com/resume.pdf','I am excited to contribute to your company.'),(10,2,1,'Jane Doe','jane@example.com','2025-06-27 03:24:39','https://example.com/resume.pdf','I am excited to apply for this job.');
/*!40000 ALTER TABLE `applications` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `companies`
--

DROP TABLE IF EXISTS `companies`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `companies` (
  `id` int NOT NULL AUTO_INCREMENT,
  `name` varchar(255) NOT NULL,
  `location` varchar(255) DEFAULT NULL,
  `description` text,
  `created_at` datetime DEFAULT NULL,
  `user_id` int DEFAULT NULL,
  `website` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `companies`
--

LOCK TABLES `companies` WRITE;
/*!40000 ALTER TABLE `companies` DISABLE KEYS */;
INSERT INTO `companies` VALUES (1,'OpenAI Nigeria','Lagos','AI company focused on Rust backend roles','2025-06-17 17:58:41',2,NULL),(2,'Google Developer Groups Nigeria','Lagos','Focused on developers interest on building with AI tools','2025-06-17 18:38:02',2,NULL),(3,'OpenAI Nigeria','Abuja','AI Research & Tools','2025-06-19 12:19:34',NULL,'https://openai.com');
/*!40000 ALTER TABLE `companies` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `jobs`
--

DROP TABLE IF EXISTS `jobs`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `jobs` (
  `id` int NOT NULL AUTO_INCREMENT,
  `title` varchar(255) NOT NULL,
  `description` text,
  `location` varchar(255) DEFAULT NULL,
  `company_id` int DEFAULT NULL,
  `user_id` int DEFAULT NULL,
  `created_at` datetime DEFAULT NULL,
  `job_type` varchar(50) DEFAULT 'full-time',
  `tags` text,
  `views` int DEFAULT '0',
  PRIMARY KEY (`id`),
  KEY `company_id` (`company_id`),
  KEY `user_id` (`user_id`),
  CONSTRAINT `jobs_ibfk_1` FOREIGN KEY (`company_id`) REFERENCES `companies` (`id`),
  CONSTRAINT `jobs_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=11 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `jobs`
--

LOCK TABLES `jobs` WRITE;
/*!40000 ALTER TABLE `jobs` DISABLE KEYS */;
INSERT INTO `jobs` VALUES (1,'Rust Backend Engineer','Work on high-performance backend systems using Rust and Actix.',NULL,1,NULL,'2025-06-17 18:38:40','full-time',NULL,2),(10,'Backend Developer','Build and maintain backend APIs using Rust','Remote',1,2,'2025-06-19 12:15:57','Full-time','rust, api, backend',0);
/*!40000 ALTER TABLE `jobs` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `users`
--

DROP TABLE IF EXISTS `users`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `users` (
  `id` int NOT NULL AUTO_INCREMENT,
  `email` varchar(255) NOT NULL,
  `password_hash` text NOT NULL,
  `role` varchar(50) NOT NULL DEFAULT 'user',
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP,
  `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  UNIQUE KEY `email` (`email`)
) ENGINE=InnoDB AUTO_INCREMENT=5 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `users`
--

LOCK TABLES `users` WRITE;
/*!40000 ALTER TABLE `users` DISABLE KEYS */;
INSERT INTO `users` VALUES (2,'chinemeremezenagu@gmail.com','$argon2i$v=19$m=16,t=2,p=1$Q29kZVdpdGhFemU$mFhC8G/5VulKDyKF7XOs/g','admin','2025-06-17 02:47:06','2025-06-17 02:47:06'),(3,'classicman695@gmail.com','$argon2id$v=19$m=19456,t=2,p=1$jGHAITCDfzzZ8JvpXQUyJA$gpGWbgXTRkhFrqUt3HQN21uRXwsd0w9v1eknbk+/nfw','user','2025-06-17 06:59:18','2025-06-17 06:59:18'),(4,'ezenaguchinemerempaschal@gmail.com','$argon2id$v=19$m=19456,t=2,p=1$kIb4lMylxdyCIuqPVeHj8A$8RJ5hoEhS7MPpEUmdcz5GLLk5dRB0SV3ksL7kRlkbFA','user','2025-06-19 09:39:42','2025-06-19 09:39:42');
/*!40000 ALTER TABLE `users` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2025-07-01 16:18:49
