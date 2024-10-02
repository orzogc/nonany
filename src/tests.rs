macro_rules! test_size {
    (crate::$name:ident, $base:ident) => {
        const _:() = assert!(core::mem::size_of::<Option<crate::$name>>() == core::mem::size_of::<$base>());
    };
    ($name:ident, $base:ident) => {
        const _:() = assert!(core::mem::size_of::<Option<$name>>() == core::mem::size_of::<$base>());
    };
}

macro_rules! test_nonany {
    (i8, $niche:expr) => {
        test_nonany!(NonAnyI8, i8, $niche, true);
    };

    (u8, $niche:expr) => {
        test_nonany!(NonAnyU8, u8, $niche, false);
    };

    ($nonany:ident, $int:ident, $niche:expr, $signed:ident) => {
        const _: () = {
            const NICHE: $int = $niche;
            type NonAny = crate::$nonany::<{ NICHE }>;
            test_size!(NonAny, $int);

            let mut i = $int::MIN;
            let mut first = true;
            loop {
                if first {
                    first = false;
                } else {
                    if i == $int::MAX {
                        break;
                    }
                    i += 1;
                }

                let non = NonAny::new(i);

                let non = match non {
                    Some(non) => non,
                    None => {
                        assert!(i == NICHE);
                        continue;
                    }
                };

                assert!(non.get() == i);
                assert!(non.0.get() == i ^ NICHE);
                
                assert!(unsafe { NonAny::new_unchecked(i).get() } == non.get());

                test_nonany!(@signed, $signed, $int, NICHE, non, i);
            };
        };
    };

    (@signed, true, $int:ident, $niche:ident, $non:ident, $i:ident) => {
        assert!($non.is_positive() == $i.is_positive());
        assert!($non.is_negative() == $i.is_negative());

        if $i == $int::MIN {
             match $non.checked_abs() {
                Ok(_) => panic!(),
                Err(err) => match err {
                    crate::CheckedError::Overflow => (),
                    crate::CheckedError::Niche => panic!()
                }
            };
        } else if $i < 0 && $i.abs() == $niche {
            match $non.checked_abs() {
                Ok(_) => panic!(),
                Err(err) => match err {
                    crate::CheckedError::Overflow => panic!(),
                    crate::CheckedError::Niche => ()
                }
            };
        } else {
            let abs = match $non.checked_abs() {
                Ok(abs) => abs,
                Err(err) => match err {
                    crate::CheckedError::Overflow => panic!(),
                    crate::CheckedError::Niche => panic!()
                }
            };
            assert!(abs.get() == $i.abs())
        };
    };

    (@signed, false, $non:ident, $int:ident, $i:ident, $niche:ident) => {};
}

test_size!(crate::NonZeroI8, i8);
test_size!(crate::NonZeroI16, i16);
test_size!(crate::NonZeroI32, i32);
test_size!(crate::NonZeroI64, i64);
test_size!(crate::NonZeroI128, i128);
test_size!(crate::NonZeroIsize, isize);

test_size!(crate::NonMinI8, i8);
test_size!(crate::NonMinI16, i16);
test_size!(crate::NonMinI32, i32);
test_size!(crate::NonMinI64, i64);
test_size!(crate::NonMinI128, i128);
test_size!(crate::NonMinIsize, isize);

test_size!(crate::NonMaxI8, i8);
test_size!(crate::NonMaxI16, i16);
test_size!(crate::NonMaxI32, i32);
test_size!(crate::NonMaxI64, i64);
test_size!(crate::NonMaxI128, i128);
test_size!(crate::NonMaxIsize, isize);

test_size!(crate::NonZeroU8, u8);
test_size!(crate::NonZeroU16, u16);
test_size!(crate::NonZeroU32, u32);
test_size!(crate::NonZeroU64, u64);
test_size!(crate::NonZeroU128, u128);
test_size!(crate::NonZeroUsize, usize);

test_size!(crate::NonMinU8, u8);
test_size!(crate::NonMinU16, u16);
test_size!(crate::NonMinU32, u32);
test_size!(crate::NonMinU64, u64);
test_size!(crate::NonMinU128, u128);
test_size!(crate::NonMinUsize, usize);

test_size!(crate::NonMaxU8, u8);
test_size!(crate::NonMaxU16, u16);
test_size!(crate::NonMaxU32, u32);
test_size!(crate::NonMaxU64, u64);
test_size!(crate::NonMaxU128, u128);
test_size!(crate::NonMaxUsize, usize);

test_nonany!(i8, -128i8);
test_nonany!(i8, -127i8);
test_nonany!(i8, -126i8);
test_nonany!(i8, -125i8);
test_nonany!(i8, -124i8);
test_nonany!(i8, -123i8);
test_nonany!(i8, -122i8);
test_nonany!(i8, -121i8);
test_nonany!(i8, -120i8);
test_nonany!(i8, -119i8);
test_nonany!(i8, -118i8);
test_nonany!(i8, -117i8);
test_nonany!(i8, -116i8);
test_nonany!(i8, -115i8);
test_nonany!(i8, -114i8);
test_nonany!(i8, -113i8);
test_nonany!(i8, -112i8);
test_nonany!(i8, -111i8);
test_nonany!(i8, -110i8);
test_nonany!(i8, -109i8);
test_nonany!(i8, -108i8);
test_nonany!(i8, -107i8);
test_nonany!(i8, -106i8);
test_nonany!(i8, -105i8);
test_nonany!(i8, -104i8);
test_nonany!(i8, -103i8);
test_nonany!(i8, -102i8);
test_nonany!(i8, -101i8);
test_nonany!(i8, -100i8);
test_nonany!(i8, -99i8);
test_nonany!(i8, -98i8);
test_nonany!(i8, -97i8);
test_nonany!(i8, -96i8);
test_nonany!(i8, -95i8);
test_nonany!(i8, -94i8);
test_nonany!(i8, -93i8);
test_nonany!(i8, -92i8);
test_nonany!(i8, -91i8);
test_nonany!(i8, -90i8);
test_nonany!(i8, -89i8);
test_nonany!(i8, -88i8);
test_nonany!(i8, -87i8);
test_nonany!(i8, -86i8);
test_nonany!(i8, -85i8);
test_nonany!(i8, -84i8);
test_nonany!(i8, -83i8);
test_nonany!(i8, -82i8);
test_nonany!(i8, -81i8);
test_nonany!(i8, -80i8);
test_nonany!(i8, -79i8);
test_nonany!(i8, -78i8);
test_nonany!(i8, -77i8);
test_nonany!(i8, -76i8);
test_nonany!(i8, -75i8);
test_nonany!(i8, -74i8);
test_nonany!(i8, -73i8);
test_nonany!(i8, -72i8);
test_nonany!(i8, -71i8);
test_nonany!(i8, -70i8);
test_nonany!(i8, -69i8);
test_nonany!(i8, -68i8);
test_nonany!(i8, -67i8);
test_nonany!(i8, -66i8);
test_nonany!(i8, -65i8);
test_nonany!(i8, -64i8);
test_nonany!(i8, -63i8);
test_nonany!(i8, -62i8);
test_nonany!(i8, -61i8);
test_nonany!(i8, -60i8);
test_nonany!(i8, -59i8);
test_nonany!(i8, -58i8);
test_nonany!(i8, -57i8);
test_nonany!(i8, -56i8);
test_nonany!(i8, -55i8);
test_nonany!(i8, -54i8);
test_nonany!(i8, -53i8);
test_nonany!(i8, -52i8);
test_nonany!(i8, -51i8);
test_nonany!(i8, -50i8);
test_nonany!(i8, -49i8);
test_nonany!(i8, -48i8);
test_nonany!(i8, -47i8);
test_nonany!(i8, -46i8);
test_nonany!(i8, -45i8);
test_nonany!(i8, -44i8);
test_nonany!(i8, -43i8);
test_nonany!(i8, -42i8);
test_nonany!(i8, -41i8);
test_nonany!(i8, -40i8);
test_nonany!(i8, -39i8);
test_nonany!(i8, -38i8);
test_nonany!(i8, -37i8);
test_nonany!(i8, -36i8);
test_nonany!(i8, -35i8);
test_nonany!(i8, -34i8);
test_nonany!(i8, -33i8);
test_nonany!(i8, -32i8);
test_nonany!(i8, -31i8);
test_nonany!(i8, -30i8);
test_nonany!(i8, -29i8);
test_nonany!(i8, -28i8);
test_nonany!(i8, -27i8);
test_nonany!(i8, -26i8);
test_nonany!(i8, -25i8);
test_nonany!(i8, -24i8);
test_nonany!(i8, -23i8);
test_nonany!(i8, -22i8);
test_nonany!(i8, -21i8);
test_nonany!(i8, -20i8);
test_nonany!(i8, -19i8);
test_nonany!(i8, -18i8);
test_nonany!(i8, -17i8);
test_nonany!(i8, -16i8);
test_nonany!(i8, -15i8);
test_nonany!(i8, -14i8);
test_nonany!(i8, -13i8);
test_nonany!(i8, -12i8);
test_nonany!(i8, -11i8);
test_nonany!(i8, -10i8);
test_nonany!(i8, -9i8);
test_nonany!(i8, -8i8);
test_nonany!(i8, -7i8);
test_nonany!(i8, -6i8);
test_nonany!(i8, -5i8);
test_nonany!(i8, -4i8);
test_nonany!(i8, -3i8);
test_nonany!(i8, -2i8);
test_nonany!(i8, -1i8);
test_nonany!(i8, 0i8);
test_nonany!(i8, 1i8);
test_nonany!(i8, 2i8);
test_nonany!(i8, 3i8);
test_nonany!(i8, 4i8);
test_nonany!(i8, 5i8);
test_nonany!(i8, 6i8);
test_nonany!(i8, 7i8);
test_nonany!(i8, 8i8);
test_nonany!(i8, 9i8);
test_nonany!(i8, 10i8);
test_nonany!(i8, 11i8);
test_nonany!(i8, 12i8);
test_nonany!(i8, 13i8);
test_nonany!(i8, 14i8);
test_nonany!(i8, 15i8);
test_nonany!(i8, 16i8);
test_nonany!(i8, 17i8);
test_nonany!(i8, 18i8);
test_nonany!(i8, 19i8);
test_nonany!(i8, 20i8);
test_nonany!(i8, 21i8);
test_nonany!(i8, 22i8);
test_nonany!(i8, 23i8);
test_nonany!(i8, 24i8);
test_nonany!(i8, 25i8);
test_nonany!(i8, 26i8);
test_nonany!(i8, 27i8);
test_nonany!(i8, 28i8);
test_nonany!(i8, 29i8);
test_nonany!(i8, 30i8);
test_nonany!(i8, 31i8);
test_nonany!(i8, 32i8);
test_nonany!(i8, 33i8);
test_nonany!(i8, 34i8);
test_nonany!(i8, 35i8);
test_nonany!(i8, 36i8);
test_nonany!(i8, 37i8);
test_nonany!(i8, 38i8);
test_nonany!(i8, 39i8);
test_nonany!(i8, 40i8);
test_nonany!(i8, 41i8);
test_nonany!(i8, 42i8);
test_nonany!(i8, 43i8);
test_nonany!(i8, 44i8);
test_nonany!(i8, 45i8);
test_nonany!(i8, 46i8);
test_nonany!(i8, 47i8);
test_nonany!(i8, 48i8);
test_nonany!(i8, 49i8);
test_nonany!(i8, 50i8);
test_nonany!(i8, 51i8);
test_nonany!(i8, 52i8);
test_nonany!(i8, 53i8);
test_nonany!(i8, 54i8);
test_nonany!(i8, 55i8);
test_nonany!(i8, 56i8);
test_nonany!(i8, 57i8);
test_nonany!(i8, 58i8);
test_nonany!(i8, 59i8);
test_nonany!(i8, 60i8);
test_nonany!(i8, 61i8);
test_nonany!(i8, 62i8);
test_nonany!(i8, 63i8);
test_nonany!(i8, 64i8);
test_nonany!(i8, 65i8);
test_nonany!(i8, 66i8);
test_nonany!(i8, 67i8);
test_nonany!(i8, 68i8);
test_nonany!(i8, 69i8);
test_nonany!(i8, 70i8);
test_nonany!(i8, 71i8);
test_nonany!(i8, 72i8);
test_nonany!(i8, 73i8);
test_nonany!(i8, 74i8);
test_nonany!(i8, 75i8);
test_nonany!(i8, 76i8);
test_nonany!(i8, 77i8);
test_nonany!(i8, 78i8);
test_nonany!(i8, 79i8);
test_nonany!(i8, 80i8);
test_nonany!(i8, 81i8);
test_nonany!(i8, 82i8);
test_nonany!(i8, 83i8);
test_nonany!(i8, 84i8);
test_nonany!(i8, 85i8);
test_nonany!(i8, 86i8);
test_nonany!(i8, 87i8);
test_nonany!(i8, 88i8);
test_nonany!(i8, 89i8);
test_nonany!(i8, 90i8);
test_nonany!(i8, 91i8);
test_nonany!(i8, 92i8);
test_nonany!(i8, 93i8);
test_nonany!(i8, 94i8);
test_nonany!(i8, 95i8);
test_nonany!(i8, 96i8);
test_nonany!(i8, 97i8);
test_nonany!(i8, 98i8);
test_nonany!(i8, 99i8);
test_nonany!(i8, 100i8);
test_nonany!(i8, 101i8);
test_nonany!(i8, 102i8);
test_nonany!(i8, 103i8);
test_nonany!(i8, 104i8);
test_nonany!(i8, 105i8);
test_nonany!(i8, 106i8);
test_nonany!(i8, 107i8);
test_nonany!(i8, 108i8);
test_nonany!(i8, 109i8);
test_nonany!(i8, 110i8);
test_nonany!(i8, 111i8);
test_nonany!(i8, 112i8);
test_nonany!(i8, 113i8);
test_nonany!(i8, 114i8);
test_nonany!(i8, 115i8);
test_nonany!(i8, 116i8);
test_nonany!(i8, 117i8);
test_nonany!(i8, 118i8);
test_nonany!(i8, 119i8);
test_nonany!(i8, 120i8);
test_nonany!(i8, 121i8);
test_nonany!(i8, 122i8);
test_nonany!(i8, 123i8);
test_nonany!(i8, 124i8);
test_nonany!(i8, 125i8);
test_nonany!(i8, 126i8);
test_nonany!(i8, 127i8);
test_nonany!(u8, 0u8);
test_nonany!(u8, 1u8);
test_nonany!(u8, 2u8);
test_nonany!(u8, 3u8);
test_nonany!(u8, 4u8);
test_nonany!(u8, 5u8);
test_nonany!(u8, 6u8);
test_nonany!(u8, 7u8);
test_nonany!(u8, 8u8);
test_nonany!(u8, 9u8);
test_nonany!(u8, 10u8);
test_nonany!(u8, 11u8);
test_nonany!(u8, 12u8);
test_nonany!(u8, 13u8);
test_nonany!(u8, 14u8);
test_nonany!(u8, 15u8);
test_nonany!(u8, 16u8);
test_nonany!(u8, 17u8);
test_nonany!(u8, 18u8);
test_nonany!(u8, 19u8);
test_nonany!(u8, 20u8);
test_nonany!(u8, 21u8);
test_nonany!(u8, 22u8);
test_nonany!(u8, 23u8);
test_nonany!(u8, 24u8);
test_nonany!(u8, 25u8);
test_nonany!(u8, 26u8);
test_nonany!(u8, 27u8);
test_nonany!(u8, 28u8);
test_nonany!(u8, 29u8);
test_nonany!(u8, 30u8);
test_nonany!(u8, 31u8);
test_nonany!(u8, 32u8);
test_nonany!(u8, 33u8);
test_nonany!(u8, 34u8);
test_nonany!(u8, 35u8);
test_nonany!(u8, 36u8);
test_nonany!(u8, 37u8);
test_nonany!(u8, 38u8);
test_nonany!(u8, 39u8);
test_nonany!(u8, 40u8);
test_nonany!(u8, 41u8);
test_nonany!(u8, 42u8);
test_nonany!(u8, 43u8);
test_nonany!(u8, 44u8);
test_nonany!(u8, 45u8);
test_nonany!(u8, 46u8);
test_nonany!(u8, 47u8);
test_nonany!(u8, 48u8);
test_nonany!(u8, 49u8);
test_nonany!(u8, 50u8);
test_nonany!(u8, 51u8);
test_nonany!(u8, 52u8);
test_nonany!(u8, 53u8);
test_nonany!(u8, 54u8);
test_nonany!(u8, 55u8);
test_nonany!(u8, 56u8);
test_nonany!(u8, 57u8);
test_nonany!(u8, 58u8);
test_nonany!(u8, 59u8);
test_nonany!(u8, 60u8);
test_nonany!(u8, 61u8);
test_nonany!(u8, 62u8);
test_nonany!(u8, 63u8);
test_nonany!(u8, 64u8);
test_nonany!(u8, 65u8);
test_nonany!(u8, 66u8);
test_nonany!(u8, 67u8);
test_nonany!(u8, 68u8);
test_nonany!(u8, 69u8);
test_nonany!(u8, 70u8);
test_nonany!(u8, 71u8);
test_nonany!(u8, 72u8);
test_nonany!(u8, 73u8);
test_nonany!(u8, 74u8);
test_nonany!(u8, 75u8);
test_nonany!(u8, 76u8);
test_nonany!(u8, 77u8);
test_nonany!(u8, 78u8);
test_nonany!(u8, 79u8);
test_nonany!(u8, 80u8);
test_nonany!(u8, 81u8);
test_nonany!(u8, 82u8);
test_nonany!(u8, 83u8);
test_nonany!(u8, 84u8);
test_nonany!(u8, 85u8);
test_nonany!(u8, 86u8);
test_nonany!(u8, 87u8);
test_nonany!(u8, 88u8);
test_nonany!(u8, 89u8);
test_nonany!(u8, 90u8);
test_nonany!(u8, 91u8);
test_nonany!(u8, 92u8);
test_nonany!(u8, 93u8);
test_nonany!(u8, 94u8);
test_nonany!(u8, 95u8);
test_nonany!(u8, 96u8);
test_nonany!(u8, 97u8);
test_nonany!(u8, 98u8);
test_nonany!(u8, 99u8);
test_nonany!(u8, 100u8);
test_nonany!(u8, 101u8);
test_nonany!(u8, 102u8);
test_nonany!(u8, 103u8);
test_nonany!(u8, 104u8);
test_nonany!(u8, 105u8);
test_nonany!(u8, 106u8);
test_nonany!(u8, 107u8);
test_nonany!(u8, 108u8);
test_nonany!(u8, 109u8);
test_nonany!(u8, 110u8);
test_nonany!(u8, 111u8);
test_nonany!(u8, 112u8);
test_nonany!(u8, 113u8);
test_nonany!(u8, 114u8);
test_nonany!(u8, 115u8);
test_nonany!(u8, 116u8);
test_nonany!(u8, 117u8);
test_nonany!(u8, 118u8);
test_nonany!(u8, 119u8);
test_nonany!(u8, 120u8);
test_nonany!(u8, 121u8);
test_nonany!(u8, 122u8);
test_nonany!(u8, 123u8);
test_nonany!(u8, 124u8);
test_nonany!(u8, 125u8);
test_nonany!(u8, 126u8);
test_nonany!(u8, 127u8);
test_nonany!(u8, 128u8);
test_nonany!(u8, 129u8);
test_nonany!(u8, 130u8);
test_nonany!(u8, 131u8);
test_nonany!(u8, 132u8);
test_nonany!(u8, 133u8);
test_nonany!(u8, 134u8);
test_nonany!(u8, 135u8);
test_nonany!(u8, 136u8);
test_nonany!(u8, 137u8);
test_nonany!(u8, 138u8);
test_nonany!(u8, 139u8);
test_nonany!(u8, 140u8);
test_nonany!(u8, 141u8);
test_nonany!(u8, 142u8);
test_nonany!(u8, 143u8);
test_nonany!(u8, 144u8);
test_nonany!(u8, 145u8);
test_nonany!(u8, 146u8);
test_nonany!(u8, 147u8);
test_nonany!(u8, 148u8);
test_nonany!(u8, 149u8);
test_nonany!(u8, 150u8);
test_nonany!(u8, 151u8);
test_nonany!(u8, 152u8);
test_nonany!(u8, 153u8);
test_nonany!(u8, 154u8);
test_nonany!(u8, 155u8);
test_nonany!(u8, 156u8);
test_nonany!(u8, 157u8);
test_nonany!(u8, 158u8);
test_nonany!(u8, 159u8);
test_nonany!(u8, 160u8);
test_nonany!(u8, 161u8);
test_nonany!(u8, 162u8);
test_nonany!(u8, 163u8);
test_nonany!(u8, 164u8);
test_nonany!(u8, 165u8);
test_nonany!(u8, 166u8);
test_nonany!(u8, 167u8);
test_nonany!(u8, 168u8);
test_nonany!(u8, 169u8);
test_nonany!(u8, 170u8);
test_nonany!(u8, 171u8);
test_nonany!(u8, 172u8);
test_nonany!(u8, 173u8);
test_nonany!(u8, 174u8);
test_nonany!(u8, 175u8);
test_nonany!(u8, 176u8);
test_nonany!(u8, 177u8);
test_nonany!(u8, 178u8);
test_nonany!(u8, 179u8);
test_nonany!(u8, 180u8);
test_nonany!(u8, 181u8);
test_nonany!(u8, 182u8);
test_nonany!(u8, 183u8);
test_nonany!(u8, 184u8);
test_nonany!(u8, 185u8);
test_nonany!(u8, 186u8);
test_nonany!(u8, 187u8);
test_nonany!(u8, 188u8);
test_nonany!(u8, 189u8);
test_nonany!(u8, 190u8);
test_nonany!(u8, 191u8);
test_nonany!(u8, 192u8);
test_nonany!(u8, 193u8);
test_nonany!(u8, 194u8);
test_nonany!(u8, 195u8);
test_nonany!(u8, 196u8);
test_nonany!(u8, 197u8);
test_nonany!(u8, 198u8);
test_nonany!(u8, 199u8);
test_nonany!(u8, 200u8);
test_nonany!(u8, 201u8);
test_nonany!(u8, 202u8);
test_nonany!(u8, 203u8);
test_nonany!(u8, 204u8);
test_nonany!(u8, 205u8);
test_nonany!(u8, 206u8);
test_nonany!(u8, 207u8);
test_nonany!(u8, 208u8);
test_nonany!(u8, 209u8);
test_nonany!(u8, 210u8);
test_nonany!(u8, 211u8);
test_nonany!(u8, 212u8);
test_nonany!(u8, 213u8);
test_nonany!(u8, 214u8);
test_nonany!(u8, 215u8);
test_nonany!(u8, 216u8);
test_nonany!(u8, 217u8);
test_nonany!(u8, 218u8);
test_nonany!(u8, 219u8);
test_nonany!(u8, 220u8);
test_nonany!(u8, 221u8);
test_nonany!(u8, 222u8);
test_nonany!(u8, 223u8);
test_nonany!(u8, 224u8);
test_nonany!(u8, 225u8);
test_nonany!(u8, 226u8);
test_nonany!(u8, 227u8);
test_nonany!(u8, 228u8);
test_nonany!(u8, 229u8);
test_nonany!(u8, 230u8);
test_nonany!(u8, 231u8);
test_nonany!(u8, 232u8);
test_nonany!(u8, 233u8);
test_nonany!(u8, 234u8);
test_nonany!(u8, 235u8);
test_nonany!(u8, 236u8);
test_nonany!(u8, 237u8);
test_nonany!(u8, 238u8);
test_nonany!(u8, 239u8);
test_nonany!(u8, 240u8);
test_nonany!(u8, 241u8);
test_nonany!(u8, 242u8);
test_nonany!(u8, 243u8);
test_nonany!(u8, 244u8);
test_nonany!(u8, 245u8);
test_nonany!(u8, 246u8);
test_nonany!(u8, 247u8);
test_nonany!(u8, 248u8);
test_nonany!(u8, 249u8);
test_nonany!(u8, 250u8);
test_nonany!(u8, 251u8);
test_nonany!(u8, 252u8);
test_nonany!(u8, 253u8);
test_nonany!(u8, 254u8);
test_nonany!(u8, 255u8);