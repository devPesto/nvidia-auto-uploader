// mod uploader;

use serde_json::json;
use std::{error::Error, time::Duration};
use thirtyfour::{prelude::*, support::sleep};
// fn main() -> Result<()> {
//     let (tx, rx) = mpsc::channel::<Result<Event>>();

//     let mut watcher = recommended_watcher(tx)?;

//     // Add a path to be watched. All files and directories at that path and
//     // below will be monitored for changes.
//     watcher.watch(
//         Path::new("/home/pesto/Desktop/auto-uploader/test/sample.txt"),
//         RecursiveMode::Recursive,
//     )?;
//     // Block forever, printing out events as they come in
//     for res in rx {
//         match res {
//             Ok(event) => println!("event: {:?}", event),
//             Err(e) => println!("watch error: {:?}", e),
//         }
//     }

//     Ok(())
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut caps = DesiredCapabilities::chrome();
    caps.set_page_load_strategy(thirtyfour::PageLoadStrategy::Normal)?;
    // caps.add_arg("--headless=new")?; // comment out in development

    let email_addr = "amirseman321@gmail.com";
    let driver = WebDriver::new("http://localhost:1234", caps).await?;
    driver.goto("https://accounts.google.com/").await?;

    // Enter email
    let email_field = driver.find(By::Tag("input")).await?;
    email_field.click().await?;
    email_field.send_keys(email_addr).await?;

    // Click next
    let next_btn = driver.find(By::LinkText("Next")).await?;
    next_btn.click().await?;

    sleep(Duration::from_secs(3)).await;
    Ok(())
}
