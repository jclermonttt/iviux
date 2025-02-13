/// Macros for write typescript in rust file.
///
///  This macros take typescript syntax for generating in wasm.
///
/// ```
///  ts! {
///    let hello: string = 'Hello, world!';
///    console.log(hello);
///  }
/// ```
#[macro_export]
macro_rules! ts {
    ($($ts:tt)*) => {};
}
