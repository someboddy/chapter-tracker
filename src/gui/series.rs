use gtk::prelude::*;

use crate::models;

pub struct SeriesActor {
    pub widgets: SeriesWidgets,
    pub series: models::Series,
    pub num_episodes: i32,
    pub num_unread: i32,
}

impl actix::Actor for SeriesActor {
    type Context = actix::Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        self.widgets.txt_series_name.get_buffer().unwrap().set_text(&self.series.name);
        self.widgets.cbo_media_type.set_active_id(Some(&self.series.media_type.to_string()));
        self.widgets.tgl_unread.set_label(&format!("{}/{}", self.num_unread, self.num_episodes));
    }
}

#[derive(woab::WidgetsFromBuilder)]
pub struct SeriesWidgets {
    pub row_series: gtk::ListBoxRow,
    pub txt_series_name: gtk::TextView,
    pub cbo_media_type: gtk::ComboBox,
    pub tgl_unread: gtk::ToggleButton,
}

#[derive(woab::BuilderSignal)]
pub enum SeriesSignal {
}

impl actix::StreamHandler<SeriesSignal> for SeriesActor {
    fn handle(&mut self, _signal: SeriesSignal, _ctx: &mut Self::Context) {
    }
}