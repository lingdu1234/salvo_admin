/*
SQLyog Ultimate
MySQL - 10.6.5-MariaDB-1:10.6.5+maria~focal : Database - poem_demo
*********************************************************************
*/

/*!40101 SET NAMES utf8 */;

/*!40101 SET SQL_MODE=''*/;

/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;
/*Data for the table `sys_dict_type` */

insert  into `sys_dict_type`(`dict_type_id`,`dict_name`,`dict_type`,`status`,`create_by`,`update_by`,`remark`,`created_at`,`updated_at`,`deleted_at`) values 
('00UHHF2S53UK5UCUDNRA1OH7AV','用户性别','sys_user_sex','1','00TV87DDOBJPU75J4TGUOC3NNG',NULL,'用户性别','2022-01-29 14:46:40',NULL,NULL),
('00UHHG0BSSUOETFAOH17300Q6R','菜单状态','sys_show_hide','1','00TV87DDOBJPU75J4TGUOC3NNG',NULL,'菜单状态','2022-01-29 14:47:41',NULL,NULL),
('00UHHG6K1346UE9P05BTTVS8V7','系统开关','sys_normal_disable','1','00TV87DDOBJPU75J4TGUOC3NNG',NULL,'系统开关','2022-01-29 14:47:53',NULL,NULL),
('00UHHGMG0UENEVVS9CCBT3EJCO','任务状态','sys_job_status','1','00TV87DDOBJPU75J4TGUOC3NNG',NULL,'任务状态','2022-01-29 14:48:26',NULL,NULL),
('00UHHGTILI6FEI50ORH6MLV9ML','任务分组','sys_job_group','1','00TV87DDOBJPU75J4TGUOC3NNG',NULL,'任务分组','2022-01-29 14:48:40',NULL,NULL),
('00UHHH4TH1MH7UGNNRNMQ7T7NK','系统是否','sys_yes_no','1','00TV87DDOBJPU75J4TGUOC3NNG',NULL,'系统是否','2022-01-29 14:48:55',NULL,NULL),
('00UHHHB744PFPU65NNNCB6ELMU','通知类型','sys_notice_type','1','00TV87DDOBJPU75J4TGUOC3NNG',NULL,'通知类型','2022-01-29 14:49:08',NULL,NULL),
('00UHHHHCRILRETB1I62TBT1LJ7','通知状态','sys_notice_status','1','00TV87DDOBJPU75J4TGUOC3NNG',NULL,'通知状态','2022-01-29 14:49:21',NULL,NULL),
('00UHHHN2UJJ2JC8H2CAQ9KR3JH','操作类型','sys_oper_type','1','00TV87DDOBJPU75J4TGUOC3NNG',NULL,'操作类型','2022-01-29 14:49:33',NULL,NULL),
('00UHHHTTHGVTTUEQDME7KMLM6T','系统状态','sys_common_status','1','00TV87DDOBJPU75J4TGUOC3NNG',NULL,'系统状态','2022-01-29 14:49:47',NULL,NULL),
('00UPH6NP9LQVDM36MP01VG2F4T','任务属性','sys_task_is_once','1','00TV87DDOBJPU75J4TGUOC3NNG','00TV87DDOBJPU75J4TGUOC3NNG','是否单次任务','2022-02-04 19:45:24','2022-02-04 19:50:03',NULL),
('00UTFF2ICB56UBRHK569HA0KP8','api请求方法','sys_api_method','1','00TV87DDOBJPU75J4TGUOC3NNG',NULL,'系统API请求方法','2022-02-07 21:18:32',NULL,NULL),
('00VVMRLPEG4AELO40LBSEK4Q0O','数据库','db','1','00TV87DDOBJPU75J4TGUOC3NNG',NULL,'数据库名称','2022-03-06 11:25:19',NULL,NULL),
('0112N7A587U37O8TF0DQTUMILT','用户类型','is_admin','1','00TV87DDOBJPU75J4TGUOC3NNG',NULL,'是否后台用户','2022-04-02 16:04:52',NULL,NULL),
('0113V85U7TMBL9KHSJD51K4IBI','api缓存方式','api_cache_method','1','00UT9J78PSU5QJRE3HSDUG94R2','00UT9J78PSU5QJRE3HSDUG94R2','api缓存方式','2022-04-03 15:23:55','2022-04-03 15:38:43',NULL),
('0113VNT5GJ3RS7VHRNFKFOO1T9','api日志记录方式','api_log_method','1','00UT9J78PSU5QJRE3HSDUG94R2',NULL,'api日志记录方式','2022-04-03 15:41:05',NULL,NULL);

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;
