use std::time::SystemTime;

pub struct Timer{
    limit: u32,
    sys_time: SystemTime
}

impl Timer{
    pub fn new(time: u32) -> Timer{
        let sys_time = SystemTime::now();
        Timer{
            limit: time,
            sys_time: sys_time
        }
    }
    pub fn get(&self) -> Option<()>{
        let elapsed = self.sys_time.elapsed().unwrap();
        let elapsed = 1000 / ((elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64 + 1) as u32;
        if elapsed > self.limit{
            Some(())
        }
        else{
            None
        }
    }
    pub fn reset(&mut self){
        self.sys_time = SystemTime::now();
    }
}
