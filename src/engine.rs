pub trait Engine {
	fn is_alloactor(&self, _func: &str) -> bool {
		return true;
	}
}

pub struct DefaultEngine {

}

impl Engine for DefaultEngine {
	
}