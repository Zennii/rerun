# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/testing/components/fuzzy.fbs".

# You can extend this class by creating a "AffixFuzzer8Ext" class in "affix_fuzzer8_ext.py".

from __future__ import annotations

from typing import Any, Sequence, Union

import numpy as np
import numpy.typing as npt
import pyarrow as pa
from attrs import define, field
from rerun._baseclasses import (
    BaseExtensionArray,
    BaseExtensionType,
)
from rerun._converters import (
    float_or_none,
)

__all__ = ["AffixFuzzer8", "AffixFuzzer8Array", "AffixFuzzer8ArrayLike", "AffixFuzzer8Like", "AffixFuzzer8Type"]


@define
class AffixFuzzer8:
    # You can define your own __init__ function as a member of AffixFuzzer8Ext in affix_fuzzer8_ext.py

    single_float_optional: float | None = field(default=None, converter=float_or_none)

    def __array__(self, dtype: npt.DTypeLike = None) -> npt.NDArray[Any]:
        # You can define your own __array__ function as a member of AffixFuzzer8Ext in affix_fuzzer8_ext.py
        return np.asarray(self.single_float_optional, dtype=dtype)


AffixFuzzer8Like = AffixFuzzer8
AffixFuzzer8ArrayLike = Union[
    AffixFuzzer8,
    Sequence[AffixFuzzer8Like],
]


# --- Arrow support ---


class AffixFuzzer8Type(BaseExtensionType):
    def __init__(self) -> None:
        pa.ExtensionType.__init__(self, pa.float32(), "rerun.testing.components.AffixFuzzer8")


class AffixFuzzer8Array(BaseExtensionArray[AffixFuzzer8ArrayLike]):
    _EXTENSION_NAME = "rerun.testing.components.AffixFuzzer8"
    _EXTENSION_TYPE = AffixFuzzer8Type

    @staticmethod
    def _native_to_pa_array(data: AffixFuzzer8ArrayLike, data_type: pa.DataType) -> pa.Array:
        raise NotImplementedError  # You need to implement native_to_pa_array_override in affix_fuzzer8_ext.py


AffixFuzzer8Type._ARRAY_TYPE = AffixFuzzer8Array

# TODO(cmc): bring back registration to pyarrow once legacy types are gone
# pa.register_extension_type(AffixFuzzer8Type())