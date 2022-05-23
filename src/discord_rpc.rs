use std::time::{Duration, SystemTime, UNIX_EPOCH};

use discord_rich_presence::activity::{self, Activity};
use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
use sysinfo::{System, SystemExt};

/*pub fn start_rpc(file_name: &str, time_stamp: u64) {
    #[allow(clippy::unreadable_literal)]
    let mut drpc = DiscordRPC::new(978157973845192734);
    let s = System::new_all();
    // Check if Discord is open
    if s.processes_by_name("Discord").peekable().peek().is_some() {
        drpc.start();

        loop {
            drpc.set_activity(|a| a.details(format!("Editing {}", file_name).timestamp()))
                .ok();
        }
    }
}*/

pub struct Rpc {
    client: DiscordIpcClient,
    file_name: String,
    timestamp: activity::Timestamps,
}

#[allow(clippy::unreadable_literal)]
impl Rpc {
    pub fn from(file_name: String) -> Self {
        #[allow(clippy::cast_possible_wrap)]
        let time_unix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .ok()
            .unwrap_or(Duration::from_secs(0))
            .as_secs() as i64;
        let timestamp = activity::Timestamps::new().start(time_unix);

        Self {
            client: DiscordIpcClient::new("978157973845192734").expect("failed to create client"),
            file_name,
            timestamp,
        }
    }

    pub fn start(&mut self) {
        let s = System::new_all();

        if s.processes_by_name("Discord").peekable().peek().is_some() {
            self.client.connect().ok();

            let details = format!("Editing {}", self.file_name);
            let activity = Activity::new()
                .details(&details)
                .timestamps(self.timestamp.clone());

            self.client.set_activity(activity).ok();
        }
    }

    pub fn file_name(&mut self, new_name: String) {
        self.file_name = new_name;

        self.update();
    }

    fn update(&mut self) {
        let s = System::new_all();

        if s.processes_by_name("Discord").peekable().peek().is_some() {
            let details = format!("Editing {}", self.file_name);
            let activity = Activity::new()
                .details(&details)
                .timestamps(self.timestamp.clone());

            self.client.set_activity(activity).ok();
        }
    }
}
