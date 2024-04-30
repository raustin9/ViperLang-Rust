use core::ops::{Add, Sub};

pub struct CodeSpan {
    pub low: BytePosition,
    pub high: BytePosition,
}


/* Thanks Leo: https://github.com/AleoHQ/leo */

/// Position trait for locations in the source code
pub trait Position {
    fn from_size(n: usize) -> Self;
    fn to_usize(&self) -> usize;
    fn from_u32(n: u32) -> Self;
    fn to_u32(&self) -> u32;
}

/// Macro expansion for implementing the Position trait
macro_rules! impl_position {
    (
        $(
            $(#[$attr:meta])*
            $vis:vis struct $ident:ident($inner_vis:vis $inner_ty:ty);
        )*
    ) => {
        $(
            $(#[$attr])*
            $vis struct $ident($inner_vis $inner_ty);

            impl Position for $ident {
                #[inline(always)]
                fn from_size(n: usize) -> $ident {
                    return $ident(n as $inner_ty);
                }
                
                #[inline(always)]
                fn to_usize(&self) -> usize {
                    return self.0 as usize;
                }
                
                #[inline(always)]
                fn from_u32(n: u32) -> $ident {
                    return $ident(n as $inner_ty);
                }
                
                #[inline(always)]
                fn to_u32(&self) -> u32 {
                    return self.0 as u32;
                }
            }

            impl Add for $ident {
                type Output = $ident;
                
                #[inline(always)]
                fn add(self, rhs: $ident) -> $ident {
                    return $ident(self.0 + rhs.0);
                }
            }

            impl Sub for $ident {
                type Output = $ident;
                
                #[inline(always)]
                fn sub(self, rhs: $ident) -> $ident {
                    return $ident(self.0 - rhs.0);
                }
            }
        )*
    };
}


impl_position! {
    #[derive(Clone)]
    pub struct BytePosition(pub u32);

    #[derive(Clone)]
    pub struct CharacterPosition(pub usize);
}
