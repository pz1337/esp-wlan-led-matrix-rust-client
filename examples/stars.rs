use std::time::Duration;

use esp_wlan_led_matrix_client::async_tokio::Client;
use rand::Rng;
use tokio::task;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let addr = "espPixelmatrix:1337";

    task::spawn(async move {
        loop {
            if let Err(err) = stars(addr).await {
                println!("pixelflut_stars ERROR {}", err);
            }
            sleep(Duration::from_secs(5)).await;
        }
    });

    let handle = task::spawn(async move {
        loop {
            if let Err(err) = stars(addr).await {
                println!("pixelflut_stars ERROR {}", err);
            }
            sleep(Duration::from_secs(5)).await;
        }
    });

    // wait for the task to end which runds in an endless loop
    handle.await.unwrap();
}

async fn stars(addr: &str) -> std::io::Result<()> {
    let mut client = Client::connect(addr).await?;

    loop {
        let (dur, x, y) = {
            let mut rng = rand::thread_rng();
            let dur = rng.gen_range(0..800);
            let x = rng.gen_range(0..client.width());
            let y = rng.gen_range(0..client.height());
            (dur, x, y)
        };

        println!("star {:3} {:3} followed by {:4} ms sleep", x, y, dur);

        for bri in &[255_u8, 100, 0] {
            let bri = bri.to_owned();
            client.pixel(x, y, bri, bri, bri).await?;
            client.flush().await?;
            sleep(Duration::from_millis(150)).await;
        }

        sleep(Duration::from_millis(dur)).await;
    }
}
