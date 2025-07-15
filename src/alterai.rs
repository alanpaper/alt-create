use futures::StreamExt;
use reqwest;
use reqwest::header;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;


#[derive(Debug, Serialize, Deserialize)]
struct EventSteamDataChoice {
    delta: EventSteamDataDelta,
}
#[derive(Debug, Serialize, Deserialize)]
struct EventSteamDataDelta {
    content: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct EventSteamData {
    choices: Vec<EventSteamDataChoice>,
}

// 这里处理markdown常见标题 列表 即# ## ### - * 1. 2. 3. 等格式化输出 使用replace方法替换为对应的格式

#[derive(Debug, Serialize, Deserialize)]
struct PromptTokensDetails {
    cached_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatCompletion {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<Choice>,
    usage: Usage,
    system_fingerprint: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Choice {
    index: u32,
    message: Message,
    logprobs: Option<()>, // Assuming logprobs is null in the response
    finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
    prompt_tokens_details: Option<PromptTokensDetails>,
    prompt_cache_hit_tokens: u32,
    prompt_cache_miss_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseFormat {
    r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RequestBody {
    messages: Vec<Message>,
    model: String,
    stream: bool,
}

#[derive(Debug)]
pub enum AlterAIError {
    RequestFailed(reqwest::Error),
    InvalidResponse(String),
}

impl fmt::Display for AlterAIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlterAIError::RequestFailed(err) => write!(f, "Request failed: {}", err),
            AlterAIError::InvalidResponse(msg) => write!(f, "Invalid response: {}", msg),
        }
    }
}

impl Error for AlterAIError {}

impl From<serde_json::Error> for AlterAIError {
    fn from(err: serde_json::Error) -> Self {
        AlterAIError::InvalidResponse(format!("Failed to parse JSON: {}", err))
    }
}

pub async fn alterai(content: String, authorization: String) -> Result<(), AlterAIError> {
    let client = reqwest::Client::new();

    let mut headers = header::HeaderMap::new();

    let auth = format!("Bearer {}", authorization);

    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&auth).map_err(|_| {
                AlterAIError::InvalidResponse("Invalid authorization header value".to_string())
            })?,
    );
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );
    headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static("text/event-stream"),
    );

    let question = RequestBody {
        messages: vec![Message {
            content: "
                ### 游戏名称：逃出升天  
**背景设定：**  
玩家在昏迷中醒来，发现自己身处一座被遗弃的巨型建筑中地下实验室。建筑内充满未知的怪物，
它们是实验失败的产物。建筑的结构因某种超自然力量而不断变化，玩家必须在资源有限的情况下收集关键物品、解决谜题，最终找到逃生的方法。  

---

### **场景补充与核心玩法**  
#### **1. 建筑区域划分**  
- **起始区域（安全屋）**  
  - 玩家醒来的房间，有一盏闪烁的应急灯、一张破损的地图和一本残缺的日记（提示怪物弱点和建筑背景）。  
  - 柜子里有一把小手电（基础照明工具，电池有限）和一瓶止痛药（回血道具）。  

- **资源区（食堂/储藏室）**  
  - 可收集罐头、矿泉水（补充体力），但部分食物已腐败（随机判定食用后中毒或恢复）。  
  - 角落里有铁管（初级武器）或消防斧（高攻击但耐久低），需撬开锁住的柜子获取。  

- **危险区（实验室/病房）**  
  - 游荡着低阶怪物（如“裂口仆从”：行动缓慢但成群出现）。  
  - 实验台上可找到血清（暂时提升夜视能力）或钥匙卡（解锁电子门）。  
  - 电脑终端可破解（需迷你游戏）获取建筑地图或关闭安保系统。  

- **Boss区（核心控制室/屋顶）**  
  - 最终逃生出口被巨型怪物“吞噬者”把守，需用收集的炸药或高压电陷阱削弱它。  
  - 逃生需启动发电机（分散在建筑各处）并输入密码（密码碎片藏在文件或尸体上）。  

---

#### **2. 关键收集品**  
- **生存类**  
  - 食物/药品：罐头（饱腹）、止痛药（回血）、肾上腺素（短暂加速）。  
  - 工具：手电升级电池、撬棍（开锁）、对讲机（随机接收幸存者提示）。  

- **武器/防御类**  
  - 近战：匕首（无声但脆弱）、链锯（高伤害但噪音大）。  
  - 远程：手枪（弹药稀缺）、弩箭（可回收）。  
  - 陷阱：捕兽夹、燃烧瓶（需酒精和布料合成）。  

- **剧情类**  
  - 日记页：揭露建筑曾是秘密实验场，怪物是“维度融合”的产物。  
  - 钥匙/门禁卡：解锁捷径或隐藏房间（如军火库）。  

---

#### **3. 怪物设计**  
- **普通怪物**  
  - **“影蠕”**：贴地爬行，弱点是光源（手电直射可击退）。  
  - **“哀嚎者”**：发出噪音吸引其他怪物，需优先击杀。  

- **精英怪物**  
  - **“缝合巨像”**：由多具尸体拼接而成，需破坏其核心（需收集酸液瓶腐蚀外壳）。  

- **环境威胁**  
  - 黑暗值：长时间无光会吸引“暗影生物”，需定期点燃蜡烛或找到安全屋。  
  - 建筑崩塌：部分区域会随机坍塌，逼迫玩家快速决策。  

---

#### **4. 逃生结局**  
- **普通结局**：启动屋顶直升机，但燃料不足仅能单人离开（道德选择：是否带走NPC）。  
- **隐藏结局**：收集全部实验数据并销毁建筑，触发爆炸前从地下隧道逃脱（解锁真相）。  

玩法设定： 你现在需要根据以上内容根据玩家操作对对剧情进行合理的发展，并告知玩家游戏进程。最后给出玩家下一步操作的选项。

最终目标是让玩家有正向反馈的快乐游玩中，逃出实验室。
            ".to_string(),
            role: "system".to_string(),
        }, Message {
            content: content.to_string(),
            role: "user".to_string(),
        }],
        model: "deepseek-chat".to_string(),
        stream: true,
    };

    let response = client
        .post("https://api.deepseek.com/chat/completions")
        .headers(headers)
        .json(&question)
        .send()
        .await
        .map_err(AlterAIError::RequestFailed)?;
    if response.status().is_success() {
        let mut stream = response.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(AlterAIError::RequestFailed)?;
            let chunk_str = String::from_utf8_lossy(&chunk);
            if chunk_str.is_empty() {
                continue;
            }
            for line in chunk_str.lines() {
                let line = line.trim();
                if line.starts_with("data:") {
                    let json_str = line.trim_start_matches("data:").trim();
                    if json_str == "[DONE]" {
                        break;
                    }
                    match serde_json::from_str::<EventSteamData>(&json_str) {
                        Ok(steam_text) => {
                            print!("{}", steam_text.choices[0].delta.content);
                        }
                        Err(err) => {
                            eprintln!("Failed to parse chunk: {}", err);
                            eprintln!("Problematic chunk: {}", line);
                        }
                    }
                } else if line.starts_with("[DONE]") {
                    break;
                }
            }
        }
        Ok(())
    } else {
        Err(AlterAIError::InvalidResponse(format!(
            "Request failed with status: {}",
            response.status()
        )))
    }
}
