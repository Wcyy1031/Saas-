// examples/demo.rs —— 测试 ai-client 库的三个场景
//
// 运行:cargo run --example demo

use ai_client::{ChatMessage, LlmClient, ToolDef};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 从 .env 加载配置
    let client = LlmClient::from_env()?;

    // ========== 场景 1:普通对话 ==========
    println!("========== 场景 1:普通对话 ==========");
    let messages = vec![
        ChatMessage::system("你是一个有帮助的AI助手。"),
        ChatMessage::user("用一句话介绍 Rust。"),
    ];

    let response = client.chat(&messages, &[]).await?;
    println!(
        "LLM: {}",
        response.content.as_deref().unwrap_or("(无文本回复)")
    );

    // ========== 场景 2:工具调用 ==========
    println!("\n========== 场景 2:工具调用 ==========");
    let tools = vec![ToolDef {
        name: "get_time".into(),
        description: "获取当前时间".into(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "timezone": { "type": "string", "description": "时区" }
            },
            "required": ["timezone"]
        }),
    }];

    let messages = vec![
        ChatMessage::system("你是一个有帮助的AI助手,可以调用工具。"),
        ChatMessage::user("现在上海时间几点?"),
    ];

    let response = client.chat(&messages, &tools).await?;

    // 检查是否有工具调用
    if response.tool_calls.is_empty() {
        println!(
            "LLM(未调用工具): {}",
            response.content.as_deref().unwrap_or("")
        );
    } else {
        for tc in &response.tool_calls {
            println!("LLM 想调用工具: {}({})", tc.name, tc.arguments);
        }
    }

    Ok(())
}
