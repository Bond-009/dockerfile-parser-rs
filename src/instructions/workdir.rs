use std::convert::TryFrom;

use crate::error::*;
use crate::{Instruction, Pair, Rule, Span, SpannedString};

use snafu::ResultExt;

/// https://docs.docker.com/reference/dockerfile/#workdir
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WorkdirInstruction {
    pub span: Span,
    pub workdir: SpannedString,
}

#[cfg(test)]
mod tests {
  use indoc::indoc;
  use pretty_assertions::assert_eq;

  use super::*;
  use crate::test_util::*;
  use crate::Dockerfile;

  #[test]
  fn expose() -> Result<()> {
    assert_eq!(
      parse_single(r#"expose 8000"#, Rule::expose)?
        .into_expose()
        .unwrap(),
      ExposeInstruction {
        span: Span::new(0, 11),
        vars: vec![ExposePort::new(
          Span::new(7, 11),
          SpannedShort {
            span: Span::new(7, 11),
            content: 8000,
          },
          None,
        )],
      }
    );

    Ok(())
  }
}
