use axum::{
    extract::Request,
    response::Html,
};
use chrono::Local;

pub async fn root_handler(req: Request) -> Html<String> {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    let headers = req.headers();
    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("Unknown");
        
    
    let ip = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .or_else(|| {
             
             Some("Direct Connection (IP not extracted)")
        })
        .unwrap_or("Unknown");

    let html = format!(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Server Info</title>
    <style>
        body {{
            font-family: 'Inter', sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            height: 100vh;
            margin: 0;
            display: flex;
            align-items: center;
            justify-content: center;
            color: white;
        }}
        .card {{
            background: rgba(255, 255, 255, 0.1);
            backdrop-filter: blur(10px);
            padding: 2rem;
            border-radius: 1rem;
            box-shadow: 0 8px 32px 0 rgba(31, 38, 135, 0.37);
            border: 1px solid rgba(255, 255, 255, 0.18);
            text-align: center;
            max-width: 400px;
            width: 100%;
        }}
        h1 {{
            margin-bottom: 0.5rem;
            font-size: 2.5rem;
        }}
        p {{
            margin: 0.5rem 0;
            opacity: 0.8;
        }}
        .info {{
            margin-top: 1.5rem;
            padding-top: 1.5rem;
            border-top: 1px solid rgba(255, 255, 255, 0.2);
            font-size: 0.9rem;
        }}
        .label {{
            font-weight: bold;
            display: block;
            margin-bottom: 0.2rem;
            color: #ffd700;
        }}
    </style>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;700&display=swap" rel="stylesheet">
</head>
<body>
    <div class="card">
        <p>Current Server Time</p>
        <h1>{time}</h1>
        <div class="info">
            <p><span class="label">User Agent</span> {ua}</p>
            <p><span class="label">IP Address</span> {ip}</p>
        </div>
    </div>
</body>
</html>
    "#, time = now, ua = user_agent, ip = ip);

    Html(html)
}
