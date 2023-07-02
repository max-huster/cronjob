extern crate cronjob;
extern crate cron;

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use cron::Schedule;
    use cronjob::CronJob;

    #[test]
    fn test_cronjob() {
        let mut cron = CronJob::new("Test Cron", Schedule::from_str("0-10 * * * * *").unwrap(), on_cron);
        cron.set_checking_interval(1000);
        cron.offset(2);
        cron.start_job();
    }

    fn on_cron(name: &str) {
        println!("{}: It's time!", name);
    }
}
