use super::*;

#[derive(Boilerplate)]
pub(crate) struct LunesHtml {
  pub(crate) entries: Vec<(LuneId, LuneEntry)>,
}

impl PageContent for LunesHtml {
  fn title(&self) -> String {
    "Lunes".to_string()
  }
}
