use super::*;

#[derive(Boilerplate, Debug, PartialEq, Serialize, Deserialize)]
pub struct LuneBalancesHtml {
  pub balances: BTreeMap<SpacedLune, BTreeMap<OutPoint, u128>>,
}

impl PageContent for LuneBalancesHtml {
  fn title(&self) -> String {
    "Lune Balances".to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const LUNE: u128 = 99246114928149462;

  #[test]
  fn display_lune_balances() {
    let balances: BTreeMap<Lune, BTreeMap<OutPoint, u128>> = vec![
      (
        Lune(LUNE),
        vec![(
          OutPoint {
            txid: txid(1),
            vout: 1,
          },
          1000,
        )]
        .into_iter()
        .collect(),
      ),
      (
        Lune(LUNE + 1),
        vec![(
          OutPoint {
            txid: txid(2),
            vout: 2,
          },
          12345678,
        )]
        .into_iter()
        .collect(),
      ),
    ]
    .into_iter()
    .collect();

    assert_regex_match!(
      LuneBalancesHtml { balances }.to_string(),
      "<h1>Lune Balances</h1>
<table>
  <tr>
    <th>lune</th>
    <th>balances</th>
  </tr>
  <tr>
    <td><a href=/lune/AAAAAAAAAAAAA>.*</a></td>
    <td>
      <table>
        <tr>
          <td class=monospace>
            <a href=/output/1{64}:1>1{64}:1</a>
          </td>
          <td class=monospace>
            1000
          </td>
        </tr>
      </table>
    </td>
  </tr>
  <tr>
    <td><a href=/lune/AAAAAAAAAAAAB>.*</a></td>
    <td>
      <table>
        <tr>
          <td class=monospace>
            <a href=/output/2{64}:2>2{64}:2</a>
          </td>
          <td class=monospace>
            12345678
          </td>
        </tr>
      </table>
    </td>
  </tr>
</table>
"
      .unindent()
    );
  }
}
