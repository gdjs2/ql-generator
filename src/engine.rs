use ureq::Agent;

pub trait Engine {
	fn is_alloactor(&self, _func: &str) -> bool {
		return true;
	}
}

pub struct CodeQLEngine {
	api_token: String,
	agent: Agent
}

impl CodeQLEngine {

	pub fn new(t: String) -> Self {
		CodeQLEngine { 
			api_token: t,
			agent: ureq::AgentBuilder::new().build()
		}
	}


}

impl Engine for CodeQLEngine {

}