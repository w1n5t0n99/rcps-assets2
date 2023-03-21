use reqwasm::http;
use reqwasm::http::Response;

use domain::response::*;


async fn get_error_msg(res: &Response) -> String {
    let error_response = res.json::<ErrorResponse>().await;
    if let Ok(error_response) = error_response {
        error_response.message
    } else {
        format!("API error: {}", res.status())
    }
}

pub async fn api_register_user(user_data: &str) -> Result<RegistrationResponse, String> {
    let response = match http::Request::post("http://localhost:8000/api/account/register")
        .header("Content-Type", "application/json")
        .body(user_data)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let err_msg = get_error_msg(&response).await;
        return Err(err_msg);
    }

    let res_json = response.json::<RegistrationResponse>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn api_login_user(credentials: &str) -> Result<UserLoginResponse, String> {
    let response = match http::Request::post("http://localhost:8000/api/session/login")
        .header("Content-Type", "application/json")
        .credentials(http::RequestCredentials::Include)
        .body(credentials)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let err_msg = get_error_msg(&response).await;
        return Err(err_msg);
    }

    let res_json = response.json::<UserLoginResponse>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn api_refresh_access_token() -> Result<UserLoginResponse, String> {
    let response = match http::Request::get("http://localhost:8000/api/session/refresh")
        .header("Content-Type", "application/json")
        .credentials(http::RequestCredentials::Include)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let err_msg = get_error_msg(&response).await;
        return Err(err_msg);
    }

    let res_json = response.json::<UserLoginResponse>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn api_user_info() -> Result<FilteredUser, String> {
    let response = match http::Request::get("http://localhost:8000/api/users/me")
        .credentials(http::RequestCredentials::Include)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let err_msg = get_error_msg(&response).await;
        return Err(err_msg);
    }

    let res_json = response.json::<UserResponse>().await;
    match res_json {
        Ok(data) => Ok(data.user),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn api_logout_user() -> Result<(), String> {
    let response = match http::Request::get("http://localhost:8000/api/session/logout")
        .credentials(http::RequestCredentials::Include)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let err_msg = get_error_msg(&response).await;
        return Err(err_msg);
    }

    Ok(())
}