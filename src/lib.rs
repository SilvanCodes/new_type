#[macro_export]
macro_rules! newtype {
    ( $( $newtype:ident ),* ) => {
        $(
            #[derive(Debug)]
            pub struct $newtype<T>(pub T);

            impl<T> std::ops::Deref for $newtype<T> {
                type Target = T;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl<T> std::ops::DerefMut for $newtype<T> {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            impl<T: std::ops::Add<Output = T>> std::ops::Add for $newtype<T> {
                type Output = Self;

                fn add(self, other: Self) -> Self {
                    Self(self.0 + other.0)
                }
            }

            impl<T: std::ops::AddAssign> std::ops::AddAssign for $newtype<T> {
                fn add_assign(&mut self, other: Self) {
                    self.0 += other.0;
                }
            }

            impl<T: std::clone::Clone> std::clone::Clone for $newtype<T> {
                fn clone(&self) -> Self {
                    Self(self.0.clone())
                }
            }

            impl<T: std::marker::Copy> std::marker::Copy for $newtype<T> {}

            impl<T: std::cmp::PartialEq> std::cmp::PartialEq for $newtype<T> {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }
            }

            impl<T: std::cmp::Eq> std::cmp::Eq for $newtype<T> {}

            impl<T: std::ops::BitAnd<Output = T>> std::ops::BitAnd for $newtype<T> {
                type Output = Self;
                fn bitand(self, rhs: Self) -> Self::Output {
                    Self(self.0 & rhs.0)
                }
            }

            // TRADEOFF: requires Copy
            impl<T: std::ops::BitAndAssign + std::ops::BitAnd<Output = T> + Copy> std::ops::BitAndAssign for $newtype<T> {
                fn bitand_assign(&mut self, rhs: Self) {
                    *self = *self & rhs
                }
            }

            impl<T: std::ops::BitOr<Output = T>> std::ops::BitOr for $newtype<T> {
                type Output = Self;

                fn bitor(self, rhs: Self) -> Self {
                    Self(self.0 | rhs.0)
                }
            }

            // TRADEOFF: requires Copy
            impl<T: std::ops::BitOrAssign + std::ops::BitOr<Output = T> + std::marker::Copy> std::ops::BitOrAssign for $newtype<T> {
                fn bitor_assign(&mut self, rhs: Self) {
                    *self = *self | rhs
                }
            }

            impl<T: std::ops::BitXor<Output = T>> std::ops::BitXor for $newtype<T> {
                type Output = Self;

                fn bitxor(self, rhs: Self) -> Self::Output {
                    Self(self.0 ^ rhs.0)
                }
            }

            // TRADEOFF: requires Copy
            impl<T: std::ops::BitXorAssign + std::ops::BitXor<Output = T> + std::marker::Copy> std::ops::BitXorAssign for $newtype<T> {
                fn bitxor_assign(&mut self, rhs: Self) {
                    *self = *self ^ rhs
                }
            }

            impl<T: std::ops::Div<Output = T>> std::ops::Div for $newtype<T> {
                type Output = Self;

                fn div(self, rhs: Self) -> Self::Output {
                    Self(self.0 / rhs.0)
                }
            }

            // TRADEOFF: requires Copy
            impl<T: std::ops::DivAssign + std::ops::Div<Output = T> + std::marker::Copy> std::ops::DivAssign for $newtype<T> {
                fn div_assign(&mut self, rhs: Self) {
                    *self = *self / rhs
                }
            }

            impl<T: std::ops::Mul<Output = T>> std::ops::Mul for $newtype<T> {
                type Output = Self;

                fn mul(self, rhs: Self) -> Self {
                    Self(self.0 * rhs.0)
                }
            }

            // TRADEOFF: requires Copy
            impl<T: std::ops::MulAssign + std::ops::Mul<Output = T> + std::marker::Copy> std::ops::MulAssign for $newtype<T> {
                fn mul_assign(&mut self, rhs: Self) {
                    *self = *self * rhs
                }
            }

            impl<T: std::ops::Not<Output = T>> std::ops::Not for $newtype<T> {
                type Output = Self;

                fn not(self) -> Self::Output {
                    Self(!self.0)
                }
            }

            impl<T: std::cmp::Ord> std::cmp::Ord for $newtype<T> {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    self.0.cmp(&other.0)
                }
            }

            impl<T: std::cmp::PartialOrd> std::cmp::PartialOrd for $newtype<T> {
                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    self.0.partial_cmp(&other.0)
                }
            }

            impl<T: std::ops::Rem<Output = T>> std::ops::Rem for $newtype<T> {
                type Output = Self;

                fn rem(self, modulus: Self) -> Self::Output {
                    Self(self.0 % modulus.0)
                }
            }

            // TRADEOFF: requires Copy
            impl<T: std::ops::RemAssign + std::ops::Rem<Output = T> + std::marker::Copy> std::ops::RemAssign for $newtype<T> {
                fn rem_assign(&mut self, modulus: Self) {
                    *self = *self % modulus;
                }
            }

            impl<T: std::ops::Sub<Output = T>> std::ops::Sub for $newtype<T> {
                type Output = Self;

                fn sub(self, other: Self) -> Self {
                    Self(self.0 - other.0)
                }
            }

            // TRADEOFF: requires Copy
            impl<T: std::ops::SubAssign + std::ops::Sub<Output = T> + std::marker::Copy> std::ops::SubAssign for $newtype<T> {
                fn sub_assign(&mut self, other: Self) {
                    *self = *self - other
                }
            }

            impl<T: std::ops::Neg<Output = T>> std::ops::Neg for $newtype<T> {
                type Output = Self;

                fn neg(self) -> Self::Output {
                    Self(-self.0)
                }
            }

            impl<T> std::convert::From<T> for $newtype<T> {
                fn from(other: T) -> Self {
                    Self(other)
                }
            }

            impl<T> std::convert::AsRef<T> for $newtype<T> {
                fn as_ref(&self) -> &T {
                    &self.0
                }
            }

            impl<T> std::convert::AsMut<T> for $newtype<T> {
                fn as_mut(&mut self) -> &mut T {
                    &mut self.0
                }
            }
        )*
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        newtype!(Id, Nested);

        let mut id = Id(0);
        let mut id_1 = Id(1);
        // Deref
        assert_eq!(*id, 0);
        //DerefMut
        *id = 2;
        assert_eq!(*id, 2);
        // Add
        assert_eq!(id + id_1, Id(3));
        // AddAssign
        id += id_1;
        assert_eq!(id, Id(3));
        // Clone
        let id_2 = id.clone();
        assert_eq!(id, id_2);
        // Copy
        let id_2 = id;
        assert_eq!(id, id_2);
        // PartialEq
        assert_eq!(id, id);
        // Eq
        assert_eq!(id, id);
        // BitAnd
        assert_eq!(Id(1) & Id(2), Id(0));
        // BitAndAssign
        id_1 &= Id(2);
        assert_eq!(id_1, Id(0));
        // BitOr
        assert_eq!(Id(1) | Id(2), Id(3));
        // BitOrAssign
        id_1 |= Id(1);
        assert_eq!(id_1, Id(1));
        // BitXor
        assert_eq!(Id(1) ^ Id(2), Id(3));
        // BitXorAssign
        id_1 ^= Id(2);
        assert_eq!(id_1, Id(3));
        // Div
        assert_eq!(Id(2) / Id(2), Id(1));
        // DivAssign
        id_1 /= Id(2);
        assert_eq!(id_1, Id(1));
        // Mul
        assert_eq!(Id(1) * Id(2), Id(2));
        // MulAssign
        id_1 *= Id(2);
        assert_eq!(id_1, Id(2));
        // Not
        assert_eq!(!Id(0), Id(-1));
        // Ord
        assert_eq!(Id(0).cmp(&Id(0)), std::cmp::Ordering::Equal);
        // PartialOrd
        assert_eq!(Id(0).partial_cmp(&Id(0)), Some(std::cmp::Ordering::Equal));
        // Rem
        assert_eq!(Id(2) % Id(2), Id(0));
        // RemAssign
        id_1 %= Id(2);
        assert_eq!(id_1, Id(0));
        // Sub
        assert_eq!(Id(1) - Id(1), Id(0));
        // SubAssign
        id_1 -= Id(1);
        assert_eq!(id_1, Id(-1));
        // Neg
        assert_eq!(-Id(1), Id(-1));
    }

    #[test]
    fn nested() {
        newtype!(A, B);

        let a = A(B(5));

        let b: B<i32> = 5.into();

        let b: A<B<i32>> = b.into();

        assert_eq!(a + b, A(B(10)))
    }
}
