// use crate::error::Result;
// use std::io::Seek;

// pub trait Read<'de>: private::Sealed {
//     fn read<'a>(&'a mut self, n: usize) -> Result<EitherLifetime<'a, 'de>> {
//         self.clear_buffer();
//         self.read_to_buffer(n)?;

//         Ok(self.take_buffer())
//     }

//     #[doc(hidden)]
//     fn next(&mut self) -> Result<Option<u8>>;

//     #[doc(hidden)]
//     fn peek(&mut self) -> Result<Option<u8>>;

//     #[doc(hidden)]
//     fn clear_buffer(&mut self);

//     #[doc(hidden)]
//     fn read_to_buffer(&mut self, n: usize) -> Result<()>;

//     #[doc(hidden)]
//     fn take_buffer<'a>(&'a mut self) -> EitherLifetime<'a, 'de>;

//     #[doc(hidden)]
//     fn read_into(&mut self, buf: &mut [u8]) -> Result<()>;

//     #[doc(hidden)]
//     fn discard(&mut self);

//     #[doc(hidden)]
//     fn offset(&self) -> u64;
// }