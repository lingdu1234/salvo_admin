use std::path::Path;

use anyhow::Result;
use configs::CFG;
use db::common::captcha::CaptchaImage;
use math_captcha::Captcha;
use salvo_core::http::form::FilePart;
use tokio::fs;

/// 获取验证码
pub fn get_captcha() -> CaptchaImage {
    let captcha = Captcha::new(130, 40);
    let uuid =
        utils::rand_utils::encrypt_password(&captcha.value.to_string(), &captcha.value.to_string());

    CaptchaImage {
        captcha_on_off: true,
        uuid,
        img: captcha.base64_img,
    }
}

/// 上传图片文件
pub async fn upload_img(file: &FilePart) -> Result<String> {
    let content_type = file
        .content_type()
        .map(|t| t.to_string())
        .unwrap_or_default();
    let old_url = file.name().map(ToString::to_string).unwrap_or_default();
    let file_type = get_file_type(&content_type);
    let now = chrono::Local::now();
    let file_path_t = CFG.web.upload_dir.clone() + "/" + &now.format("%Y-%m").to_string();
    let url_path_t = CFG.web.upload_url.clone() + "/" + &now.format("%Y-%m").to_string();
    fs::create_dir_all(&file_path_t).await?;
    let file_name = now.format("%d").to_string() + "-" + &scru128::new_string() + &file_type;
    let file_path = file_path_t + "/" + &file_name;
    let url_path = url_path_t + "/" + &file_name;

    if let Err(e) = std::fs::copy(file.path(), Path::new(&file_path)) {
        Err(anyhow::anyhow!(e.to_string()))
    } else {
        if !old_url.is_empty() {
            delete_file(&old_url).await;
        }
        // 删除文件
        Ok(url_path)
    }
}

fn get_file_type(content_type: &str) -> String {
    match content_type {
        "image/jpeg" => ".jpg".to_string(),
        "image/png" => ".png".to_string(),
        "image/gif" => ".gif".to_string(),
        _ => "".to_string(),
    }
}

/// 删除文件
pub async fn delete_file(file_path: &str) {
    let path = file_path.replace(&CFG.web.upload_url, &CFG.web.upload_dir);
    match fs::remove_file(&path).await {
        Ok(_) => {}
        Err(_) => {
            tracing::error!("删除文件失败:{}", path);
        }
    }
}
