use super::common::*;

use crate::{
    entry::{EntryIdentifier, HiddenStatus},
    ui::util::parent_child_types,
};

use std::collections::{HashMap, HashSet};

pub async fn action_handler(msg: &Action, state: &mut RSState) -> RedrawType {
    // we only need to update page entries if entries changed
    match msg {
        Action::Redraw
        | Action::EntryRemoved(_)
        | Action::EntryUpdate(_, _)
        | Action::ChangePage(_) => {}

        Action::Hide => {
            if let Some(selected) = state.page_entries.get(state.selected) {
                state.entries.hide(selected);
            }
        }
        _ => {
            return RedrawType::None;
        }
    };

    let last_sel = state.page_entries.get(state.selected);

    let (p, c) = parent_child_types(state.current_page);

    let mut parents = HashSet::new();
    state.entries.iter_type(c).for_each(|(_, e)| {
        parents.insert(e.parent);
    });

    for (_, p_e) in state.entries.iter_type_mut(p) {
        p_e.hidden = match parents.get(&Some(p_e.index)) {
            Some(_) => HiddenStatus::HiddenKids,
            None => HiddenStatus::NoKids,
        };
    }

    let entries_changed = state.page_entries.set(
        state
            .current_page
            .generate_page(&state.entries, &state.ui_mode)
            .map(|x| *x.0)
            .collect::<Vec<EntryIdentifier>>(),
        p,
    );

    match state.ui_mode {
        UIMode::MoveEntry(ident, _) => {
            if let Some(i) = state.page_entries.iter_entries().position(|&x| x == ident) {
                state.selected = i;
            }
        }
        _ => {
            if let Some(i) = state
                .page_entries
                .iter_entries()
                .position(|&x| Some(x) == last_sel)
            {
                state.selected = i;
            }
        }
    };

    if entries_changed {
        DISPATCH
            .event(Action::CreateMonitors(
                if state.current_page != PageType::Cards {
                    monitor_list(state)
                } else {
                    HashMap::new()
                },
            ))
            .await;

        RedrawType::Entries
    } else {
        RedrawType::None
    }
}

fn monitor_list(state: &mut RSState) -> HashMap<EntryIdentifier, Option<u32>> {
    let mut monitors = HashMap::new();
    state.page_entries.iter_entries().for_each(|ident| {
        if let Some(entry) = state.entries.get(ident) {
            monitors.insert(
                EntryIdentifier::new(entry.entry_type, entry.index),
                entry.monitor_source(&state.entries),
            );
        }
    });

    log::error!("{:?}", monitors);

    monitors
}
