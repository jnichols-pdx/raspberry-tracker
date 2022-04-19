use crate::common::{relative_date_string, small_date_format};
use crate::session::*;
use egui::*;
use time::{Date, OffsetDateTime};
use time_tz::{OffsetDateTimeExt, Tz};

pub struct SessionList {
    sessions: Vec<Session>,
    selected: Option<usize>,
    display_sequence: Vec<RowType>,
    latest_session_date: Option<Date>,
    time_zone: &'static Tz,
}

enum RowType {
    Date(Date),
    Session(usize),
}

impl SessionList {
    pub fn new() -> Self {
        SessionList {
            sessions: Vec::<Session>::new(),
            selected: None,
            display_sequence: Vec::<RowType>::new(),
            latest_session_date: None,
            time_zone: time_tz::system::get_timezone().unwrap(),
        }
    }

    pub fn new_from_vec(sessions: Vec<Session>) -> Self {
        let mut display_sequence = Vec::<RowType>::new();
        let mut last_date_seen = None;
        let mut counter = 0;
        for session in sessions.iter() {
            let session_date = session.local_start_date();
            if last_date_seen.is_none() {
                last_date_seen = Some(session_date);
            } else if session_date != last_date_seen.unwrap() {
                display_sequence.push(RowType::Date(last_date_seen.unwrap()));
                last_date_seen = Some(session_date);
            }
            display_sequence.push(RowType::Session(counter));
            counter += 1;
        }
        if counter > 0 {
            display_sequence.push(RowType::Date(last_date_seen.unwrap()));
        }
        println!("{} distinct dates", display_sequence.len() - sessions.len());
        SessionList {
            sessions,
            selected: None,
            display_sequence,
            latest_session_date: last_date_seen,
            time_zone: time_tz::system::get_timezone().unwrap(),
        }
    }

    pub fn push(&mut self, new_session: Session) {
        if self.latest_session_date.is_none() {
            self.latest_session_date = Some(new_session.local_start_date());
        } else {
            let new_date = new_session.local_start_date();
            if new_date != self.latest_session_date.unwrap() {
                self.latest_session_date = Some(new_date);
                self.display_sequence
                    .push(RowType::Session(self.sessions.len()));
                self.display_sequence.push(RowType::Date(new_date));
            } else {
                //Adding to the same day - need to ensure the 'date' stays in front of all sessions
                //from this day.
                let date_to_keep = self.display_sequence.pop().unwrap();
                self.display_sequence
                    .push(RowType::Session(self.sessions.len()));
                self.display_sequence.push(date_to_keep);
            }
        }
        self.selected = Some(self.sessions.len());
        self.sessions.push(new_session);
    }

    pub fn selected_mut(&mut self) -> Option<&mut Session> {
        if self.selected.is_some() {
            self.sessions.get_mut(self.selected.unwrap())
        } else {
            None
        }
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

    pub fn ui(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) -> bool {
        let mut user_clicked_a_session = false;
        ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
            let text_style = TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            ScrollArea::vertical().show_rows(
                ui,
                row_height,
                self.display_sequence.len(),
                |ui, row_range| {
                    let visible_length = row_range.end - row_range.start;
                    let now_date = OffsetDateTime::now_utc().to_timezone(self.time_zone).date();
                    let selected_index = self.selected.unwrap_or(usize::MAX);
                    let sequence = self.display_sequence.iter().rev();
                    let mut shown = 0;

                    for row in sequence.skip(row_range.start) {
                        match row {
                            RowType::Date(date) => {
                                ui.label(relative_date_string(date, &now_date))
                                    .on_hover_text(small_date_format(date));
                            }
                            RowType::Session(index) => {
                                let session = &self.sessions[*index];
                                let this_text = if *index == selected_index {
                                    format!(
                                        "{} {}▶",
                                        session.current_character().name_with_outfit(),
                                        session.duration_string()
                                    )
                                } else {
                                    format!(
                                        "{} {}     ",
                                        session.current_character().name_with_outfit(),
                                        session.duration_string()
                                    )
                                };

                                if ui
                                    .add(Label::new(this_text).sense(Sense::click()))
                                    .clicked()
                                {
                                    self.selected = Some(*index);
                                    user_clicked_a_session = true;
                                }
                            }
                        }
                        shown += 1;
                        if shown > visible_length {
                            break;
                        }
                    }
                },
            );
        });

        user_clicked_a_session
    }
}

impl Default for SessionList {
    fn default() -> Self {
        Self::new()
    }
}
