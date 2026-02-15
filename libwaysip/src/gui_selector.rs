use std::sync::{Arc, mpsc};
use std::time::Duration;

use iced::Theme;
use iced_layershell::daemon;
use iced_layershell::reexport::{Anchor, KeyboardInteractivity};
use iced_layershell::settings::{LayerShellSettings, Settings};
use libwayshot::WayshotConnection;
use libwayshot::output::OutputInfo;
use libwayshot::region::TopLevel;

use crate::iced_selector::IcedSelector;

/// Interface struct to start a GUI area selector and retrieve its result
#[derive(Default)]
pub struct AreaSelectorGUI {
    conn: Option<WayshotConnection>,
}

/// Represents the user's selection made through interaction with the GUI area selector
pub enum GUISelection {
    Toplevel(TopLevel),
    Output(OutputInfo),
    Failed,
}

impl AreaSelectorGUI {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_connection(mut self, conn: WayshotConnection) -> Self {
        self.conn = Some(conn);
        self
    }

    /// Launches a GUI area selector
    pub fn launch(self) -> GUISelection {
        let (tx, rx) = mpsc::channel::<GUISelection>();
        let conn = Arc::new(match self.conn {
            Some(conn) => conn,
            None => WayshotConnection::new().expect("Couldn't establish a Wayshot connection"),
        });

        let _ = daemon(
            move || IcedSelector::new(tx.clone(), conn.clone()),
            IcedSelector::namespace,
            IcedSelector::update,
            IcedSelector::view,
        )
        .title(IcedSelector::title)
        .settings(Settings {
            layer_settings: LayerShellSettings {
                size: Some((400, 400)),
                exclusive_zone: 0,
                anchor: Anchor::Bottom | Anchor::Left | Anchor::Right | Anchor::Top,
                keyboard_interactivity: KeyboardInteractivity::None,
                ..Default::default()
            },
            ..Default::default()
        })
        .theme(Theme::Dark)
        .run();

        // Gets the selection from the GUI
        rx.recv_timeout(Duration::from_secs(1))
            .unwrap_or(GUISelection::Failed)
    }
}
