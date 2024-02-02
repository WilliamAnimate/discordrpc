use discord_rich_presence::{
    activity::{self, Activity},
    DiscordIpc, DiscordIpcClient,
};
use std::{
    time::{SystemTime, UNIX_EPOCH},
    vec,
};

use crate::cli::Cli;

pub fn run(args: Cli) {
    let state = args.state;
    let details = args.details;
    let large_image = args.large_image;
    let large_text = args.large_text;
    let small_image = args.small_image;
    let small_text = args.small_text;
    let button_1_text = args.button_1_text;
    let button_1_url = args.button_1_url;
    let button_2_text = args.button_2_text;
    let button_2_url = args.button_2_url;
    let enable_timer = args.enable_time;

    let mut client = DiscordIpcClient::new(&args.client_id).expect("failed to create client");

    let activity = Activity::new();

    client
        .connect()
        .expect("Failed to connect to Discord");

    println!("{} {}", "details :", details);

    let mut activity: Activity = activity.details(&details);

    if state != "none" {
        println!("{} {}", "state :", state);

        activity = activity.state(&state);
    }

    let mut assets = activity::Assets::new();

    if large_image != "none" {
        println!("{} {}", "large image :", large_image);

        assets = assets.large_image(&large_image);
    }

    if large_text != "none" {
        println!("{} {}", "large image text :", large_text);

        assets = assets.large_text(&large_text);
    }

    if small_image != "none" {
        println!("{} {}", "small image :", small_image);

        assets = assets.small_image(&small_image);
    }

    if small_text != "none" {
        println!("{} {}", "small image text :", small_text);

        assets = assets.small_text(&small_text);
    }

    activity = activity.assets(assets);

    if button_1_text != "none" && button_1_url != "none" {
        println!("{} {}", "button 1 text :", button_1_text);
        println!("{} {}", "button 1 url :", button_1_url);

        activity = activity.buttons(vec![activity::Button::new(&button_1_text, &button_1_url)]);
    }

    if button_1_text != "none"
        && button_1_url != "none"
        && button_2_text != "none"
        && button_2_url != "none"
    {
        println!("{} {}", "button 2 text :", button_2_text);
        println!("{} {}", "button 2 url :", button_2_url);

        activity = activity.buttons(vec![
            activity::Button::new(&button_1_text, &button_1_url),
            activity::Button::new(&button_2_text, &button_2_url),
        ]);
    }

    if enable_timer {
        let time_unix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        activity = activity.timestamps(activity::Timestamps::new().start(time_unix))
    }

    client.set_activity(activity).expect("client set activity");
    println!("connected!\npress control + c to exit");

    loop {
        std::thread::sleep(std::time::Duration::from_secs(86400));
    }
}
