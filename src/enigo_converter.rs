use super::{Keyboard::{self as KB, *}, Mouse::{self as M, *}, Error, OptionToResult as _, decl_keycode};

use enigo::Key as EnigoKey;
use enigo::Button as EnigoButton;

pub trait Keyboard {
  fn to_enigo_key(&self) -> Option<EnigoKey>;
}

pub trait Mouse {
  fn to_enigo_button(&self) -> Option<EnigoButton>;
}

impl Keyboard for KB {
  fn to_enigo_key(&self) -> Option<EnigoKey> {
    match self {
      A => Some(EnigoKey::A),
      B => Some(EnigoKey::B),
      C => Some(EnigoKey::C),
      D => Some(EnigoKey::D),
      E => Some(EnigoKey::E),
      F => Some(EnigoKey::F),
      G => Some(EnigoKey::G),
      H => Some(EnigoKey::H),
      I => Some(EnigoKey::I),
      J => Some(EnigoKey::J),
      K => Some(EnigoKey::K),
      L => Some(EnigoKey::L),
      M => Some(EnigoKey::M),
      N => Some(EnigoKey::N),
      O => Some(EnigoKey::O),
      P => Some(EnigoKey::P),
      Q => Some(EnigoKey::Q),
      R => Some(EnigoKey::R),
      S => Some(EnigoKey::S),
      T => Some(EnigoKey::T),
      U => Some(EnigoKey::U),
      V => Some(EnigoKey::V),
      W => Some(EnigoKey::W),
      X => Some(EnigoKey::X),
      Y => Some(EnigoKey::Y),
      Z => Some(EnigoKey::Z),
      Backspace => Some(EnigoKey::Backspace),
      CapsLock => Some(EnigoKey::CapsLock),
      Enter => Some(EnigoKey::Return),
      Esc => Some(EnigoKey::Escape),
      AltLeft => Some(EnigoKey::Alt),
      AltRight => Some(EnigoKey::Alt),
      CtrlLeft => Some(EnigoKey::Control),
      CtrlRight => Some(EnigoKey::Control),
      ShiftLeft => Some(EnigoKey::Shift),
      ShiftRight => Some(EnigoKey::Shift),
      Space => Some(EnigoKey::Space),
      Tab => Some(EnigoKey::Tab),
      F1 => Some(EnigoKey::F1),
      F2 => Some(EnigoKey::F2),
      F3 => Some(EnigoKey::F3),
      F4 => Some(EnigoKey::F4),
      F5 => Some(EnigoKey::F5),
      F6 => Some(EnigoKey::F6),
      F7 => Some(EnigoKey::F7),
      F8 => Some(EnigoKey::F8),
      F9 => Some(EnigoKey::F9),
      F10 => Some(EnigoKey::F10),
      F11 => Some(EnigoKey::F11),
      F12 => Some(EnigoKey::F12),
      Digit1 => todo!(),
      Digit2 => todo!(),
      Digit3 => todo!(),
      Digit4 => todo!(),
      Digit5 => todo!(),
      Digit6 => todo!(),
      Digit7 => todo!(),
      Digit8 => todo!(),
      Digit9 => todo!(),
      Digit0 => todo!(),
      Backquote => todo!(),
      Minus => todo!(),
      Equal => todo!(),
      BracketLeft => todo!(),
      BracketRight => todo!(),
      Backslash => todo!(),
      Semicolon => todo!(),
      Quote => todo!(),
      Comma => todo!(),
      Period => todo!(),
      Slash => todo!(),
      MetaLeft => todo!(),
      Delete => todo!(),
      ContextMenu => todo!(),
      Num1 => todo!(),
      Num2 => todo!(),
      Num3 => todo!(),
      Num4 => todo!(),
      Num5 => todo!(),
      Num6 => todo!(),
      Num7 => todo!(),
      Num8 => todo!(),
      Num9 => todo!(),
      Num0 => todo!(),
      NumLock => todo!(),
      NumAdd => todo!(),
      NumSubtract => todo!(),
      NumDivide => todo!(),
      NumMultiply => todo!(),
      NumDecimal => todo!(),
      ArrowUp => todo!(),
      ArrowDown => todo!(),
      ArrowLeft => todo!(),
      ArrowRight => todo!(),
      Home => todo!(),
      End => todo!(),
      Insert => todo!(),
      PageUp => todo!(),
      PageDown => todo!(),
      Pause => todo!(),
      ScrollLock => todo!(),
    }
  }
}

impl TryFrom<KB> for EnigoKey {
  type Error = Error;
  fn try_from(value: KB) -> Result<Self, Self::Error> {
    value.to_enigo_key().to_result(Error::NoKeycode)
  }
}

impl Mouse for M {
  fn to_enigo_button(&self) -> Option<EnigoButton> {
    match self {
      Left => Some(EnigoButton::Left),
      Right => Some(EnigoButton::Right),
      Middle => Some(EnigoButton::Middle),
      Back => Some(EnigoButton::Back),
      Forward => Some(EnigoButton::Forward)
    }
  }
}

impl TryFrom<M> for EnigoButton {
  type Error = Error;
  fn try_from(value: M) -> Result<Self, Self::Error> {
    value.to_enigo_button().to_result(Error::NoKeycode)
  }
}



