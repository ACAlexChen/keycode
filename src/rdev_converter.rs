use super::{Keyboard::{self as KB, *}, Mouse::{self as M, *}, Error, OptionToResult as _, decl_keycode};

use rdev::Key as RdevKey;
use rdev::Button as RdevButton;

pub trait Keyboard {
  fn to_rdev_key(&self) -> Option<RdevKey>;
  fn from_rdev_key(value: RdevKey) -> Option<KB>;
}

impl Keyboard for KB {
  decl_keycode!(
    (to_rdev_key, from_rdev_key) -> RdevKey [
      A > RdevKey::KeyA
      B > RdevKey::KeyB
      C > RdevKey::KeyC
      D > RdevKey::KeyD
      E > RdevKey::KeyE
      F > RdevKey::KeyF
      G > RdevKey::KeyG
      H > RdevKey::KeyH
      I > RdevKey::KeyI
      J > RdevKey::KeyJ
      K > RdevKey::KeyK
      L > RdevKey::KeyL
      M > RdevKey::KeyM
      N > RdevKey::KeyN
      O > RdevKey::KeyO
      P > RdevKey::KeyP
      Q > RdevKey::KeyQ
      R > RdevKey::KeyR
      S > RdevKey::KeyS
      T > RdevKey::KeyT
      U > RdevKey::KeyU
      V > RdevKey::KeyV
      W > RdevKey::KeyW
      X > RdevKey::KeyX
      Y > RdevKey::KeyY
      Z > RdevKey::KeyZ
      Backspace > RdevKey::Backspace
      CapsLock > RdevKey::CapsLock
      Enter > RdevKey::Return
      Esc > RdevKey::Escape
      AltLeft > RdevKey::Alt
      AltRight > RdevKey::Alt
      CtrlLeft > RdevKey::ControlLeft
      CtrlRight > RdevKey::ControlRight
      ShiftLeft > RdevKey::ShiftLeft
      ShiftRight > RdevKey::ShiftRight
      Space > RdevKey::Space
      Tab > RdevKey::Tab
      F1 > RdevKey::F1
      F2 > RdevKey::F2
      F3 > RdevKey::F3
      F4 > RdevKey::F4
      F5 > RdevKey::F5
      F6 > RdevKey::F6
      F7 > RdevKey::F7
      F8 > RdevKey::F8
      F9 > RdevKey::F9
      F10 > RdevKey::F10
      F11 > RdevKey::F11
      F12 > RdevKey::F12
      Digit1 > RdevKey::Num1
      Digit2 > RdevKey::Num2
      Digit3 > RdevKey::Num3
      Digit4 > RdevKey::Num4
      Digit5 > RdevKey::Num5
      Digit6 > RdevKey::Num6
      Digit7 > RdevKey::Num7
      Digit8 > RdevKey::Num8
      Digit9 > RdevKey::Num9
      Digit0 > RdevKey::Num0
      Backquote > RdevKey::BackQuote
      Minus > RdevKey::Minus
      Equal > RdevKey::Equal
      BracketLeft > RdevKey::LeftBracket
      BracketRight > RdevKey::RightBracket
      Backslash > RdevKey::BackSlash
      Semicolon > RdevKey::SemiColon
      Quote > RdevKey::Quote
      Comma > RdevKey::Comma
      Slash > RdevKey::Slash
      MetaLeft > RdevKey::MetaLeft
      Delete > RdevKey::Delete
      Num1 > RdevKey::Kp1
      Num2 > RdevKey::Kp2
      Num3 > RdevKey::Kp3
      Num4 > RdevKey::Kp4
      Num5 > RdevKey::Kp5
      Num6 > RdevKey::Kp6
      Num7 > RdevKey::Kp7
      Num8 > RdevKey::Kp8
      Num9 > RdevKey::Kp9
      Num0 > RdevKey::Kp0
      NumLock > RdevKey::NumLock
      NumSubtract > RdevKey::KpMinus
      NumDivide > RdevKey::KpDivide
      NumMultiply > RdevKey::KpMultiply
      ArrowUp > RdevKey::UpArrow
      ArrowDown > RdevKey::DownArrow
      ArrowLeft > RdevKey::LeftArrow
      ArrowRight > RdevKey::RightArrow
      Home > RdevKey::Home
      End > RdevKey::End
      Insert > RdevKey::Insert
      PageUp > RdevKey::PageUp
      PageDown > RdevKey::PageDown
      Pause > RdevKey::Pause
      ScrollLock > RdevKey::ScrollLock
    ]
  );
}

impl TryFrom<KB> for RdevKey {
  type Error = Error;
  fn try_from(value: KB) -> Result<Self, Self::Error> {
    value.to_rdev_key().to_result(Error::NoKeycode)
  }
}

pub trait Mouse {
  fn to_rdev_button(&self) -> Option<RdevButton>;
  fn from_rdev_button(value: RdevButton) -> Option<M>;
}

impl Mouse for M {
  decl_keycode!(
    (to_rdev_button, from_rdev_button) -> RdevButton [
      Left > RdevButton::Left
      Right > RdevButton::Right
      Middle > RdevButton::Middle
    ]
  );
}

impl TryFrom<M> for RdevButton {
  type Error = Error;
  fn try_from(value: M) -> Result<Self, Self::Error> {
    value.to_rdev_button().to_result(Error::NoKeycode)
  }
}

