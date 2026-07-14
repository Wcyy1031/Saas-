//! ai-client —— 封装 DashScope(通义千问)LLM 调用的库
//!
//! 对外暴露 5 个类型和 2 个方法:
//!   - LlmClient::from_env() 从 .env 读取配置
//!   - LlmClient::chat(&messages, &tools) 调用 LLM,返回 LlmResponse

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

// ============================================================
// 1. ChatMessage —— 四种消息角色
//    用 serde 的 tag = "role" 让它序列化成
//    {"role": "user", "content": "..."} 的格式(DashScope 要求)
// ============================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum ChatMessage {
    System {
        content: String,
    },
    User {
        content: String,
    },
    #[serde(rename_all = "camelCase")]
    Assistant {
        #[serde(skip_serializing_if = "Option::is_none")]
        content: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_calls: Option<Vec<ToolCall>>,
    },
    Tool {
        tool_call_id: String,
        content: String,
    },
}

// 便利构造函数,让 demo 里写起来更简单
impl ChatMessage {
    pub fn system(content: impl Into<String>) -> Self {
        ChatMessage::System {
            content: content.into(),
        }
    }
    pub fn user(content: impl Into<String>) -> Self {
        ChatMessage::User {
            content: content.into(),
        }
    }
    pub fn assistant(content: impl Into<String>) -> Self {
        ChatMessage::Assistant {
            content: Some(content.into()),
            tool_calls: None,
        }
    }
}

// ============================================================
// 2. ToolDef —— 工具定义(发给 LLM,告诉它有哪些工具可用)
//    对应请求里 tools 数组的一个元素
// ============================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDef {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value, // JSON Schema
}

// ============================================================
// 3. ToolCall —— LLM 返回的工具调用请求
//    对应响应里 tool_calls 数组的一个元素
// ============================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: String, // LLM 生成的参数,是一段 JSON 字符串
}

// ============================================================
// 4. LlmResponse —— chat() 的返回结果
// ============================================================
#[derive(Debug, Clone)]
pub struct LlmResponse {
    pub content: Option<String>,
    pub tool_calls: Vec<ToolCall>,
}

// ============================================================
// 5. LlmClient —— HTTP 客户端封装
// ============================================================
pub struct LlmClient {
    api_key: String,
    model: String,
    endpoint: String,
    http: reqwest::Client,
}

impl LlmClient {
    /// 从 .env 读取 DASHSCOPE_API_KEY 和 DASHSCOPE_MODEL
    pub fn from_env() -> Result<Self> {
        // 加载 .env(找不到文件也不报错,可能用系统环境变量)
        let _ = dotenv::dotenv();

        let api_key = std::env::var("DASHSCOPE_API_KEY")
            .context("缺少环境变量 DASHSCOPE_API_KEY,请在 .env 里配置")?;

        // 模型可选,默认用 qwen-plus
        let model =
            std::env::var("DASHSCOPE_MODEL").unwrap_or_else(|_| "qwen-plus".to_string());

        // DashScope 的 OpenAI 兼容端点
        let endpoint = std::env::var("DASHSCOPE_ENDPOINT").unwrap_or_else(|_| {
            "https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions".to_string()
        });

        Ok(LlmClient {
            api_key,
            model,
            endpoint,
            http: reqwest::Client::new(),
        })
    }

    /// 调用 DashScope API
    ///
    /// - messages: 对话历史
    /// - tools:    可用工具(为空则普通对话)
    pub async fn chat(
        &self,
        messages: &[ChatMessage],
        tools: &[ToolDef],
    ) -> Result<LlmResponse> {
        // --- 1. 构造请求体 ---
        let mut body = serde_json::json!({
            "model": self.model,
            "messages": messages,
        });

        // 有工具时,把 ToolDef 转成 DashScope 期望的
        // { "type": "function", "function": {...} } 结构
        if !tools.is_empty() {
            let tool_array: Vec<serde_json::Value> = tools
                .iter()
                .map(|t| {
                    serde_json::json!({
                        "type": "function",
                        "function": {
                            "name": t.name,
                            "description": t.description,
                            "parameters": t.parameters,
                        }
                    })
                })
                .collect();
            body["tools"] = serde_json::Value::Array(tool_array);
        }

        // --- 2. 发送 HTTP 请求 ---
        let resp = self
            .http
            .post(&self.endpoint)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await
            .context("HTTP 请求发送失败(网络错误)")?;

        // --- 3. 检查状态码(错误处理:不 panic,返回 Err) ---
        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            anyhow::bail!("API 返回错误 {}: {}", status, text);
        }

        // --- 4. 解析响应 JSON ---
        let json: serde_json::Value =
            resp.json().await.context("解析响应 JSON 失败")?;

        // DashScope(OpenAI 兼容)结构:choices[0].message
        let message = &json["choices"][0]["message"];

        // 文本内容(可能为 null)
        let content = message["content"].as_str().map(|s| s.to_string());

        // 工具调用(可能不存在)
        let mut tool_calls = Vec::new();
        if let Some(arr) = message["tool_calls"].as_array() {
            for tc in arr {
                tool_calls.push(ToolCall {
                    id: tc["id"].as_str().unwrap_or_default().to_string(),
                    name: tc["function"]["name"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                    arguments: tc["function"]["arguments"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                });
            }
        }

        Ok(LlmResponse {
            content,
            tool_calls,
        })
    }
}
