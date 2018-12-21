//! Allows access to the keypad.

use super::*;

/// The Key Input Register.
///
/// This register follows the "low-active" convention. If you want your code to
/// follow the "high-active" convention (hint: you probably do, it's far easier
/// to work with) then call `read_key_input()` rather than reading this register
/// directly. It will perform the necessary bit flip operation for you.
pub const KEYINPUT: VolAddress<u16> = unsafe { VolAddress::new_unchecked(0x400_0130) };

/// A "tribool" value helps us interpret the arrow pad.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
#[allow(missing_docs)]
pub enum TriBool {
  Minus = -1,
  Neutral = 0,
  Plus = 1,
}

newtype! {
  /// Records a particular key press combination.
  ///
  /// Methods here follow the "high-active" convention, where a bit is enabled
  /// when it's part of the set.
  #[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
  KeyInput, u16
}

#[allow(missing_docs)]
impl KeyInput {
  register_bit!(A_BIT, u16, 1, a_pressed);
  register_bit!(B_BIT, u16, 1 << 1, b_pressed);
  register_bit!(SELECT_BIT, u16, 1 << 2, select_pressed);
  register_bit!(START_BIT, u16, 1 << 3, start_pressed);
  register_bit!(RIGHT_BIT, u16, 1 << 4, right_pressed);
  register_bit!(LEFT_BIT, u16, 1 << 5, left_pressed);
  register_bit!(UP_BIT, u16, 1 << 6, up_pressed);
  register_bit!(DOWN_BIT, u16, 1 << 7, down_pressed);
  register_bit!(R_BIT, u16, 1 << 8, r_pressed);
  register_bit!(L_BIT, u16, 1 << 9, l_pressed);

  /// Takes the set difference between these keys and another set of keys.
  pub fn difference(self, other: Self) -> Self {
    KeyInput(self.0 ^ other.0)
  }

  /// Gives the arrow pad value as a tribool, with Plus being increased column
  /// value (right).
  pub fn column_direction(self) -> TriBool {
    if self.right_pressed() {
      TriBool::Plus
    } else if self.left_pressed() {
      TriBool::Minus
    } else {
      TriBool::Neutral
    }
  }

  /// Gives the arrow pad value as a tribool, with Plus being increased row
  /// value (down).
  pub fn row_direction(self) -> TriBool {
    if self.down_pressed() {
      TriBool::Plus
    } else if self.up_pressed() {
      TriBool::Minus
    } else {
      TriBool::Neutral
    }
  }
}

/// Gets the current state of the keys
pub fn read_key_input() -> KeyInput {
  // Note(Lokathor): The 10 used bits are "low when pressed" style, but the 6
  // unused bits are always low, so we XOR with this mask to get a result where
  // the only active bits are currently pressed keys.
  KeyInput(KEYINPUT.read() ^ 0b0000_0011_1111_1111)
}

newtype! {
  /// Allows configuration of when a keypad interrupt fires.
  ///
  /// * The most important bit here is the `irq_enabled` bit, which determines
  ///   if an interrupt happens at all.
  /// * The second most important bit is the `irq_logical_and` bit. If this bit
  ///   is set, _all_ the selected buttons are required to be set for the
  ///   interrupt to be fired (logical AND). If it's not set then _any_ of the
  ///   buttons selected can be pressed to fire the interrupt (logical OR).
  /// * All other bits select a particular button to be required or not as part
  ///   of the interrupt firing.
  ///
  /// NOTE: This _only_ configures the operation of when keypad interrupts can
  /// fire. You must still set the `IME` to have interrupts at all, and you must
  /// further set `IE` for keypad interrupts to be possible.
  #[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
  KeyInterruptSetting, u16
}
#[allow(missing_docs)]
impl KeyInterruptSetting {
  register_bit!(A_BIT, u16, 1, a_pressed);
  register_bit!(B_BIT, u16, 1 << 1, b_pressed);
  register_bit!(SELECT_BIT, u16, 1 << 2, select_pressed);
  register_bit!(START_BIT, u16, 1 << 3, start_pressed);
  register_bit!(RIGHT_BIT, u16, 1 << 4, right_pressed);
  register_bit!(LEFT_BIT, u16, 1 << 5, left_pressed);
  register_bit!(UP_BIT, u16, 1 << 6, up_pressed);
  register_bit!(DOWN_BIT, u16, 1 << 7, down_pressed);
  register_bit!(R_BIT, u16, 1 << 8, r_pressed);
  register_bit!(L_BIT, u16, 1 << 9, l_pressed);
  //
  register_bit!(IRQ_ENABLE_BIT, u16, 1 << 14, irq_enabled);
  register_bit!(IRQ_AND_BIT, u16, 1 << 15, irq_logical_and);
}

/// Use this to configure when a keypad interrupt happens.
///
/// See the `KeyInterruptSetting` type for more.
pub const KEYCNT: VolAddress<KeyInterruptSetting> = unsafe { VolAddress::new_unchecked(0x400_0132) };
