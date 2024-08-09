# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/components/image_buffer.fbs".

# You can extend this class by creating a "ImageBufferExt" class in "image_buffer_ext.py".

from __future__ import annotations

from .. import datatypes
from .._baseclasses import (
    ComponentBatchMixin,
    ComponentMixin,
)

__all__ = ["ImageBuffer", "ImageBufferBatch", "ImageBufferType"]


class ImageBuffer(datatypes.Blob, ComponentMixin):
    """
    **Component**: A buffer that is known to store image data.

    To interpret the contents of this buffer, see, [`components.ImageFormat`][rerun.components.ImageFormat].
    """

    _BATCH_TYPE = None
    # You can define your own __init__ function as a member of ImageBufferExt in image_buffer_ext.py

    # Note: there are no fields here because ImageBuffer delegates to datatypes.Blob
    pass


class ImageBufferType(datatypes.BlobType):
    _TYPE_NAME: str = "rerun.components.ImageBuffer"


class ImageBufferBatch(datatypes.BlobBatch, ComponentBatchMixin):
    _ARROW_TYPE = ImageBufferType()


# This is patched in late to avoid circular dependencies.
ImageBuffer._BATCH_TYPE = ImageBufferBatch  # type: ignore[assignment]
