use anyhow::{anyhow, Result};
use dotenv::dotenv;
use reqwest::{Client, ClientBuilder};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct LlmClient {
    pub api_key: String,
    pub model: String,
    pub endpoint: String,
    http_client: Client,
}

impl LlmClient {
    pub fn from_env() -> Result<Self> {
        if dotenv().is_err() {
            eprintln!("警告：未加载到 .env 文件，将读取系统环境变量");
        }

        let api_key = std::env::var("DASHSCOPE_API_KEY")
            .map_err(|_| anyhow!("缺少环境变量 DASHSCOPE_API_KEY"))?;

        let model = std::env::var("DASHSCOPE_MODEL")
            .unwrap_or_else(|_| "qwen-turbo".to_string());

        let endpoint = "https://dashscope.aliyuncs.com/api/v1/services/aigc/text-generation/generation".to_string();

        let http_client = ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        Ok(Self {
            api_key,
            model,
            endpoint,
            http_client,
        })
    }

    pub async fn chat(
        &self,
        messages: &[ChatMessage],
        tools: &[ToolDef],
    ) -> Result<LlmResponse> {
        let mut input_messages = Vec::new();
        for msg in messages {
            input_messages.push(msg);
        }

        let mut payload = serde_json::json!({
            "model": self.model,
            "input": {
                "messages": input_messages
            },
            "parameters": {}
        });

        if !tools.is_empty() {
            payload["parameters"]["tools"] = serde_json::to_value(tools)?;
        }

        let resp = self.http_client
            .post(&self.endpoint)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        // 捕获401鉴权错误，满足作业场景3
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            if status == 401 {
                return Err(anyhow!("错误：API 返回 401 Unauthorized"));
            }
            return Err(anyhow!("HTTP 请求失败，状态码：{}", status));
        }

        let raw_resp: serde_json::Value = resp.json().await?;

        if let Some(err_msg) = raw_resp["message"].as_str() {
            return Err(anyhow!("大模型接口错误：{}", err_msg));
        }

        let output = &raw_resp["output"];
        let choices = output["choices"].as_array()
            .ok_or_else(|| anyhow!("接口返回数据异常：choices不存在"))?;
        let msg_obj = choices.get(0)
            .ok_or_else(|| anyhow!("choices 数组为空"))?;

        let content = msg_obj["content"]
            .as_str()
            .unwrap_or_default()
            .to_string();

        let mut tool_calls = Vec::new();
        if let Some(calls) = msg_obj["tool_calls"].as_array() {
            for call in calls {
                let func = &call["function"];
                let name = func["name"].as_str().unwrap_or_default().to_string();
                let args = func["arguments"].clone();
                tool_calls.push(ToolCall {
                    name,
                    arguments: args,
                });
            }
        }

        Ok(LlmResponse {
            content,
            tool_calls,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "role")]
pub enum ChatMessage {
    #[serde(rename = "system")]
    System { content: String },
    #[serde(rename = "user")]
    User { content: String },
    #[serde(rename = "assistant")]
    Assistant { content: String },
    #[serde(rename = "tool")]
    Tool { content: String },
}

impl ChatMessage {
    pub fn system(content: &str) -> Self {
        Self::System {
            content: content.to_string(),
        }
    }

    pub fn user(content: &str) -> Self {
        Self::User {
            content: content.to_string(),
        }
    }

    pub fn tool_result(content: &str) -> Self {
        Self::Tool {
            content: content.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDef {
    pub name: String,
    pub description: String,
    pub parameters: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct ToolCall {
    pub name: String,
    pub arguments: Value,
}

#[derive(Debug, Clone)]
pub struct LlmResponse {
    pub content: String,
    pub tool_calls: Vec<ToolCall>,
}