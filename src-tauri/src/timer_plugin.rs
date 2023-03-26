use std::thread;
use std::time::{Duration, Instant};
use tauri::async_runtime::{channel, Sender};
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{command, Manager, Runtime, State};

const TICK: Duration = Duration::from_secs(1);

type Timer = Sender<u64>;

#[command]
async fn start_timer(duration: u64, timer: State<'_, Timer>) -> Result<(), String> {
    dbg!("start_time", duration);
    timer.send(duration).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
async fn stop_timer(timer: State<'_, Timer>) -> Result<(), String> {
    dbg!("stop_timer");
    timer.send(0).await.map_err(|e| e.to_string())?;
    Ok(())
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("timer")
        .setup(|app_handle| {
            let (tx, mut rx) = channel::<u64>(30);
            let timer = app_handle.clone();
            app_handle.manage(tx);
            tauri::async_runtime::spawn(async move {
                while let Some(mut duration) = rx.recv().await {
                    if duration > 0 {
                        let start_time = Instant::now();
                        loop {
                            let mut time_left =
                                duration.saturating_sub(start_time.elapsed().as_secs());
                            dbg!("tick");
                            timer.emit_all("tick", time_left).unwrap();
                            // handle new message
                            if let Ok(new_duration) = rx.try_recv() {
                                if new_duration == 0 {
                                    time_left = 0;
                                }
                                duration = new_duration;
                            }
                            // handle end of timer
                            if time_left == 0 {
                                break;
                            }
                            // sleep
                            thread::sleep(TICK);
                        }
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![start_timer, stop_timer])
        .build()
}
