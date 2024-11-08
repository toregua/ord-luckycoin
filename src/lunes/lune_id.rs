use {super::*, std::num::TryFromIntError};

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq, Ord, PartialOrd)]
pub struct LuneId {
  pub height: u64,
  pub index: u32,
}

impl TryFrom<u128> for LuneId {
  type Error = TryFromIntError;

  fn try_from(n: u128) -> Result<Self, Self::Error> {
    Ok(Self {
      height: u64::try_from(n >> 16)?,
      index: u32::try_from(n & 0xFFFF).unwrap(),
    })
  }
}

impl From<LuneId> for u128 {
  fn from(id: LuneId) -> Self {
    u128::from(id.height) << 16 | u128::from(id.index)
  }
}

impl Display for LuneId {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}:{}", self.height, self.index,)
  }
}

impl FromStr for LuneId {
  type Err = crate::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let (height, index) = s
      .split_once(':')
      .ok_or_else(|| anyhow!("invalid lune ID: {s}"))?;

    Ok(Self {
      height: height.parse()?,
      index: index.parse()?,
    })
  }
}

impl Serialize for LuneId {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.collect_str(self)
  }
}

impl<'de> Deserialize<'de> for LuneId {
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
  fn lune_id_to_128() {
    assert_eq!(
      0b11_0000_0000_0000_0001u128,
      LuneId {
        height: 3,
        index: 1,
      }
      .into()
    );
  }

  #[test]
  fn display() {
    assert_eq!(
      LuneId {
        height: 1,
        index: 2
      }
      .to_string(),
      "1:2"
    );
  }

  #[test]
  fn from_str() {
    assert!(":".parse::<LuneId>().is_err());
    assert!("1:".parse::<LuneId>().is_err());
    assert!(":2".parse::<LuneId>().is_err());
    assert!("a:2".parse::<LuneId>().is_err());
    assert!("1:a".parse::<LuneId>().is_err());
    assert_eq!(
      "1:2".parse::<LuneId>().unwrap(),
      LuneId {
        height: 1,
        index: 2
      }
    );
  }

  #[test]
  fn try_from() {
    assert_eq!(
      LuneId::try_from(0x060504030201).unwrap(),
      LuneId {
        height: 0x06050403,
        index: 0x0201
      }
    );

    assert!(LuneId::try_from(0x07060504030201).is_err());
  }

  #[test]
  fn serde() {
    let lune_id = LuneId {
      height: 1,
      index: 2,
    };
    let json = "\"1:2\"";
    assert_eq!(serde_json::to_string(&lune_id).unwrap(), json);
    assert_eq!(serde_json::from_str::<LuneId>(json).unwrap(), lune_id);
  }
}
