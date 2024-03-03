use salvo_core::Router;

use crate::common;

pub mod api;
pub mod api_fn;

pub fn system_api() -> Router {
    Router::new()
        .push(
            Router::with_path("user") // 用户api
                .push(Router::with_path("list").get(api::sys_user::get_sort_list))
                .push(Router::with_path("get_by_id").get(api::sys_user::get_by_id))
                .push(Router::with_path("get_profile").get(api::sys_user::get_profile)) // 按当前获取用户信息
                .push(Router::with_path("update_profile").put(api::sys_user::update_profile)) // 更新用户信息
                .push(Router::with_path("add").post(api::sys_user::add)) // 添加用户
                .push(Router::with_path("edit").put(api::sys_user::edit)) // 更新用户
                .push(Router::with_path("delete").delete(api::sys_user::delete)) // 硬删除用户
                .push(Router::with_path("get_info").get(api::sys_user::get_info)) // 获取用户信息
                .push(Router::with_path("reset_passwd").put(api::sys_user::reset_passwd)) // 重置密码
                .push(Router::with_path("update_passwd").put(api::sys_user::update_passwd)) // 重置密码
                .push(Router::with_path("change_status").put(api::sys_user::change_status)) // 修改状态
                .push(Router::with_path("change_role").put(api::sys_user::change_role)) // 切换角色
                .push(Router::with_path("change_dept").put(api::sys_user::change_dept)) // 切换部门
                .push(Router::with_path("fresh_token").put(api::sys_user::fresh_token)) // 修改状态
                .push(Router::with_path("update_avatar").post(api::sys_user::update_avatar)), // 修改头像
        )
        .push(
            Router::with_path("dict")
                .push(
                    Router::with_path("type")
                        .push(Router::with_path("list").get(api::sys_dict_type::get_sort_list)) // 获取筛选分页
                        .push(Router::with_path("get_all").get(api::sys_dict_type::get_all)) // 获取筛选分页
                        .push(Router::with_path("get_by_id").get(api::sys_dict_type::get_by_id)) // 按id获取
                        .push(Router::with_path("add").post(api::sys_dict_type::add)) // 添加
                        .push(Router::with_path("edit").put(api::sys_dict_type::edit)) // 更新
                        .push(Router::with_path("delete").delete(api::sys_dict_type::delete)), // 硬删除
                )
                .push(
                    Router::with_path("data")
                        .push(Router::with_path("list").get(api::sys_dict_data::get_sort_list)) // 获取筛选分页
                        .push(Router::with_path("get_all").get(api::sys_dict_data::get_all)) // 获取筛选分页
                        .push(Router::with_path("get_by_id").get(api::sys_dict_data::get_by_id)) // 按id获取
                        .push(Router::with_path("get_by_type").get(api::sys_dict_data::get_by_type)) // 按type获取
                        .push(Router::with_path("add").post(api::sys_dict_data::add)) // 添加
                        .push(Router::with_path("edit").put(api::sys_dict_data::edit)) // 更新
                        .push(Router::with_path("delete").delete(api::sys_dict_data::delete)), // 硬删除
                ),
        )
        .push(
            Router::with_path("post")
                .push(Router::with_path("list").get(api::sys_post::get_sort_list)) // 获取筛选分页
                .push(Router::with_path("get_all").get(api::sys_post::get_all)) // 获取筛选分页
                .push(Router::with_path("get_by_id").get(api::sys_post::get_by_id)) // 按id获取
                .push(Router::with_path("add").post(api::sys_post::add)) // 添加
                .push(Router::with_path("edit").put(api::sys_post::edit)) // 更新
                .push(Router::with_path("delete").delete(api::sys_post::delete)), // 硬删除
        )
        .push(
            Router::with_path("dept")
                .push(Router::with_path("list").get(api::sys_dept::get_sort_list)) // 获取筛选分页
                .push(Router::with_path("get_all").get(api::sys_dept::get_all)) // 获取筛选分页
                .push(Router::with_path("get_dept_tree").get(api::sys_dept::get_dept_tree)) // 获取部门树
                .push(Router::with_path("get_by_id").get(api::sys_dept::get_by_id)) // 按id获取
                .push(Router::with_path("add").post(api::sys_dept::add)) // 添加
                .push(Router::with_path("edit").put(api::sys_dept::edit)) // 更新
                .push(Router::with_path("delete").delete(api::sys_dept::delete)), // 硬删除
        )
        .push(
            Router::with_path("role")
                .push(Router::with_path("list").get(api::sys_role::get_sort_list)) // 获取筛选分页
                .push(Router::with_path("get_all").get(api::sys_role::get_all)) // 获取筛选分页
                .push(Router::with_path("get_by_id").get(api::sys_role::get_by_id)) // 按id获取
                .push(Router::with_path("add").post(api::sys_role::add)) // 添加
                .push(Router::with_path("edit").put(api::sys_role::edit)) // 更新
                .push(Router::with_path("change_status").put(api::sys_role::change_status)) // 设置状态
                .push(Router::with_path("set_data_scope").put(api::sys_role::set_data_scope)) // 设置数据权限范围
                .push(Router::with_path("delete").delete(api::sys_role::delete)) // 硬删除
                .push(Router::with_path("get_role_menu").get(api::sys_role::get_role_menu)) // 获取角色菜单
                .push(Router::with_path("get_role_dept").get(api::sys_role::get_role_dept)), // 获取角色部门
        )
        .push(
            Router::with_path("menu")
                .push(Router::with_path("list").get(api::sys_menu::get_sort_list)) // 获取筛选分页
                .push(Router::with_path("get_by_id").get(api::sys_menu::get_by_id)) // 按id获取
                .push(Router::with_path("add").post(api::sys_menu::add)) // 添加
                .push(Router::with_path("edit").put(api::sys_menu::edit)) // 更新
                .push(
                    Router::with_path("update_log_cache_method")
                        .put(api::sys_menu::update_log_cache_method),
                ) // 更新api缓存方式和日志记录方式
                .push(Router::with_path("delete").delete(api::sys_menu::delete)) // 硬删除
                .push(
                    Router::with_path("get_all_enabled_menu_tree")
                        .get(api::sys_menu::get_all_enabled_menu_tree),
                ) // 获取全部正常的路由菜单树
                .push(Router::with_path("get_routers").get(api::sys_menu::get_routers)) // 获取用户菜单树
                .push(
                    Router::with_path("get_auth_list").get(api::sys_menu::get_related_api_and_db),
                ), // 获取用户菜单树
        )
        .push(
            Router::with_path("login-log")
                .push(Router::with_path("list").get(api::sys_login_log::get_sort_list)) // 获取筛选分页
                .push(Router::with_path("clean").delete(api::sys_login_log::clean)) // 清空
                .push(Router::with_path("delete").delete(api::sys_login_log::delete)), // 硬删除
        )
        .push(
            Router::with_path("online")
                .push(Router::with_path("list").get(api::sys_user_online::get_sort_list)) // 获取筛选分页
                .push(Router::with_path("delete").delete(api::sys_user_online::delete)), // 删除
        )
        .push(
            Router::with_path("job")
                .push(Router::with_path("list").get(api::sys_job::get_sort_list)) // 获取筛选分页
                .push(Router::with_path("get_by_id").get(api::sys_job::get_by_id)) // 按id获取
                .push(Router::with_path("change_status").put(api::sys_job::change_status)) // 设置状态
                .push(Router::with_path("run_task_once").put(api::sys_job::run_task_once)) // 设置状态
                .push(Router::with_path("add").post(api::sys_job::add)) // 添加
                .push(Router::with_path("edit").put(api::sys_job::edit)) // 更新
                .push(Router::with_path("delete").delete(api::sys_job::delete)) // 硬删除
                .push(Router::with_path("validate_cron_str").post(api::sys_job::validate_cron_str)), // 验证cron_str
        )
        .push(
            Router::with_path("job_log")
                .push(Router::with_path("list").get(api::sys_job_log::get_sort_list)) // 获取筛选分页
                .push(Router::with_path("clean").delete(api::sys_job_log::clean)) // 清空
                .push(Router::with_path("delete").delete(api::sys_job_log::delete)), // 硬删除
        )
        .push(
            Router::with_path("oper_log")
                .push(Router::with_path("list").get(api::sys_oper_log::get_sort_list)) // 获取筛选分页
                .push(Router::with_path("get_by_id").get(api::sys_oper_log::get_by_id)) // 按id获取
                .push(Router::with_path("clean").delete(api::sys_oper_log::clean)) // 清空
                .push(Router::with_path("delete").delete(api::sys_oper_log::delete)), // 硬删除
        )
        .push(
            Router::with_path("api_db")
                .push(Router::with_path("get_by_id").get(api::sys_api_db::get_by_id)) // 按id获取
                .push(Router::with_path("add").post(api::sys_api_db::add)), // 添加
        )
        .push(
            Router::with_path("monitor")
                .push(Router::with_path("server").get(common::get_server_info)) // 服务器信息
                .push(Router::with_path("server-event").get(common::get_server_info_sse)), // 服务器信息
        )
        .push(
            Router::with_path("update_log")
                .push(Router::with_path("add").post(api::sys_update_log::add)) // 添加
                .push(Router::with_path("edit").put(api::sys_update_log::edit)) // 更新
                .push(Router::with_path("delete").delete(api::sys_update_log::delete)) // 硬删除
                .push(Router::with_path("get_all").get(api::sys_update_log::get_all)), // 获取全部
        )
}
