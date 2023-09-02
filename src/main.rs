use std::fs::read_to_string;
use reqwest::Client;
use serde_json::{Value, json};

async fn login(client: &Client, user: String) -> Value{
    let user_length = user.len();
    let login = client.post("https://login.chumbacasino.com/login")
        .body(user)
        .header("User-Agent", "PostmanRuntime/7.32.3")
        .header("Content-Type", "application/json")
        .header("Content-Length", user_length)
        .header("HOST", "login.chumbacasino.com")
        .send()
        .await;
    match login {
        Err(why) => panic!("{:?}", why),
        Ok(login) => {
            let cookies = login.cookies();
            let mut cookie_str = "".to_owned();
            for cookie in cookies {
                cookie_str.push_str(&format!("{}={};", cookie.name(), cookie.value()).to_owned());
            }
            let mut login_result: String = login.text().await.unwrap();
            login_result.pop();
            login_result.push_str(r#", "cookies": ""#);
            login_result.push_str(&cookie_str);
            login_result.push_str(r#""}"#);
            let login_result_json: Value = serde_json::from_str(&login_result).unwrap();
            if login_result_json.get("success").unwrap() == true {
                return login_result_json;
            }
        },
    }
    return json!({"success": false});
}

async fn check_daily(client: &Client, login_data: &Value) -> Value {
    let cookie_str: String = login_data.get("cookies").unwrap().to_string();
    let daily_result = client.get("https://offers.chumbacasino.com/claim-api/customers/daily-bonus")
        .header("Origin", "https://lobby.chumbacasino.com")
        .header("HOST", "lobby.chumbacasino.com")
        .header("Cookie", cookie_str)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let daily_result_json: Result<Value, serde_json::Error> = serde_json::from_str(&daily_result);
    match daily_result_json {
        Err(_) => return serde_json::from_str("{}").unwrap(),
        Ok(daily_result_json) => return daily_result_json
    }
}

async fn collect_daily(client: &Client, login_data: &Value, daily_data: &Value) {
    let daily_uuid = daily_data[0].get("packageUuid").unwrap().to_string().replace("\"", "");
    let cookie_str: String = login_data.get("cookies").unwrap().to_string();
    client.post(
            format!("https://offers.chumbacasino.com/claim-api/customers/daily-bonus/{}", daily_uuid)
        )
        .header("User-Agent", "PostmanRuntime/7.32.3")
        .header("Origin", "https://lobby.chumbacasino.com")
        .header("HOST", "lobby.chumbacasino.com")
        .header("Cookie", &cookie_str)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
}

#[tokio::main]
async fn main() {
    let user_jsons_file: String = read_to_string("users.json").expect("No such file exists.");
    let user_jsons: Value = serde_json::from_str(user_jsons_file.as_str()).expect("Invalid JSON format.");
    for user_json in user_jsons.get("users").expect("Invalid users.json").as_array().unwrap() {
        let client: Client = reqwest::Client::new();
        let login_data: Value = login(&client, user_json.to_string()).await;
        if login_data.get("success").unwrap() == true {
            println!("Logged in! Checking if daily is available...");
            let daily_data = check_daily(&client, &login_data).await;
            let empty_object: Value = serde_json::from_str("{}").unwrap();
            if daily_data != empty_object {
                println!("Daily is available! Collecting...");
                collect_daily(&client, &login_data, &daily_data).await;
            }
            else {
                println!("Daily is unavailable. Was it already claimed?");
            }
        } else {
            println!("Login failed. Exiting.")
        }
    }
}
