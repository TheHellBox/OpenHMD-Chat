extern crate time;

use std::time::SystemTime;

pub struct Timer{
    limit: u32,
    time_start: i64
}

impl Timer{
    pub fn new(time: u32) -> Timer{
        let current_time = time::get_time();
        let milliseconds = (current_time.sec as i64 * 1000) +
                       (current_time.nsec as i64 / 1000 / 1000);
        Timer{
            limit: time,
            time_start: milliseconds,
        }
    }
    pub fn get(&self) -> Option<()>{
        let current_time = time::get_time();
        let milliseconds = (current_time.sec as i64 * 1000) +
                       (current_time.nsec as i64 / 1000 / 1000);
        if milliseconds - self.time_start > self.limit as i64{
            Some(())
        }
        else{
            None
        }
    }
    pub fn reset(&mut self){
        let current_time = time::get_time();
        let milliseconds = (current_time.sec as i64 * 1000) +
                       (current_time.nsec as i64 / 1000 / 1000);
        self.time_start = milliseconds;
    }
}
