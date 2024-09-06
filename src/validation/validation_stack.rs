//! This module contains the [`ValidationStack`] data structure
//!
//! The [`ValidationStack`] is a unified stack, in the sense that it unifies both
//! [`ValidationStackEntry::Val`] and [`ValidationStackEntry::Label`]. It therefore mixes type
//! information with structured control flow information.
#![allow(unused)] // TODO remove this once sidetable implementation lands
use super::Result;
use alloc::vec::Vec;

use crate::{Error, ValType};

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ValidationStack {
    stack: Vec<ValidationStackEntry>,
}

impl ValidationStack {
    /// Initialize a new ValidationStack
    pub(super) fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub(super) fn len(&self) -> usize {
        self.stack.len()
    }

    pub(super) fn push_valtype(&mut self, valtype: ValType) {
        self.stack.push(ValidationStackEntry::Val(valtype));
    }

    pub(super) fn push_label(&mut self, label_info: LabelInfo) {
        self.stack.push(ValidationStackEntry::Label(label_info));
    }

    /// This puts an unspecified element on top of the stack.
    /// While the top of the stack is unspecified, arbitrary value types can be popped.
    /// To remove this, a new label has to be pushed or an existing one has to be popped.
    pub(super) fn make_unspecified(&mut self) {
        // Pop everything until next label or until the stack is empty.
        // This is okay, because these values cannot be accessed during execution ever again.
        while let Some(entry) = self.stack.last() {
            match entry {
                ValidationStackEntry::Val(_) | ValidationStackEntry::UnspecifiedValTypes => {
                    self.stack.pop();
                }
                ValidationStackEntry::Label(_) => break,
            }
        }

        self.stack.push(ValidationStackEntry::UnspecifiedValTypes)
    }

    /// Pop a [`ValidationStackEntry`] from the [`ValidationStack`]
    ///
    /// # Returns
    ///
    /// - Returns `Ok(_)` with the former top-most [`ValidationStackEntry`] inside, if the stack had
    ///   at least one element.
    /// - Returns `Err(_)` if the stack was already empty.
    fn pop(&mut self) -> Result<ValidationStackEntry> {
        self.stack
            .pop()
            .ok_or(Error::InvalidValidationStackValType(None))
    }

    /// Assert the top-most [`ValidationStackEntry`] is a specific [`ValType`], after popping it from the [`ValidationStack`]
    ///
    /// # Returns
    ///
    /// - Returns `Ok(())` if the top-most [`ValidationStackEntry`] is a [`ValType`] identical to
    ///   `expected_ty`.
    /// - Returns `Err(_)` otherwise.
    pub(super) fn assert_pop_val_type(&mut self, expected_ty: ValType) -> Result<()> {
        if let Some(ValidationStackEntry::UnspecifiedValTypes) = self.stack.last() {
            // An unspecified value is always correct, and will never disappear by popping.
            return Ok(());
        }

        match self.pop()? {
            ValidationStackEntry::Val(ty) => (ty == expected_ty)
                .then_some(())
                .ok_or(Error::InvalidValidationStackValType(Some(ty))),
            ValidationStackEntry::Label(li) => Err(Error::FoundLabel(li.kind)),
            ValidationStackEntry::UnspecifiedValTypes => {
                unreachable!("we just checked if the topmost entry is of this type")
            }
        }
    }

    /// Asserts that the values on top of the stack match those of a value iterator
    ///
    /// The last element of the `val_types` [`Iterator`] is compared to the top-most
    /// [`ValidationStackEntry`], the second last `val_types` element to the second top-most
    /// [`ValidationStackEntry`] etc.
    ///
    /// Any occurence of the [`ValidationStackEntry::Label`] variant in the stack tail will cause an
    /// error. This method does not mutate the [`ValidationStack::stack`] in any way.
    ///
    /// # Returns
    ///
    /// - `Ok(_)`, the tail of the stack matches the `expected_val_types`
    /// - `Err(_)` otherwise
    pub(super) fn assert_val_types_on_top(&self, expected_val_types: &[ValType]) -> Result<()> {
        let stack_tail = self
            .stack
            .get(self.stack.len() - expected_val_types.len()..)
            .ok_or(Error::InvalidValType)?;

        // Now we check the valtypes in reverse.
        // That way we can stop checking if we encounter an `UnspecifiedValTypes`.

        let mut current_expected_valtype = expected_val_types.iter().rev();
        for entry in stack_tail.iter().rev() {
            match entry {
                ValidationStackEntry::Label(label) => return Err(Error::EndInvalidValueStack),
                ValidationStackEntry::Val(valtype) => {
                    if Some(valtype) != current_expected_valtype.next() {
                        return Err(Error::EndInvalidValueStack);
                    }
                }
                ValidationStackEntry::UnspecifiedValTypes => {
                    return Ok(());
                }
            }
        }

        Ok(())
    }

    pub(super) fn assert_val_types(&self, expected_val_types: &[ValType]) -> Result<()> {
        let topmost_label_index = self.find_topmost_label_idx();

        let first_valtype = topmost_label_index.map(|idx| idx + 1).unwrap_or(0);

        // Now we check the valtypes in reverse.
        // That way we can stop checking if we encounter an `UnspecifiedValTypes`.

        let mut current_expected_valtype = expected_val_types.iter().rev();
        for entry in self.stack[first_valtype..].iter().rev() {
            match entry {
                ValidationStackEntry::Label(_) => unreachable!(
                    "we started at the top-most label so we cannot find any more labels"
                ),
                ValidationStackEntry::Val(valtype) => {
                    if Some(valtype) != current_expected_valtype.next() {
                        return Err(Error::EndInvalidValueStack);
                    }
                }
                ValidationStackEntry::UnspecifiedValTypes => {
                    return Ok(());
                }
            }
        }

        Ok(())
    }

    /// Finds the index of the topmost label that is currently on the stack.
    fn find_topmost_label_idx(&self) -> Option<usize> {
        self.stack
            .iter()
            .enumerate()
            .rev()
            .find(|(_idx, entry)| matches!(entry, ValidationStackEntry::Label(_)))
            .map(|(idx, _entry)| idx)
    }

    /// Removes all valtypes until the next lower label.
    /// If a label is found and popped, its info is returned.
    fn pop_label_and_above(&mut self) -> Option<LabelInfo> {
        /// Delete all the values until the topmost label or until the stack is empty
        match self.find_topmost_label_idx() {
            Some(idx) => {
                if self.stack.len() > idx + 1 {
                    self.stack.drain((idx + 1)..);
                }
            }
            None => self.stack.clear(),
        }

        // Pop the label itself
        match self.pop() {
            Ok(ValidationStackEntry::Label(info)) => Some(info),
            Ok(_) => unreachable!(
                "we just removed everything until the next label, thus new topmost entry must be a label"
            ),
            Err(_) => None,
        }
    }

    /// Return true if the stack has at least one remaining label
    pub(super) fn has_remaining_label(&self) -> bool {
        self.stack
            .iter()
            .any(|e| matches!(e, ValidationStackEntry::Label(_)))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ValidationStackEntry {
    /// A value
    Val(ValType),

    /// A label
    Label(LabelInfo),

    /// Special variant to encode that any possible number of ValTypes could be here
    ///
    /// Caused by `return` and `unreachable, as both can push an arbitrary number of values
    /// from/to the stack.
    ///
    /// When this variant is popped onto the stack, all valtypes until the next lower label are deleted.
    /// They are not needed anymore because this variant can expand to all of them.
    UnspecifiedValTypes,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LabelInfo {
    pub(crate) kind: LabelKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LabelKind {
    Block,
    Loop,
    If,
}