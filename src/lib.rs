#[cfg(feature = "enigo")]
pub mod enigo_converter;

#[cfg(feature = "rdev")]
pub mod rdev_converter;

use strum_macros::{Display, EnumString};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("A key do not exist")]
  NoKeycode
}

#[macro_export]
macro_rules! decl_keycode {
  ($vis: vis ($to_converter: ident, $from_converter: ident) -> $convert_target: ty [$($from_value: path > $to_value: path)*]) => {
    $vis fn $to_converter(&self) -> Option<$convert_target> {
      match self {
        $(
          $from_value => Some($to_value),
        )*
        _ => None
      }
    }

    $vis fn $from_converter(value: $convert_target) -> Option<Self> {
      match value {
        $(
          #[allow(unreachable_patterns)]
          $to_value => Some($from_value),
        )*
        _ => None
      }
    }
  };
}


#[allow(unused)]
trait OptionToResult<T> {
  fn to_result<E: std::error::Error>(self, err: E) -> Result<T, E>;
}

impl<T> OptionToResult<T> for Option<T> {
  fn to_result<E: std::error::Error>(self, err: E) -> Result<T, E> {
    if let Some(value) = self {
      Ok(value)
    } else {
      Err(err)
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum Keyboard {
  A,
  B,
  C,
  D,
  E,
  F,
  G,
  H,
  I,
  J,
  K,
  L,
  M,
  N,
  O,
  P,
  Q,
  R,
  S,
  T,
  U,
  V,
  W,
  X,
  Y,
  Z,
  Backspace,
  CapsLock,
  Enter,
  Esc,
  AltLeft,
  AltRight,
  CtrlLeft,
  CtrlRight,
  ShiftLeft,
  ShiftRight,
  Space,
  Tab,
  F1,
  F2,
  F3,
  F4,
  F5,
  F6,
  F7,
  F8,
  F9,
  F10,
  F11,
  F12,
  Digit1,
  Digit2,
  Digit3,
  Digit4,
  Digit5,
  Digit6,
  Digit7,
  Digit8,
  Digit9,
  Digit0,
  Backquote,
  Minus,
  Equal,
  BracketLeft,
  BracketRight,
  Backslash,
  Semicolon,
  Quote,
  Comma,
  Period,
  Slash,
  MetaLeft,
  Delete,
  ContextMenu,
  Num1,
  Num2,
  Num3,
  Num4,
  Num5,
  Num6,
  Num7,
  Num8,
  Num9,
  Num0,
  NumLock,
  NumAdd,
  NumSubtract,
  NumDivide,
  NumMultiply,
  NumDecimal,
  ArrowUp,
  ArrowDown,
  ArrowLeft,
  ArrowRight,
  Home,
  End,
  Insert,
  PageUp,
  PageDown,
  Pause,
  ScrollLock,
}

// A => todo!(),
// B => todo!(),
// C => todo!(),
// D => todo!(),
// E => todo!(),
// F => todo!(),
// G => todo!(),
// H => todo!(),
// I => todo!(),
// J => todo!(),
// K => todo!(),
// L => todo!(),
// M => todo!(),
// N => todo!(),
// O => todo!(),
// P => todo!(),
// Q => todo!(),
// R => todo!(),
// S => todo!(),
// T => todo!(),
// U => todo!(),
// V => todo!(),
// W => todo!(),
// X => todo!(),
// Y => todo!(),
// Z => todo!(),
// Backspace => todo!(),
// CapsLock => todo!(),
// Enter => todo!(),
// Esc => todo!(),
// AltLeft => todo!(),
// AltRight => todo!(),
// CtrlLeft => todo!(),
// CtrlRight => todo!(),
// ShiftLeft => todo!(),
// ShiftRight => todo!(),
// Space => todo!(),
// Tab => todo!(),
// F1 => todo!(),
// F2 => todo!(),
// F3 => todo!(),
// F4 => todo!(),
// F5 => todo!(),
// F6 => todo!(),
// F7 => todo!(),
// F8 => todo!(),
// F9 => todo!(),
// F10 => todo!(),
// F11 => todo!(),
// F12 => todo!(),
// Digit1 => todo!(),
// Digit2 => todo!(),
// Digit3 => todo!(),
// Digit4 => todo!(),
// Digit5 => todo!(),
// Digit6 => todo!(),
// Digit7 => todo!(),
// Digit8 => todo!(),
// Digit9 => todo!(),
// Digit0 => todo!(),
// Backquote => todo!(),
// Minus => todo!(),
// Equal => todo!(),
// BracketLeft => todo!(),
// BracketRight => todo!(),
// Backslash => todo!(),
// Semicolon => todo!(),
// Quote => todo!(),
// Comma => todo!(),
// Period => todo!(),
// Slash => todo!(),
// MetaLeft => todo!(),
// Delete => todo!(),
// ContextMenu => todo!(),
// Num1 => todo!(),
// Num2 => todo!(),
// Num3 => todo!(),
// Num4 => todo!(),
// Num5 => todo!(),
// Num6 => todo!(),
// Num7 => todo!(),
// Num8 => todo!(),
// Num9 => todo!(),
// Num0 => todo!(),
// NumLock => todo!(),
// NumAdd => todo!(),
// NumSubtract => todo!(),
// NumDivide => todo!(),
// NumMultiply => todo!(),
// NumDecimal => todo!(),
// ArrowUp => todo!(),
// ArrowDown => todo!(),
// ArrowLeft => todo!(),
// ArrowRight => todo!(),
// Home => todo!(),
// End => todo!(),
// Insert => todo!(),
// PageUp => todo!(),
// PageDown => todo!(),
// Pause => todo!(),
// ScrollLock => todo!(),

impl Keyboard {
  pub fn to_ascii(&self) -> u8 {
    use Keyboard::*;
    match self {
      A => 65,
      B => 66,
      C => 67,
      D => 68,
      E => 69,
      F => 70,
      G => 71,
      H => 72,
      I => 73,
      J => 74,
      K => 75,
      L => 76,
      M => 77,
      N => 78,
      O => 79,
      P => 80,
      Q => 81,
      R => 82,
      S => 83,
      T => 84,
      U => 85,
      V => 86,
      W => 87,
      X => 88,
      Y => 89,
      Z => 90,
      Backspace => 8,
      CapsLock => 20,
      Enter => 13,
      Esc => 27,
      AltLeft => 18,
      AltRight => 18,
      CtrlLeft => 17,
      CtrlRight => 17,
      ShiftLeft => 16,
      ShiftRight => 16,
      Space => 32,
      Tab => 9,
      F1 => 112,
      F2 => 113,
      F3 => 114,
      F4 => 115,
      F5 => 116,
      F6 => 117,
      F7 => 118,
      F8 => 119,
      F9 => 120,
      F10 => 121,
      F11 => 122,
      F12 => 123,
      Digit1 => 49,
      Digit2 => 50,
      Digit3 => 51,
      Digit4 => 52,
      Digit5 => 53,
      Digit6 => 54,
      Digit7 => 55,
      Digit8 => 56,
      Digit9 => 57,
      Digit0 => 48,
      Backquote => 8,
      Minus => 189,
      Equal => 187,
      BracketLeft => 219,
      BracketRight => 221,
      Backslash => 220,
      Semicolon => 186,
      Quote => 222,
      Comma => 188,
      Period => 190,
      Slash => 191,
      MetaLeft => 91,
      Delete => 46,
      ContextMenu => 93,
      Num1 => 97,
      Num2 => 98,
      Num3 => 99,
      Num4 => 100,
      Num5 => 101,
      Num6 => 102,
      Num7 => 103,
      Num8 => 104,
      Num9 => 105,
      Num0 => 96,
      NumLock => 144,
      NumAdd => 107,
      NumSubtract => 109,
      NumDivide => 111,
      NumMultiply => 106,
      NumDecimal => 46,
      ArrowUp => 38,
      ArrowDown => 40,
      ArrowLeft => 37,
      ArrowRight => 39,
      Home => 36,
      End => 35,
      Insert => 45,
      PageUp => 33,
      PageDown => 34,
      Pause => 19,
      ScrollLock => 145,
    }
  }



}

impl From<Keyboard> for u8 {
  fn from(value: Keyboard) -> Self {
    value.to_ascii()
  }
}

impl From<Keyboard> for String {
  fn from(value: Keyboard) -> Self {
    value.to_string()
  }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Mouse {
  Left,
  Right,
  Middle,
  Forward,
  Back
}




#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Key {
  Keyboard(Keyboard),
  Mouse(Mouse)
}

