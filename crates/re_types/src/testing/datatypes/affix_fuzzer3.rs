// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/testing/datatypes/fuzzy.fbs".

#![allow(trivial_numeric_casts)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

#[derive(Clone, Debug, PartialEq)]
pub enum AffixFuzzer3 {
    Degrees(f32),
    Radians(Option<f32>),
    Craziness(Vec<crate::testing::datatypes::AffixFuzzer1>),
    FixedSizeShenanigans([f32; 3usize]),
}

impl<'a> From<AffixFuzzer3> for ::std::borrow::Cow<'a, AffixFuzzer3> {
    #[inline]
    fn from(value: AffixFuzzer3) -> Self {
        std::borrow::Cow::Owned(value)
    }
}

impl<'a> From<&'a AffixFuzzer3> for ::std::borrow::Cow<'a, AffixFuzzer3> {
    #[inline]
    fn from(value: &'a AffixFuzzer3) -> Self {
        std::borrow::Cow::Borrowed(value)
    }
}

impl crate::Loggable for AffixFuzzer3 {
    type Name = crate::DatatypeName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.testing.datatypes.AffixFuzzer3".into()
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        use ::arrow2::datatypes::*;
        DataType::Union(
            vec![
                Field {
                    name: "_null_markers".to_owned(),
                    data_type: DataType::Null,
                    is_nullable: true,
                    metadata: [].into(),
                },
                Field {
                    name: "degrees".to_owned(),
                    data_type: DataType::Float32,
                    is_nullable: false,
                    metadata: [].into(),
                },
                Field {
                    name: "radians".to_owned(),
                    data_type: DataType::Float32,
                    is_nullable: false,
                    metadata: [].into(),
                },
                Field {
                    name: "craziness".to_owned(),
                    data_type: DataType::List(Box::new(Field {
                        name: "item".to_owned(),
                        data_type: <crate::testing::datatypes::AffixFuzzer1>::arrow_datatype(),
                        is_nullable: false,
                        metadata: [].into(),
                    })),
                    is_nullable: false,
                    metadata: [].into(),
                },
                Field {
                    name: "fixed_size_shenanigans".to_owned(),
                    data_type: DataType::FixedSizeList(
                        Box::new(Field {
                            name: "item".to_owned(),
                            data_type: DataType::Float32,
                            is_nullable: false,
                            metadata: [].into(),
                        }),
                        3usize,
                    ),
                    is_nullable: false,
                    metadata: [].into(),
                },
            ],
            Some(vec![0i32, 1i32, 2i32, 3i32, 4i32]),
            UnionMode::Dense,
        )
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn try_to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> crate::SerializationResult<Box<dyn ::arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        use crate::{Loggable as _, ResultExt as _};
        use ::arrow2::{array::*, datatypes::*};
        Ok({
            let data: Vec<_> = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    datum
                })
                .collect();
            UnionArray::new(
                <crate::testing::datatypes::AffixFuzzer3>::arrow_datatype(),
                data.iter()
                    .map(|a| match a.as_deref() {
                        None => 0,
                        Some(AffixFuzzer3::Degrees(_)) => 1i8,
                        Some(AffixFuzzer3::Radians(_)) => 2i8,
                        Some(AffixFuzzer3::Craziness(_)) => 3i8,
                        Some(AffixFuzzer3::FixedSizeShenanigans(_)) => 4i8,
                    })
                    .collect(),
                vec![
                    NullArray::new(DataType::Null, data.iter().filter(|v| v.is_none()).count())
                        .boxed(),
                    {
                        let (somes, degrees): (Vec<_>, Vec<_>) = data
                            .iter()
                            .filter(|datum| {
                                matches!(datum.as_deref(), Some(AffixFuzzer3::Degrees(_)))
                            })
                            .map(|datum| {
                                let datum = match datum.as_deref() {
                                    Some(AffixFuzzer3::Degrees(v)) => Some(v.clone()),
                                    _ => None,
                                };
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let degrees_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        PrimitiveArray::new(
                            DataType::Float32,
                            degrees.into_iter().map(|v| v.unwrap_or_default()).collect(),
                            degrees_bitmap,
                        )
                        .boxed()
                    },
                    {
                        let (somes, radians): (Vec<_>, Vec<_>) = data
                            .iter()
                            .filter(|datum| {
                                matches!(datum.as_deref(), Some(AffixFuzzer3::Radians(_)))
                            })
                            .map(|datum| {
                                let datum = match datum.as_deref() {
                                    Some(AffixFuzzer3::Radians(v)) => Some(v.clone()),
                                    _ => None,
                                }
                                .flatten();
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let radians_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        PrimitiveArray::new(
                            DataType::Float32,
                            radians.into_iter().map(|v| v.unwrap_or_default()).collect(),
                            radians_bitmap,
                        )
                        .boxed()
                    },
                    {
                        let (somes, craziness): (Vec<_>, Vec<_>) = data
                            .iter()
                            .filter(|datum| {
                                matches!(datum.as_deref(), Some(AffixFuzzer3::Craziness(_)))
                            })
                            .map(|datum| {
                                let datum = match datum.as_deref() {
                                    Some(AffixFuzzer3::Craziness(v)) => Some(v.clone()),
                                    _ => None,
                                };
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let craziness_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            use arrow2::{buffer::Buffer, offset::OffsetsBuffer};
                            let craziness_inner_data: Vec<_> = craziness
                                .iter()
                                .flatten()
                                .flatten()
                                .cloned()
                                .map(Some)
                                .collect();
                            let craziness_inner_bitmap: Option<::arrow2::bitmap::Bitmap> = None;
                            let offsets = ::arrow2::offset::Offsets::<i32>::try_from_lengths(
                                craziness.iter().map(|opt| {
                                    opt.as_ref().map(|datum| datum.len()).unwrap_or_default()
                                }),
                            )
                            .unwrap()
                            .into();
                            ListArray::new(
                                DataType::List(Box::new(Field {
                                    name: "item".to_owned(),
                                    data_type:
                                        <crate::testing::datatypes::AffixFuzzer1>::arrow_datatype(),
                                    is_nullable: false,
                                    metadata: [].into(),
                                })),
                                offsets,
                                {
                                    _ = craziness_inner_bitmap;
                                    crate::testing::datatypes::AffixFuzzer1::try_to_arrow_opt(
                                        craziness_inner_data,
                                    )?
                                },
                                craziness_bitmap,
                            )
                            .boxed()
                        }
                    },
                    {
                        let (somes, fixed_size_shenanigans): (Vec<_>, Vec<_>) = data
                            .iter()
                            .filter(|datum| {
                                matches!(
                                    datum.as_deref(),
                                    Some(AffixFuzzer3::FixedSizeShenanigans(_))
                                )
                            })
                            .map(|datum| {
                                let datum = match datum.as_deref() {
                                    Some(AffixFuzzer3::FixedSizeShenanigans(v)) => Some(v.clone()),
                                    _ => None,
                                };
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let fixed_size_shenanigans_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            use arrow2::{buffer::Buffer, offset::OffsetsBuffer};
                            let fixed_size_shenanigans_inner_data: Vec<_> = fixed_size_shenanigans
                                .iter()
                                .flatten()
                                .flatten()
                                .cloned()
                                .map(Some)
                                .collect();
                            let fixed_size_shenanigans_inner_bitmap: Option<
                                ::arrow2::bitmap::Bitmap,
                            > = fixed_size_shenanigans_bitmap.as_ref().map(|bitmap| {
                                bitmap
                                    .iter()
                                    .map(|i| std::iter::repeat(i).take(3usize))
                                    .flatten()
                                    .collect::<Vec<_>>()
                                    .into()
                            });
                            FixedSizeListArray::new(
                                DataType::FixedSizeList(
                                    Box::new(Field {
                                        name: "item".to_owned(),
                                        data_type: DataType::Float32,
                                        is_nullable: false,
                                        metadata: [].into(),
                                    }),
                                    3usize,
                                ),
                                PrimitiveArray::new(
                                    DataType::Float32,
                                    fixed_size_shenanigans_inner_data
                                        .into_iter()
                                        .map(|v| v.unwrap_or_default())
                                        .collect(),
                                    fixed_size_shenanigans_inner_bitmap,
                                )
                                .boxed(),
                                fixed_size_shenanigans_bitmap,
                            )
                            .boxed()
                        }
                    },
                ],
                Some({
                    let mut degrees_offset = 0;
                    let mut radians_offset = 0;
                    let mut craziness_offset = 0;
                    let mut fixed_size_shenanigans_offset = 0;
                    let mut nulls_offset = 0;
                    data.iter()
                        .map(|v| match v.as_deref() {
                            None => {
                                let offset = nulls_offset;
                                nulls_offset += 1;
                                offset
                            }
                            Some(AffixFuzzer3::Degrees(_)) => {
                                let offset = degrees_offset;
                                degrees_offset += 1;
                                offset
                            }
                            Some(AffixFuzzer3::Radians(_)) => {
                                let offset = radians_offset;
                                radians_offset += 1;
                                offset
                            }
                            Some(AffixFuzzer3::Craziness(_)) => {
                                let offset = craziness_offset;
                                craziness_offset += 1;
                                offset
                            }
                            Some(AffixFuzzer3::FixedSizeShenanigans(_)) => {
                                let offset = fixed_size_shenanigans_offset;
                                fixed_size_shenanigans_offset += 1;
                                offset
                            }
                        })
                        .collect()
                }),
            )
            .boxed()
        })
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn try_from_arrow_opt(
        arrow_data: &dyn ::arrow2::array::Array,
    ) -> crate::DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        use crate::{Loggable as _, ResultExt as _};
        use ::arrow2::{array::*, buffer::*, datatypes::*};
        Ok({
            let arrow_data = arrow_data
                .as_any()
                .downcast_ref::<::arrow2::array::UnionArray>()
                .ok_or_else(|| {
                    crate::DeserializationError::datatype_mismatch(
                        DataType::Union(
                            vec![
                            Field { name : "_null_markers".to_owned(), data_type :
                            DataType::Null, is_nullable : true, metadata : [].into(), },
                            Field { name : "degrees".to_owned(), data_type :
                            DataType::Float32, is_nullable : false, metadata : [].into(),
                            }, Field { name : "radians".to_owned(), data_type :
                            DataType::Float32, is_nullable : false, metadata : [].into(),
                            }, Field { name : "craziness".to_owned(), data_type :
                            DataType::List(Box::new(Field { name : "item".to_owned(),
                            data_type : < crate ::testing::datatypes::AffixFuzzer1 >
                            ::arrow_datatype(), is_nullable : false, metadata : []
                            .into(), })), is_nullable : false, metadata : [].into(), },
                            Field { name : "fixed_size_shenanigans".to_owned(), data_type
                            : DataType::FixedSizeList(Box::new(Field { name : "item"
                            .to_owned(), data_type : DataType::Float32, is_nullable :
                            false, metadata : [].into(), }), 3usize), is_nullable :
                            false, metadata : [].into(), },
                        ],
                            Some(vec![0i32, 1i32, 2i32, 3i32, 4i32]),
                            UnionMode::Dense,
                        ),
                        arrow_data.data_type().clone(),
                    )
                })
                .with_context("rerun.testing.datatypes.AffixFuzzer3")?;
            if arrow_data.is_empty() {
                Vec::new()
            } else {
                let (arrow_data_types, arrow_data_arrays) =
                    (arrow_data.types(), arrow_data.fields());
                let arrow_data_offsets = arrow_data
                    .offsets()
                    .ok_or_else(|| {
                        crate::DeserializationError::datatype_mismatch(
                            Self::arrow_datatype(),
                            arrow_data.data_type().clone(),
                        )
                    })
                    .with_context("rerun.testing.datatypes.AffixFuzzer3")?;
                if arrow_data_types.len() != arrow_data_offsets.len() {
                    return Err(crate::DeserializationError::offset_slice_oob(
                        (0, arrow_data_types.len()),
                        arrow_data_offsets.len(),
                    ))
                    .with_context("rerun.testing.datatypes.AffixFuzzer3");
                }
                let degrees = {
                    if 1usize >= arrow_data_arrays.len() {
                        return Ok(Vec::new());
                    }
                    let arrow_data = &*arrow_data_arrays[1usize];
                    arrow_data
                        .as_any()
                        .downcast_ref::<Float32Array>()
                        .ok_or_else(|| {
                            crate::DeserializationError::datatype_mismatch(
                                DataType::Float32,
                                arrow_data.data_type().clone(),
                            )
                        })
                        .with_context("rerun.testing.datatypes.AffixFuzzer3#degrees")?
                        .into_iter()
                        .map(|opt| opt.copied())
                        .collect::<Vec<_>>()
                };
                let radians = {
                    if 2usize >= arrow_data_arrays.len() {
                        return Ok(Vec::new());
                    }
                    let arrow_data = &*arrow_data_arrays[2usize];
                    arrow_data
                        .as_any()
                        .downcast_ref::<Float32Array>()
                        .ok_or_else(|| {
                            crate::DeserializationError::datatype_mismatch(
                                DataType::Float32,
                                arrow_data.data_type().clone(),
                            )
                        })
                        .with_context("rerun.testing.datatypes.AffixFuzzer3#radians")?
                        .into_iter()
                        .map(|opt| opt.copied())
                        .collect::<Vec<_>>()
                };
                let craziness = {
                    if 3usize >= arrow_data_arrays.len() {
                        return Ok(Vec::new());
                    }
                    let arrow_data = &*arrow_data_arrays[3usize];
                    {
                        let arrow_data = arrow_data
                            .as_any()
                            .downcast_ref::<::arrow2::array::ListArray<i32>>()
                            .ok_or_else(|| crate::DeserializationError::datatype_mismatch(
                                DataType::List(
                                    Box::new(Field {
                                        name: "item".to_owned(),
                                        data_type: <crate::testing::datatypes::AffixFuzzer1>::arrow_datatype(),
                                        is_nullable: false,
                                        metadata: [].into(),
                                    }),
                                ),
                                arrow_data.data_type().clone(),
                            ))
                            .with_context(
                                "rerun.testing.datatypes.AffixFuzzer3#craziness",
                            )?;
                        if arrow_data.is_empty() {
                            Vec::new()
                        } else {
                            let arrow_data_inner = {
                                let arrow_data_inner = &**arrow_data.values();
                                crate::testing::datatypes::AffixFuzzer1::try_from_arrow_opt(
                                        arrow_data_inner,
                                    )
                                    .with_context(
                                        "rerun.testing.datatypes.AffixFuzzer3#craziness",
                                    )?
                                    .into_iter()
                                    .collect::<Vec<_>>()
                            };
                            let offsets = arrow_data.offsets();
                            arrow2::bitmap::utils::ZipValidity::new_with_validity(
                                    offsets.iter().zip(offsets.lengths()),
                                    arrow_data.validity(),
                                )
                                .map(|elem| {
                                    elem
                                        .map(|(start, len)| {
                                            let start = *start as usize;
                                            let end = start + len;
                                            if end as usize > arrow_data_inner.len() {
                                                return Err(
                                                    crate::DeserializationError::offset_slice_oob(
                                                        (start, end),
                                                        arrow_data_inner.len(),
                                                    ),
                                                );
                                            }

                                            #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                            let data = unsafe {
                                                arrow_data_inner.get_unchecked(start as usize..end as usize)
                                            };
                                            let data = data
                                                .iter()
                                                .cloned()
                                                .map(Option::unwrap_or_default)
                                                .collect();
                                            Ok(data)
                                        })
                                        .transpose()
                                })
                                .collect::<crate::DeserializationResult<Vec<Option<_>>>>()?
                        }
                            .into_iter()
                    }
                        .collect::<Vec<_>>()
                };
                let fixed_size_shenanigans = {
                    if 4usize >= arrow_data_arrays.len() {
                        return Ok(Vec::new());
                    }
                    let arrow_data = &*arrow_data_arrays[4usize];
                    {
                        let arrow_data = arrow_data
                            .as_any()
                            .downcast_ref::<::arrow2::array::FixedSizeListArray>()
                            .ok_or_else(|| crate::DeserializationError::datatype_mismatch(
                                DataType::FixedSizeList(
                                    Box::new(Field {
                                        name: "item".to_owned(),
                                        data_type: DataType::Float32,
                                        is_nullable: false,
                                        metadata: [].into(),
                                    }),
                                    3usize,
                                ),
                                arrow_data.data_type().clone(),
                            ))
                            .with_context(
                                "rerun.testing.datatypes.AffixFuzzer3#fixed_size_shenanigans",
                            )?;
                        if arrow_data.is_empty() {
                            Vec::new()
                        } else {
                            let offsets = (0..)
                                .step_by(3usize)
                                .zip((3usize..).step_by(3usize).take(arrow_data.len()));
                            let arrow_data_inner = {
                                let arrow_data_inner = &**arrow_data.values();
                                arrow_data_inner
                                    .as_any()
                                    .downcast_ref::<Float32Array>()
                                    .ok_or_else(|| crate::DeserializationError::datatype_mismatch(
                                        DataType::Float32,
                                        arrow_data_inner.data_type().clone(),
                                    ))
                                    .with_context(
                                        "rerun.testing.datatypes.AffixFuzzer3#fixed_size_shenanigans",
                                    )?
                                    .into_iter()
                                    .map(|opt| opt.copied())
                                    .collect::<Vec<_>>()
                            };
                            arrow2::bitmap::utils::ZipValidity::new_with_validity(
                                    offsets,
                                    arrow_data.validity(),
                                )
                                .map(|elem| {
                                    elem
                                        .map(|(start, end)| {
                                            debug_assert!(end - start == 3usize);
                                            if end as usize > arrow_data_inner.len() {
                                                return Err(
                                                    crate::DeserializationError::offset_slice_oob(
                                                        (start, end),
                                                        arrow_data_inner.len(),
                                                    ),
                                                );
                                            }

                                            #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                            let data = unsafe {
                                                arrow_data_inner.get_unchecked(start as usize..end as usize)
                                            };
                                            let data = data
                                                .iter()
                                                .cloned()
                                                .map(Option::unwrap_or_default);
                                            let arr = array_init::from_iter(data).unwrap();
                                            Ok(arr)
                                        })
                                        .transpose()
                                })
                                .collect::<crate::DeserializationResult<Vec<Option<_>>>>()?
                        }
                            .into_iter()
                    }
                        .collect::<Vec<_>>()
                };
                arrow_data_types
                    .iter()
                    .enumerate()
                    .map(|(i, typ)| {
                        let offset = arrow_data_offsets[i];
                        if *typ == 0 {
                            Ok(None)
                        } else {
                            Ok(
                                Some(
                                    match typ {
                                        1i8 => {
                                            AffixFuzzer3::Degrees({
                                                if offset as usize >= degrees.len() {
                                                    return Err(
                                                            crate::DeserializationError::offset_oob(
                                                                offset as _,
                                                                degrees.len(),
                                                            ),
                                                        )
                                                        .with_context(
                                                            "rerun.testing.datatypes.AffixFuzzer3#degrees",
                                                        );
                                                }

                                                #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                                unsafe { degrees.get_unchecked(offset as usize) }
                                                    .clone()
                                                    .ok_or_else(crate::DeserializationError::missing_data)
                                                    .with_context(
                                                        "rerun.testing.datatypes.AffixFuzzer3#degrees",
                                                    )?
                                            })
                                        }
                                        2i8 => {
                                            AffixFuzzer3::Radians({
                                                if offset as usize >= radians.len() {
                                                    return Err(
                                                            crate::DeserializationError::offset_oob(
                                                                offset as _,
                                                                radians.len(),
                                                            ),
                                                        )
                                                        .with_context(
                                                            "rerun.testing.datatypes.AffixFuzzer3#radians",
                                                        );
                                                }

                                                #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                                unsafe { radians.get_unchecked(offset as usize) }.clone()
                                            })
                                        }
                                        3i8 => {
                                            AffixFuzzer3::Craziness({
                                                if offset as usize >= craziness.len() {
                                                    return Err(
                                                            crate::DeserializationError::offset_oob(
                                                                offset as _,
                                                                craziness.len(),
                                                            ),
                                                        )
                                                        .with_context(
                                                            "rerun.testing.datatypes.AffixFuzzer3#craziness",
                                                        );
                                                }

                                                #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                                unsafe { craziness.get_unchecked(offset as usize) }
                                                    .clone()
                                                    .ok_or_else(crate::DeserializationError::missing_data)
                                                    .with_context(
                                                        "rerun.testing.datatypes.AffixFuzzer3#craziness",
                                                    )?
                                            })
                                        }
                                        4i8 => {
                                            AffixFuzzer3::FixedSizeShenanigans({
                                                if offset as usize >= fixed_size_shenanigans.len() {
                                                    return Err(
                                                            crate::DeserializationError::offset_oob(
                                                                offset as _,
                                                                fixed_size_shenanigans.len(),
                                                            ),
                                                        )
                                                        .with_context(
                                                            "rerun.testing.datatypes.AffixFuzzer3#fixed_size_shenanigans",
                                                        );
                                                }

                                                #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                                unsafe {
                                                    fixed_size_shenanigans.get_unchecked(offset as usize)
                                                }
                                                    .clone()
                                                    .ok_or_else(crate::DeserializationError::missing_data)
                                                    .with_context(
                                                        "rerun.testing.datatypes.AffixFuzzer3#fixed_size_shenanigans",
                                                    )?
                                            })
                                        }
                                        _ => {
                                            return Err(
                                                    crate::DeserializationError::missing_union_arm(
                                                        Self::arrow_datatype(),
                                                        "<invalid>",
                                                        *typ as _,
                                                    ),
                                                )
                                                .with_context("rerun.testing.datatypes.AffixFuzzer3");
                                        }
                                    },
                                ),
                            )
                        }
                    })
                    .collect::<crate::DeserializationResult<Vec<_>>>()
                    .with_context("rerun.testing.datatypes.AffixFuzzer3")?
            }
        })
    }
}