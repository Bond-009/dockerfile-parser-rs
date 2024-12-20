use std::convert::TryFrom;

use crate::{error::*, parse_string};
use crate::{Instruction, Pair, Rule, Span, SpannedString};

use snafu::ResultExt;

/// https://docs.docker.com/reference/dockerfile/#user
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UserInstruction {
  pub span: Span,
  pub user: SpannedString,
  pub group: Option<SpannedString>,
}

impl UserInstruction {
  pub(crate) fn from_record(record: Pair) -> Result<UserInstruction> {
    let span = Span::from_pair(&record);
    let mut user: Option<SpannedString> = None;
    let mut group: Option<SpannedString> = None;

    for field in record.into_inner() {
      match field.as_rule() {
        Rule::user_name => user = Some(parse_string(&field)?),
        Rule::user_group => group = Some(parse_string(&field)?),
        _ => return Err(unexpected_token(field)),
      }
    }

    let user = user.ok_or_else(|| Error::GenericParseError {
      message: "user instruction requires a username".into(),
    })?;

    Ok(UserInstruction { span, user, group })
  }
}

impl<'a> TryFrom<&'a Instruction> for &'a UserInstruction {
  type Error = Error;

  fn try_from(instruction: &'a Instruction) -> std::result::Result<Self, Self::Error> {
    if let Instruction::User(e) = instruction {
      Ok(e)
    } else {
      Err(Error::ConversionError {
        from: format!("{:?}", instruction),
        to: "UserInstruction".into(),
      })
    }
  }
}

#[cfg(test)]
mod tests {
  use indoc::indoc;
  use pretty_assertions::assert_eq;

  use super::*;
  use crate::test_util::*;

  #[test]
  fn user_basic() -> Result<()> {
    assert_eq!(
      parse_single("user foo", Rule::user)?,
      UserInstruction {
        span: Span::new(0, 8),
        user: SpannedString {
          span: Span::new(5, 8),
          content: "foo".to_string(),
        },
        group: None
      }
      .into()
    );

    assert_eq!(
      parse_single("user \"foo\"", Rule::user)?,
      UserInstruction {
        span: Span::new(0, 10),
        user: SpannedString {
          span: Span::new(5, 10),
          content: "foo".to_string(),
        },
        group: None
      }
      .into()
    );

    assert_eq!(
      parse_single("user foo bar", Rule::user)?,
      UserInstruction {
        span: Span::new(0, 12),
        user: SpannedString {
          span: Span::new(5, 12),
          content: "foo bar".to_string(),
        },
        group: None
      }
      .into()
    );

    assert_eq!(
      parse_single("user foo\\ bar", Rule::user)?,
      UserInstruction {
        span: Span::new(0, 13),
        user: SpannedString {
          span: Span::new(5, 13),
          content: "foo bar".to_string(),
        },
        group: None
      }
      .into()
    );

    Ok(())
  }
}
