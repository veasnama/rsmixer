use crate::{Letter, DISPATCH};

use tokio::{stream::StreamExt, sync::broadcast::Receiver};

pub async fn start(mut rx: Receiver<Letter>) {
    let mut reader = crossterm::event::EventStream::new();

    loop {
        let input_event = reader.next();
        let recv_event = rx.next();

        tokio::select! {
            ev = input_event => {
                let ev = if let Some(ev) = ev { ev } else { continue; };
                let ev = if let Ok(ev) = ev { ev } else { continue; };

                match ev {
                    crossterm::event::Event::Key(event) => {
                        DISPATCH.event(Letter::KeyPress(event.clone())).await;
                    }
                    crossterm::event::Event::Resize(_, _) => {
                        DISPATCH.event(Letter::Redraw).await;
                    }
                    _ => {}
                };
            }
            ev = recv_event => {
                let ev = if let Some(ev) = ev { ev } else { continue; };
                let ev = if let Ok(ev) = ev { ev } else { continue; };
                if ev == Letter::ExitSignal {
                    break;
                }
            }
        };
    }
}
