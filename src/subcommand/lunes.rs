use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Output {
  pub lunes: BTreeMap<Lune, LuneInfo>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LuneInfo {
  pub block: u64,
  pub burned: u128,
  pub divisibility: u8,
  pub etching: Txid,
  pub height: u64,
  pub id: LuneId,
  pub index: u32,
  pub terms: Option<Terms>,
  pub mints: u128,
  pub number: u64,
  pub premine: u128,
  pub lune: Lune,
  pub spacers: u32,
  pub supply: u128,
  pub symbol: Option<char>,
  pub timestamp: DateTime<Utc>,
  pub turbo: bool,
  pub tx: u32,
}

pub(crate) fn run(options: Options) -> SubcommandResult {
  let index = Index::open(&options)?;

  ensure!(
    index.has_lune_index(),
    "`ord lunes` requires index created with `--index-lunes` flag",
  );

  index.update()?;

  Ok(Box::new(Output {
    lunes: index
      .lunes()?
      .into_iter()
      .map(
        |(
          id,
          entry @ LuneEntry {
            block,
            burned,
            divisibility,
            etching,
            terms,
            mints,
            number,
            premine,
            lune,
            spacers,
            supply,
            symbol,
            timestamp,
            turbo,
          },
        )| {
          (
            lune,
            LuneInfo {
              block,
              burned,
              divisibility,
              etching,
              height: id.height,
              id,
              index: id.index,
              terms,
              mints,
              number,
              premine,
              timestamp: crate::timestamp(timestamp),
              lune,
              spacers,
              supply,
              symbol,
              turbo,
              tx: id.index,
            },
          )
        },
      )
      .collect::<BTreeMap<Lune, LuneInfo>>(),
  }))
}
