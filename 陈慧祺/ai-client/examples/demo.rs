use ai_client::{LlmClient, ChatMessage, ToolDef, ToolCall};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    // 1. 从 .env 加载配置
    let client = LlmClient::from_env()?;

    println!("========== 场景1：普通对话 ==========");
    // 2. 构建消息
    let messages = vec![
        ChatMessage::system("你是一个有帮助的AI助手。"),
        ChatMessage::user("用一句话介绍 Rust。"),
    ];

    // 3. 调用 LLM（无工具）
    let response = client.chat(&messages, &[]).await?;
    if response.content.trim().is_empty() {
        println!("LLM: 模型未返回文本内容");
    } else {
        println!("LLM: {}", response.content.unwrap_or_else(|| "无回复内容".to_string()));
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    println!("\n========== 场景2：工具调用 ==========");
    // 4. 调用 LLM（带工具）
    let tools = vec![
        ToolDef {
            name: "get_time".into(),
            description: "获取当前时间".into(),
            parameters: serde_json::json!({
                "type": "object",
                "required": ["timezone"],
                "properties": {
                    "timezone": {"type": "string", "description": "时区，例如 Asia/Shanghai"}
                }
            }),
        },
    ];

    let messages2 = vec![
        ChatMessage::system("你是一个有帮助的AI助手。"),
        ChatMessage::user("现在上海是什么时间？"),
    ];
    let response = client.chat(&messages2, &tools).await?;

    // 5. 检查是否有工具调用
    if !response.tool_calls.is_empty() {
        for tc in &response.tool_calls {
            println!("LLM 想调用: {} ({})", tc.name, tc.arguments);
        }
    } else {
        if response.content.trim().is_empty() {
            println!("LLM: 模型未返回文本内容");
        } else {
            println!("LLM: {}", response.content);
        }
    }

    Ok(())
}