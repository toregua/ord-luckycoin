use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Output {
  pub lunes: BTreeMap<SpacedLune, BTreeMap<OutPoint, u128>>,
}

pub(crate) fn run(options: Options) -> SubcommandResult {
  let index = Index::open(&options)?;

  ensure!(
    index.has_lune_index(),
    "`ord balances` requires index created with `--index-lunes` flag",
  );

  index.update()?;

  Ok(Box::new(Output {
    lunes: index.get_lune_balance_map()?,
  }))
}
