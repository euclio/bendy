//! An encoder for bencode. Guarantees that the output string is valid bencode
//!
//! # Encoding a structure
//!
//! The easiest way to encode a structure is to implement [`Encodable`] for it. For most structures,
//! this should be very simple:
//!
//! ```
//! # use bendy::encoder::{Encodable, SingleItemEncoder};
//! # use bendy::Error;
//!
//! struct Message {
//!     foo: i32,
//!     bar: String,
//! }
//!
//! impl Encodable for Message {
//!     // Atoms have depth one. The struct wrapper adds one level to that
//!     const MAX_DEPTH: usize = 1;
//!
//!     fn encode(&self, encoder: SingleItemEncoder) -> Result<(), Error> {
//!         encoder.emit_dict(|mut e| {
//!             // Use e to emit the values
//!             e.emit_pair(b"bar", &self.bar)?;
//!             e.emit_pair(b"foo", &self.foo)
//!         })
//!     }
//! }
//! ```
//!
//! Then, messages can be serialized using [`Encodable::to_bytes`]:
//!
//! ```
//! # use bendy::encoder::{Encodable, SingleItemEncoder};
//! # use bendy::Error;
//! #
//! # struct Message {
//! #    foo: i32,
//! #    bar: String,
//! # }
//! #
//! # impl Encodable for Message {
//! #     // Atoms have depth zero. The struct wrapper adds one level to that
//! #     const MAX_DEPTH: usize = 1;
//! #
//! #     fn encode(&self, encoder: SingleItemEncoder) -> Result<(), Error> {
//! #         encoder.emit_dict(|mut e| {
//! #             // Use e to emit the values. They must be in sorted order here.
//! #             // If sorting the dict first is annoying, you can also use
//! #             // encoder.emit_and_sort_dict
//! #             e.emit_pair(b"bar", &self.bar)?;
//! #             e.emit_pair(b"foo", &self.foo)
//! #         })
//! #     }
//! # }
//! # let result: Result<Vec<u8>, Error> =
//! Message{
//!     foo: 1,
//!     bar: "quux".to_string(),
//! }.to_bytes()
//! # ;
//! # assert!(result.is_ok());
//! ```
//!
//! Most primitive types already implement [`Encodable`].
//!
//! # Nesting depth limits
//!
//! To allow this to be used on limited platforms, all implementations of [`Encodable`] include a
//! maximum nesting depth. Atoms (integers and byte strings) are considered to have depth 0. An
//! object (a list or dict) containing only atoms has depth 1, and in general, an object has a depth
//! equal to the depth of its deepest member plus one. In some cases, an object doesn't have a
//! statically known depth. For example, ASTs may be arbitrarily nested. Such objects should
//! have their depth set to 0, and callers should construct the Encoder manually, adding an
//! appropriate buffer for the depth:
//!
//! ```
//! # use bendy::encoder::{Encodable, Encoder};
//! # use bendy::Error;
//! #
//! # type ObjectType = u32;
//! # static object: u32 = 0;
//! #
//! # fn main() -> Result<(), Error> {
//! let mut encoder = Encoder::new()
//!     .with_max_depth(ObjectType::MAX_DEPTH + 10);
//! encoder.emit(object)?;
//! encoder.get_output()
//! #   .map(|_| ()) // ignore a success return value
//! # }
//! ```
//!
//! # Error handling
//!
//! Once an error occurs during encoding, all future calls to the same encoding stream will fail
//! early with the same error. It is not defined whether any callback or implementation of
//! [`Encodable::encode`] is called before returning an error; such callbacks should respond to
//! failure by bailing out as quickly as possible.
//!
//! Not all values in [`Error`] can be caused by an encoding operation. Specifically, you only need
//! to worry about [`UnsortedKeys`] and [`NestingTooDeep`].
//!
//! [`UnsortedKeys`]: self::Error#UnsortedKeys
//! [`NestingTooDeep`]: self::Error#NestingTooDeep

mod encodable;
mod encoder;
mod printable_integer;

pub use self::{
    encodable::{AsString, Encodable},
    encoder::{Encoder, SingleItemEncoder, SortedDictEncoder, UnsortedDictEncoder},
    printable_integer::PrintableInteger,
};