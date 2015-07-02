use fccore::fcconfig::FCConfig;
use std::thread::{spawn, JoinHandle};

pub struct FCCore {
  armed : bool,
  config : FCConfig
}

impl FCCore {
  pub fn new(config_file : &str) -> FCCore {
    
    let mut core = FCCore{
      armed: false,
      config: FCConfig::new(config_file)
    };

    core.start_thread();
    return core;
  }
  
  fn start_thread(&mut self) {
    spawn(move || {
      FCCore::fccore_thread_loop(self); 
    });    
  }
  
  fn fccore_thread_loop(core : &mut FCCore) {
  }
}
