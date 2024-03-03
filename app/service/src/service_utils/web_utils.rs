use std::borrow::Cow;

use configs::CFG;
use db::common::client::{ClientInfo, ClientNetInfo, UserAgentInfo};
use headers::HeaderMap;
use user_agent_parser::UserAgentParser;
use xdb::{search_by_ip, searcher_init};

pub async fn get_client_info(header: &HeaderMap, remote_ip: String) -> ClientInfo {
    let user_agent = header.get("user-agent").unwrap().to_str().unwrap();
    let ua = get_user_agent_info(user_agent);
    let ip = match get_remote_ip(header) {
        None => remote_ip,
        Some(v) => {
            if !v.is_empty() {
                v
            } else {
                remote_ip
            }
        }
    };
    // let net = get_city_by_ip(&ip).await.unwrap();
    let net = get_city_by_ip2(&ip);
    ClientInfo { net, ua }
}

pub fn get_remote_ip(header: &HeaderMap) -> Option<String> {
    let ip = match header.get("X-Forwarded-For") {
        Some(x) => {
            let mut ips = x.to_str().unwrap().split(',');
            Some(ips.next().unwrap().trim().to_string())
        }
        None => header
            .get("X-Real-IP")
            .map(|x| x.to_str().unwrap().to_string()),
    };
    ip
}

pub fn get_user_agent_info(user_agent: &str) -> UserAgentInfo {
    let ua_parser = UserAgentParser::from_path(&CFG.system.user_agent_parser).unwrap();
    let product_v = ua_parser.parse_product(user_agent);
    let os_v = ua_parser.parse_os(user_agent);
    let device_v = ua_parser.parse_device(user_agent);
    let browser = product_v.name.unwrap_or(Cow::Borrowed("")).to_string()
        + " "
        + product_v
            .major
            .unwrap_or(Cow::Borrowed(""))
            .to_string()
            .as_str();
    let os = os_v.name.unwrap_or(Cow::Borrowed("")).to_string()
        + " "
        + os_v.major.unwrap_or(Cow::Borrowed("")).to_string().as_str();
    let device = device_v.name.unwrap_or(Cow::Borrowed("")).to_string();
    UserAgentInfo {
        browser: browser.trim().to_string(),
        os: os.trim().to_string(),
        device,
    }
}

// async fn get_city_by_ip(ip: &str) -> Result<ClientNetInfo, Box<dyn std::error::Error>> {
//     let url = "http://whois.pconline.com.cn/ipJson.jsp?json=true&ip=".to_string() + ip;
//     println!("ip查询地址：{}", &url);
//     let resp = reqwest::get(url.as_str())
//         .await?
//         .text_with_charset("utf-8")
//         .await?;
//     println!("ip查询地址--------------res：{}", &resp);
//     println!("ip查询地址--------------res");
//     let res = serde_json::from_str::<HashMap<String, String>>(resp.as_str())?;
//     let location = format!("{}{}", res["pro"], res["city"]);
//     let net_work = res["addr"].split(' ').collect::<Vec<&str>>()[1].to_string();
//     Ok(ClientNetInfo {
//         ip: res["ip"].to_string(),
//         location,
//         net_work,
//     })
// }

fn get_city_by_ip2(ip: &str) -> ClientNetInfo {
    let v = search_by_ip(ip).unwrap();
    let data = v.split('|').collect::<Vec<&str>>();
    let location = format!(
        "{}{}{}",
        if data[0] == "0" { "" } else { data[0] },
        if data[2] == "0" { "" } else { data[2] },
        if data[3] == "0" { "" } else { data[3] }
    );
    let net_work = (if data[4] == "0" { "" } else { data[4] }).to_string();
    ClientNetInfo {
        ip: ip.to_string(),
        location,
        net_work,
    }
}

pub fn set_xdb() {
    let xdb = &CFG.system.ip2region;
    searcher_init(Some(xdb.to_owned()));
    tracing::info!("ip查询数据库初始化完成");
}
