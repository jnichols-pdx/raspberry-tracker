use crate::session::*;
use egui::*;

pub struct SessionList {
    sessions: Vec<Session>,
    selected: Option<usize>,
}

impl SessionList {
    pub fn new() -> Self {
        SessionList {
            sessions: Vec::<Session>::new(),
            selected: None,
        }
    }
    pub fn new_from_vec(sessions: Vec<Session>) -> Self {
        SessionList {
            sessions,
            selected: None,
        }
    }
    pub fn push(&mut self, new_session: Session) {
        self.sessions.push(new_session);
        self.selected = Some(self.sessions.len() - 1);
    }

    pub fn selected(&self) -> Option<&Session> {
        if self.selected.is_some() {
            self.sessions.get(self.selected.unwrap())
        } else {
            None
        }
    }

    pub fn active_session(&self) -> Option<&Session> {
        if let Some(session) = self.sessions.last() {
            if session.is_active() {
                Some(session)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn active_session_mut(&mut self) -> Option<&mut Session> {
        if let Some(session) = self.sessions.last_mut() {
            if session.is_active() {
                Some(session)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.sessions.len()
    }

    pub fn current_mut(&mut self) -> Option<&mut Session> {
        self.sessions.last_mut()
    }

    pub fn ui(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) -> bool{
        let selected_before = self.selected;
        ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
            let text_style = TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            ScrollArea::vertical().show_rows(
                ui,
                row_height,
                self.sessions.len(),
                |ui, row_range|
            {
                let rev = self.sessions.iter().rev();
                let visible_length = row_range.end - row_range.start;
                let mut shown = 0;
                let count_sessions = self.sessions.len();
                let selected_index = self.selected.unwrap_or(usize::MAX);
                for session in rev.skip(row_range.start) {
                    let session_index = count_sessions - row_range.start - shown - 1;
                    let this_text = if session_index == selected_index {
                       format!("{} {}▶", session.current_character().name_with_outfit(), session.duration_string())
                    } else {
                       format!("{} {}", session.current_character().name_with_outfit(), session.duration_string())
                    };

                    if ui.add(Label::new(this_text)
                        .sense(Sense::click()))
                        .clicked()
                    {
                        self.selected = Some(session_index);
                    }
                    shown += 1;
                    if shown > visible_length {
                        break;
                    }
                }
            });
        });
        selected_before != self.selected
    }
}

impl Default for SessionList {
    fn default() -> Self {
        Self::new()
    }
}

  
