use crossterm::event::EventStream;
use ratatui_templates::app::{App, AppResult};
use ratatui_templates::event::{Event, EventsPublisher};
use ratatui_templates::handler::handle_key_events;
use ratatui_templates::tui::Tui;
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    // let app =
    let mut app = App::new();

    // Setup the terminal
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;


    // TODO: create the events pubisher
    let events_publisher= EventsPublisher::new(60);

    // TODO: init the terminal user interface
    let mut tui = Tui::new(terminal, events_publisher);
    tui.init()?;

    // Start the main loop.
    while app.running {
        tui.draw(&mut app)?;
        
        let event = tui.events.next().await?;

        match event {
            Event::Key(event)=>handle_key_events(event, &mut app),
            Event::Tick => Ok(()),
            Event::Mouse(mouse_event) => todo!(),
            Event::Resize(_, _) => todo!(),
        }?;
        
    }
    // while app.running {
        // TODO: Render the user interface.

        // TODO: Handle events.
        // Hint: wait for events and handle them

    // }

    // TODO: Reset the terminal if the app has been terminated
    tui.exit()


    // Ok(())
}
