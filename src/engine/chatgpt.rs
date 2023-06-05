use std::{thread, time::Duration};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use ureq::Agent;

use crate::{
    constant::{ALLOCATOR_PROMPT, OPENAI_CHAT_COMPLETION_MODEL, OPENAI_CHAT_COMPLETION_URL, DEALLOCATOR_PROMPT},
    extractor::Func,
};

use super::Engine;

/**
This is an implementation of ChatGPT Engine, utilizing
ChatGPT to decide the classification of functions.
*/
pub struct ChatGPTEngine {
    api_token: String,
    agent: Agent,
}

/**
Single message sent to ChatGPT.
Further information can be got from [link](https://platform.openai.com/docs/api-reference/chat/create)
*/
#[derive(Serialize, Deserialize, Debug)]
struct Message {
    pub role: String,
    pub content: String,
}

/**
Post body that should be sent to ChatGPT API.
Further information can be got from [link](https://platform.openai.com/docs/api-reference/chat/create)
*/
#[derive(Serialize, Deserialize, Debug)]
struct PostBody {
    model: String,
    temperature: f64,
    messages: Vec<Message>,
}

/**
Some functions implementation for ChatGPT Engine.
*/
impl ChatGPTEngine {
    /**
    Create a new ChatGPT Engine.

    * t: [`String`], OpenAI API Token
    */
    pub fn new(t: String) -> Self {
        ChatGPTEngine {
            api_token: t,
            agent: ureq::AgentBuilder::new().build(),
        }
    }

    /**
    Post a body to the chat completion API.

    * b: [`PostBody`], the body should be posted
    */
    fn post_request(&self, b: PostBody) -> Value {
        log::debug!("[CodeQL Engine] send request: {:?}", &b);

        let rep = self
            .agent
            .post(OPENAI_CHAT_COMPLETION_URL)
            .set("Content-Type", "application/json")
            .set(
                "Authorization",
                format!("Bearer {}", self.api_token.as_str()).as_str(),
            )
            .send_json(b)
            .unwrap();

        log::debug!("[CodeQL Engine] receive response: {:?}", &rep);

        rep.into_json::<Value>().unwrap()["choices"][0]["message"]["content"].clone()
    }
}

/**
Implementation of Engine for ChatGPT Engine.
*/
impl Engine for ChatGPTEngine {
    fn is_allocator(&self, f: &Func) -> bool {
        let b = PostBody {
            model: OPENAI_CHAT_COMPLETION_MODEL.to_string(),
            temperature: 1.0,
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: ALLOCATOR_PROMPT.to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: format!("{}", f),
                },
            ],
        };

        let resp = serde_json::from_str::<Value>(self.post_request(b).as_str().unwrap()).unwrap();
        log::debug!("[is_allocator]: resp{{{:?}}}", resp);

        let res = resp["result"].as_str().unwrap();

        log::debug!("[is_allocator]: res{{{}}}", res);

        thread::sleep(Duration::from_millis(300));

        if res == "Yes" {
            return true;
        } else {
            return false;
        }
    }

    fn is_deallocator(&self, f: &Func) -> bool {
        let b = PostBody {
            model: OPENAI_CHAT_COMPLETION_MODEL.to_string(),
            temperature: 1.0,
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: DEALLOCATOR_PROMPT.to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: format!("{}", f),
                },
            ],
        };

        let resp = serde_json::from_str::<Value>(self.post_request(b).as_str().unwrap()).unwrap();
        log::debug!("[is_deallocator]: resp{{{:?}}}", resp);

        let res = resp["result"].as_str().unwrap();

        log::debug!("[is_deallocator]: res{{{}}}", res);

        thread::sleep(Duration::from_millis(300));

        if res == "Yes" {
            return true;
        } else {
            return false;
        }
    }
}
