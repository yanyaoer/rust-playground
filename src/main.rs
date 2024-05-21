use std::env;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateContentResponse {
    pub candidates: Vec<Candidate>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candidate {
    pub content: Content,
    pub finish_reason: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub role: String,
    pub parts: Vec<Part>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Part {
    Text(String),
    InlineData {
        mime_type: String,
        data: String,
    },
    FileData {
        mime_type: String,
        file_uri: String,
    },
    FunctionCall {
        name: String,
        args: HashMap<String, String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
  let args: Vec<String> = env::args().collect();
  let def_msg = &String::from("hello, gemini");
  let msg: &str = args.get(1).unwrap_or(def_msg);
  let proxy = env::var("proxy").unwrap_or("socks5://127.0.0.1:1088".to_string());
  let api_key = env::var("api").expect("api key not set");
  // println!("proxy: {}", proxy);
  // println!("api: {}", api_key);

  /*
  curl -x socks5://127.0.0.1:1088 -H 'Content-Type: application/json' \
    -d '{"contents":[{"parts":[{"text":"'$@'"}]}]}' \
    -X POST 'https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key=${API_KEY}' \
    | jq -r .candidates.[].content.parts.[].text
  */

  let client = reqwest::Client::builder()
    .proxy(reqwest::Proxy::https(proxy)?)
    .build()?;
  let res = client
    .post(format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key={api_key}"))
    .header("Content-Type", "application/json")
    .json(&serde_json::json!({
      "contents": [{
        "parts": [{"text": &msg }]
      }]
    }))
    .send()
    .await?
    .json::<GenerateContentResponse>()
    .await?;

  /*
  let res = serde_json::from_str::<GenerateContentResponse>(r#"{
    "candidates": [{
      "content": {
        "parts": [{
          "text": "**鼻炎的分类：**\n\n鼻炎主要分为两大类：**过敏性鼻炎**和**非过敏性鼻炎**。\n\n**一、过敏性鼻炎**\n\n又称变应性鼻炎，是由 于接触过敏原（如花粉、尘螨、动物皮屑等）而引起的鼻黏膜炎症反应。主要症状包括：\n\n* **季节性或常年性鼻塞**\n* **打喷嚏**\n* **流鼻涕**，通常为清澈的水样鼻涕\n* **鼻痒**\n* **眼睛痒、流泪、眼睑肿胀**\n\n根据发病季节，过敏性鼻炎可进一步分为：\n\n* **季节性过敏性鼻炎：**通常在特定季节发作，如春季（花粉症）或秋季（霉菌过敏）。\n* **常年性过敏性鼻炎：**全年都可能发作，通常是由于对尘螨、动物皮屑等过敏原的持续接触。\n\n**二、非过敏性鼻炎**\n\n又称血管运动性鼻炎，原因较复杂，通常与以下因素有关：\n\n* **温度变化**\n* **气味刺激**\n* **情绪压力**\n* **药物刺激**\n* **激素水平变化**\n* **感染**\n\n主要症状包括：\n\n* **鼻塞**，通常是交替性鼻塞，即左右鼻孔轮流堵塞。\n* **流鼻涕**，通常为粘稠的鼻涕，有时伴有鼻腔分泌物变色。\n* **鼻痒**\n* **嗅觉减退**\n* **头痛**\n\n**此外，鼻炎还可根据病程分为：**\n\n* **急性鼻炎：**一般持续时间较短，症状较轻，常见于感冒。\n* **慢性鼻炎：**症状持续存在，反复发作，可持续数月甚至数年，严重影响生活质量。\n\n**需要注意的是，**以上分类仅供参考，具体的诊断需要专业的医生进行评估和判断。如果您出现鼻炎症状，建议及时就医，明确诊断并接受合理的治疗。\n"
        }],
        "role": "model"
      }
    }]
  }"#).unwrap();
  */

  res.candidates.iter().for_each(|candidate| {
      candidate.content.parts.iter().for_each(|part| {
          if let Part::Text(text) = part {
              termimad::print_inline(&text);
              // println!("{}", text);
          }
      });
  });
  Ok(())
}
