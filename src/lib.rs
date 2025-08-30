#![no_std]

/// Matches an expression to any of the patterns and executes the same expression arm for any match.
///
/// This macro allows you to use the same expression arm for different types
/// by creating the same match arm for each of the patterns separated by `|`.
/// A standard match statement only allows such patterns for the same type:
///
/// ```compile_fail
/// let result: Result<i64, i32> = Err(42);
/// let int: i64 = match result { Ok(i) | Err(i) => i.into() }; // does not compile!
/// ```
///
/// ```
/// # use match_any::match_any;
/// let result: Result<i64, i32> = Err(42);
/// let int: i64 = match_any!(result, Ok(i) | Err(i) => i.into()); // compiles just fine
/// assert_eq!(int, 42);
/// ```
///
/// # Examples
///
/// ```
/// use match_any::match_any;
///
/// enum Id { U8(u8), I16(i16), I32(i32) }
/// use Id::*;
///
/// let id = Id::I16(-2);
/// let id: i32 = match_any!(id, U8(x) | I16(x) | I32(x) => x.into());
/// assert_eq!(id, -2);
/// ```
///
/// With multiple match arms:
///
/// ```
/// use core::convert::TryFrom;
/// use match_any::match_any;
///
/// enum Id { U8(u8), I16(i16), I32(i32), U64(u64) }
/// use Id::*;
///
/// let id = Id::I32(-3);
/// let id: i32 = match_any!(id,
///     U8(a) | I16(a) | I32(a) => a.into(),
///     U64(b) => i32::try_from(b).unwrap_or(0)
/// );
/// assert_eq!(id, -3);
/// ```
///
/// # Macro Expansion
///
/// ```
/// # use match_any::match_any;
/// let result: Result<i32, i32> = Err(42);
/// match_any!(result, Ok(i) | Err(i) => Some(i));
/// ```
/// expands to
/// ```
/// let result: Result<i32, i32> = Err(42);
/// match result { Ok(i) => Some(i), Err(i) => Some(i) };
/// ```
///
/// # Conditional Compilation Support
///
/// The macro supports `#[cfg(...)]` attributes for conditional compilation:
///
/// ```
/// use match_any::match_any;
///
/// enum MediaType { Video(String), Audio(String), Image(String) }
/// use MediaType::*;
///
/// let media = MediaType::Video("movie.mp4".to_string());
/// let result = match_any!(media,
///     #[cfg(feature = "video-player")]
///     Video(name) => format!("Playing video: {}", name),
///     #[cfg(feature = "audio-player")]
///     Audio(name) => format!("Playing audio: {}", name),
///     Image(name) => format!("Displaying image: {}", name),
///     _ => "Unsupported media type".to_string()
/// );
/// # // 对于测试，我们假设没有启用任何 feature
/// # assert_eq!(result, "Unsupported media type");
/// ```
///
/// # Enum Dispatch
///
/// Similarly to the [enum_dispatch crate](https://crates.io/crates/enum_dispatch),
/// this macro can be used to implement "enum dispatch" as an alternative to dynamic dispatch.
/// The major difference between the enum_dispatch crate and this macro is,
/// that enum_dispatch provides a _procedural_ macro, while this is a _declarative_ macro.
/// This allows enum_dispatch to reduce the boilerplate code a lot more than match_any.
/// However IDE support should be a bit better with match_any.
///
/// ## Enum Dispatch Example
///
/// ```
/// use match_any::match_any;
///
/// trait IntId {
///     fn int_id(&self) -> i32;
/// }
///
/// impl IntId for u64 {
///     fn int_id(&self) -> i32 { 64 }
/// }
///
/// impl IntId for u32 {
///     fn int_id(&self) -> i32 { 32 }
/// }
///
/// enum IntIdKind { U64(u64), U32(u32) }
///
/// impl IntId for IntIdKind {
///     fn int_id(&self) -> i32 {
///         use IntIdKind::*;
///         match_any!(self, U64(i) | U32(i) => i.int_id())
///     }
/// }
///
/// let int_id_kind = IntIdKind::U32(0);
/// assert_eq!(int_id_kind.int_id(), 32); // enum dispatch
/// let int_id_box: Box<dyn IntId> = Box::new(0_u32);
/// assert_eq!(int_id_box.int_id(), 32); // dynamic dispatch
/// ```
#[macro_export]
macro_rules! match_any {
    // 递归处理规则：处理 #[cfg] 分支（保留属性）
    (@impl $expr:expr; [$($arms:tt)*]; #[cfg($($cfg_meta:tt)*)] $($cfg_pat:pat)|+ => $cfg_expr:expr, $($rest:tt)*) => {
        match_any!(@impl $expr; [$($arms)* #[cfg($($cfg_meta)*)] $($cfg_pat => $cfg_expr,)+]; $($rest)*)
    };

    // 递归处理规则：处理普通分支（添加到结果中）
    (@impl $expr:expr; [$($arms:tt)*]; $($pat:pat)|+ => $arm_expr:expr, $($rest:tt)*) => {
        match_any!(@impl $expr; [$($arms)* $($pat => $arm_expr,)+]; $($rest)*)
    };

    // 递归处理规则：处理最后一个 #[cfg] 分支
    (@impl $expr:expr; [$($arms:tt)*]; #[cfg($($cfg_meta:tt)*)] $($cfg_pat:pat)|+ => $cfg_expr:expr) => {
        match $expr {
            $($arms)*
            #[cfg($($cfg_meta)*)]
            $($cfg_pat => $cfg_expr,)+
        }
    };

    // 递归处理规则：处理最后一个普通分支
    (@impl $expr:expr; [$($arms:tt)*]; $($pat:pat)|+ => $arm_expr:expr) => {
        match $expr {
            $($arms)* $($pat => $arm_expr,)+
        }
    };

    // 递归处理规则：所有分支都已处理完
    (@impl $expr:expr; [$($arms:tt)*];) => {
        match $expr {
            $($arms)*
        }
    };

    // 公共入口点
    ($expr:expr, $($arms:tt)*) => {
        match_any!(@impl $expr; []; $($arms)*)
    };
}
