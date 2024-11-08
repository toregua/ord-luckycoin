use super::*;

#[derive(Copy, Clone, Debug, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub struct SpacedLune {
  pub(crate) lune: Lune,
  pub(crate) spacers: u32,
}

impl SpacedLune {
  pub fn new(lune: Lune, spacers: u32) -> Self {
    Self { lune, spacers }
  }
}

impl FromStr for SpacedLune {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut lune = String::new();
    let mut spacers = 0u32;

    for c in s.chars() {
      match c {
        'A'..='Z' => lune.push(c),
        '.' | '•' => {
          let flag = 1 << lune.len().checked_sub(1).context("leading spacer")?;
          if spacers & flag != 0 {
            bail!("double spacer");
          }
          spacers |= flag;
        }
        _ => bail!("invalid character"),
      }
    }

    if 32 - spacers.leading_zeros() >= lune.len().try_into().unwrap() {
      bail!("trailing spacer")
    }

    Ok(SpacedLune {
      lune: lune.parse()?,
      spacers,
    })
  }
}

impl Display for SpacedLune {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    let lune = self.lune.to_string();

    for (i, c) in lune.chars().enumerate() {
      write!(f, "{c}")?;

      if i < lune.len() - 1 && self.spacers & 1 << i != 0 {
        write!(f, "•")?;
      }
    }

    Ok(())
  }
}

impl Serialize for SpacedLune {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.collect_str(self)
  }
}

impl<'de> Deserialize<'de> for SpacedLune {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    Ok(DeserializeFromStr::deserialize(deserializer)?.0)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn display() {
    assert_eq!("A.B".parse::<SpacedLune>().unwrap().to_string(), "A•B");
    assert_eq!("A.B.C".parse::<SpacedLune>().unwrap().to_string(), "A•B•C");
  }

  #[test]
  fn from_str() {
    #[track_caller]
    fn case(s: &str, lune: &str, spacers: u32) {
      assert_eq!(
        s.parse::<SpacedLune>().unwrap(),
        SpacedLune {
          lune: lune.parse().unwrap(),
          spacers
        },
      );
    }

    assert_eq!(
      ".A".parse::<SpacedLune>().unwrap_err().to_string(),
      "leading spacer",
    );

    assert_eq!(
      "A..B".parse::<SpacedLune>().unwrap_err().to_string(),
      "double spacer",
    );

    assert_eq!(
      "A.".parse::<SpacedLune>().unwrap_err().to_string(),
      "trailing spacer",
    );

    assert_eq!(
      "Ax".parse::<SpacedLune>().unwrap_err().to_string(),
      "invalid character",
    );

    case("A.B", "AB", 0b1);
    case("A.B.C", "ABC", 0b11);
    case("A•B", "AB", 0b1);
    case("A•B•C", "ABC", 0b11);
  }

  #[test]
  fn serde() {
    let spaced_lune = SpacedLune {
      lune: Lune(26),
      spacers: 1,
    };
    let json = "\"A•A\"";
    assert_eq!(serde_json::to_string(&spaced_lune).unwrap(), json);
    assert_eq!(
      serde_json::from_str::<SpacedLune>(json).unwrap(),
      spaced_lune
    );
  }
}
