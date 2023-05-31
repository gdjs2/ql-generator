use serde::{Serialize, Deserialize};
use serde_json::Value;
use ureq::Agent;

use crate::{extractor::Func, constant::{OPENAI_CHAT_COMPLETION_URL, OPENAI_CHAT_COMPLETION_MODEL, ALLOCATOR_PROMPT}};

pub trait Engine {
	fn is_alloactor(&self, f: &Func) -> bool;
}

pub struct CodeQLEngine {
	api_token: String,
	agent: Agent
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
	pub role: String,
	pub content: String
}

#[derive(Serialize, Deserialize, Debug)]
struct PostBody {
	model: String,
	temperature: f64,
	messages: Vec<Message>
}

impl CodeQLEngine {

	pub fn new(t: String) -> Self {
		CodeQLEngine { 
			api_token: t,
			agent: ureq::AgentBuilder::new().build()
		}
	}

	fn post_request(&self, b: PostBody) -> Value {

		log::debug!("[CodeQL Engine] send request: {:?}", &b);

		let rep = self.agent
			.post(OPENAI_CHAT_COMPLETION_URL)
			.set("Content-Type", "application/json")
			.set("Authorization", format!("Bearer {}", self.api_token.as_str()).as_str())
			.send_json(b)
			.unwrap();
		
		log::debug!("[CodeQL Engine] receive response: {:?}", &rep);

		rep.into_json::<Value>().unwrap()["choices"][0]["message"]["content"].clone()

	}


}

impl Engine for CodeQLEngine {
	fn is_alloactor(&self, f: &Func) -> bool {
		let b = PostBody {
			model: OPENAI_CHAT_COMPLETION_MODEL.to_string(),
			temperature: 1.0,
			messages: vec![
				Message { 
					role: "system".to_string(), 
					content: ALLOCATOR_PROMPT.to_string()
				},
				Message {
					role: "user".to_string(),
					content: format!("{}", f)
				}
			]
		};

		let resp = self.post_request(b);

		log::debug!("{:?}", resp);

		true
	}
}