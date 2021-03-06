use crate::{
    entry::{Entry, EntryIdentifier},
    models::PageType,
};

use ev_apple::{messages, Message};

use std::collections::HashMap;

use pulse::volume::ChannelVolumes;

use crossterm::event::KeyEvent;

use statics::*;

messages!(Action,
    ExitSignal => EXIT_MESSAGE_ID,

    // redraw the whole screen (called every window resize)
    Redraw => MAIN_MESSAGE,

    // entry updates
    EntryRemoved(EntryIdentifier) => MAIN_MESSAGE,
    EntryUpdate(EntryIdentifier, Box<Entry>) => MAIN_MESSAGE,
    PeakVolumeUpdate(EntryIdentifier, f32) => MAIN_MESSAGE,

    // move around the UI
    MoveUp(u16) => MAIN_MESSAGE,
    MoveDown(u16) => MAIN_MESSAGE,
    ChangePage(PageType) => MAIN_MESSAGE,
    // positive - forwards, negative - backwards
    CyclePages(i8) => MAIN_MESSAGE,

    // volume changes
    RequestMute => MAIN_MESSAGE,
    InputVolumeValue => MAIN_MESSAGE,
    // request volume change where the argument is a
    // number of percentage points it should be changed by
    RequstChangeVolume(i16) => MAIN_MESSAGE,

    // context menus
    OpenContextMenu => MAIN_MESSAGE,
    CloseContextMenu => MAIN_MESSAGE,
    Confirm => MAIN_MESSAGE,

    ShowHelp => MAIN_MESSAGE,

    Hide => MAIN_MESSAGE,

    // PulseAudio connection status
    RetryIn(u64) => MAIN_MESSAGE,
    ConnectToPA => MAIN_MESSAGE,
    PADisconnected => MAIN_MESSAGE,

    KeyPress(KeyEvent) => MAIN_MESSAGE,

    MuteEntry(EntryIdentifier, bool) => PA_MESSAGE,
    MoveEntryToParent(EntryIdentifier, EntryIdentifier) => PA_MESSAGE,
    ChangeCardProfile(EntryIdentifier, String) => PA_MESSAGE,
    SetVolume(EntryIdentifier, ChannelVolumes) => PA_MESSAGE,
    CreateMonitors(HashMap<EntryIdentifier, Option<u32>>) => PA_MESSAGE,
    SetSuspend(EntryIdentifier, bool) => PA_MESSAGE,
    KillEntry(EntryIdentifier) => PA_MESSAGE,
    PADisconnected2 => PA_MESSAGE,
);

pub mod statics {
    pub static CHANNEL_CAPACITY: usize = 32;

    pub static MAIN_MESSAGE: u32 = 1;
    pub static PA_MESSAGE: u32 = 2;
    pub static RUN_PA_MESSAGE: u32 = 3;
    pub static INPUT_MESSAGE: u32 = 4;

    pub static EXIT_MESSAGE_ID: u32 = 0;
}
