use chrono::{FixedOffset, Local};
use cron::Schedule;

use std::str::FromStr;
use std::thread;
use std::time::Duration;

use command::Command;

/// The object to create and execute cronjobs for yout application.
pub struct CronJob {
    name: String,
    command: Box<dyn Command>,
    offset: Option<FixedOffset>,
    schedule: Schedule,
    interval: u64,
}

impl CronJob {
    /// Constructs new `CronJob` object.
    pub fn new<C: Command>(name: &str, schedule: Schedule, command: C) -> Self {
        
        CronJob {
            name: name.to_string(),
            command: Box::new(command),
            offset: None,
            schedule: schedule,
            interval: 500,            
        }
    }
    
    pub fn offset(&mut self, timezone_offset: i32) -> &mut Self {
        self.offset = Some(FixedOffset::east(timezone_offset));
        self
    }
    /// Set checking interval in millis
    pub fn set_checking_interval(&mut self, interval: u64) -> &mut Self {
        self.interval = interval;
        self
    }

    /// Returns the schedule for the cronjob, with this you are able to get the next occurences.
    pub fn get_schedule(&self) -> Schedule {
        self.schedule.to_owned()
    }

    /// Starts the cronjob without threading.
    pub fn start_job(&mut self) {
        let schedule = self.get_schedule();
        let offset = self.offset.unwrap_or_else(|| FixedOffset::east(0));

        loop {
            let mut upcoming = schedule.upcoming(offset).take(1);
            thread::sleep(Duration::from_millis(self.interval));
            let local = &Local::now();

            if let Some(datetime) = upcoming.next() {
                if datetime.timestamp() <= local.timestamp() {
                    self.command.execute(&self.name);
                }
            }
        }
    }

    /// Starts the cronjob with threading. Stops when application quits.
    pub fn start_job_threaded(mut cronjob: CronJob) {
        thread::Builder::new()
            .name(cronjob.name.clone())
            .spawn(move || cronjob.start_job())
            .expect("There was an error in an cronjob");
    }
}
