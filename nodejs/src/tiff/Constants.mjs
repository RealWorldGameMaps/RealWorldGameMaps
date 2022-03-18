export const FILE_EXTENSIONS = {
  TIFF: ['tif', 'tiff'],
  BIG_TIFF: ['tif', 'tf8', 'btf'],
};

export const MAX_FILE_SIZE = {
  TIFF: 4294967296, // 2^32
  BIG_TIFF: 18446744073709552000, // 2^64
};

export const BYTE_ORDER = {
  LITTLE_ENDIAN: 'II', // Intel
  BIG_ENDIAN: 'MM', // Motorola
};

export const MAGIC_NUMBER = {
  TIFF: 42,
  BIG_TIFF: 43,
};

export const DATA_FORMAT = {
  U_INT_8: 0, // 'Uint8
  U_INT_16: 1, // 'Uint16
  U_INT_32: 2, // 'Uint32
  U_INT_64: 3, // 'Uint64
  S_INT_8: 4, // 'Int8
  S_INT_16: 5, // 'Int16
  S_INT_32: 6, // 'Int32
  S_INT_64: 7, // 'Int64
  FLOAT_32: 8, // 'Float32
  FLOAT_64: 9, // 'Float64
};

export const DATA_OFFSETS_SIZES = {
  HEADER: {
    OFFSET: 0,
    SIZE: 16, // TIFF=8, BigTIFF=16 (So 16 Byte to support both)
    BYTE_ORDER: {
      SIZE: 2,
    },
    MAGIC_NUMBER: {
      OFFSET: 2,
      DATA_FORMAT: DATA_FORMAT.U_INT_16,
    },
  },
  TIFF: {
    HEADER: {
      IFD_OFFSET: {
        OFFSET: 4,
        DATA_FORMAT: DATA_FORMAT.U_INT_32,
      },
    },
    IFD: {
      TAGS_COUNT: {
        SIZE: 2,
        DATA_FORMAT: DATA_FORMAT.U_INT_16,
      },
      TAG: {
        OFFSET: 2,
        SIZE: 12,
        TAG_ID: {
          OFFSET: 0,
          DATA_FORMAT: DATA_FORMAT.U_INT_16,
        },
        TAG_TYPE: {
          OFFSET: 2,
          DATA_FORMAT: DATA_FORMAT.U_INT_16,
        },
        TAG_COUNT: {
          OFFSET: 4,
          DATA_FORMAT: DATA_FORMAT.U_INT_32,
        },
        TAG_DATA: {
          OFFSET: 8,
          SIZE: 4,
          DATA_FORMAT: DATA_FORMAT.U_INT_32,
        },
      },
      IFD_OFFSET: {
        SIZE: 4,
        DATA_FORMAT: DATA_FORMAT.U_INT_32,
      },
    },
  },
  BIG_TIFF: {
    HEADER: {
      BYTE_SIZE_OFFSET: {
        OFFSET: 4,
        DATA_FORMAT: DATA_FORMAT.U_INT_16,
      },
      ALWAYS_0: {
        OFFSET: 6,
        DATA_FORMAT: DATA_FORMAT.U_INT_16,
      },
      IFD_OFFSET: {
        OFFSET: 8,
        DATA_FORMAT: DATA_FORMAT.U_INT_64,
      },
    },
    IFD: {
      TAGS_COUNT: {
        SIZE: 8,
        DATA_FORMAT: DATA_FORMAT.U_INT_64,
      },
      TAG: {
        OFFSET: 8,
        SIZE: 20,
        TAG_ID: {
          OFFSET: 0,
          DATA_FORMAT: DATA_FORMAT.U_INT_16,
        },
        TAG_TYPE: {
          OFFSET: 2,
          DATA_FORMAT: DATA_FORMAT.U_INT_16,
        },
        TAG_COUNT: {
          OFFSET: 4,
          DATA_FORMAT: DATA_FORMAT.U_INT_64,
        },
        TAG_DATA: {
          OFFSET: 12,
          SIZE: 8,
          DATA_FORMAT: DATA_FORMAT.U_INT_64,
        },
      },
      IFD_OFFSET: {
        SIZE: 8,
        DATA_FORMAT: DATA_FORMAT.U_INT_64,
      },
    },
  },
};

export const IFD_TAG_TYPE = {
  BYTE: 1,
  ASCII: 2,
  SHORT: 3,
  LONG: 4,
  RATIONAL: 5,
  SBYTE: 6,
  UNDEFINED: 7,
  SSHORT: 8,
  SLONG: 9,
  SRATIONAL: 10,
  FLOAT: 11,
  DOUBLE: 12,
  // 13-15
  LONG8: 16, // BigTIFF
  SLONG8: 17, // BigTIFF
  IFD8: 18, // BigTIFF
};

export const IFD_TAG_TYPE_SIZE = {
  BYTE: 1,
  ASCII: 1,
  SHORT: 2,
  LONG: 4,
  RATIONAL: 8,
  SBYTE: 1,
  UNDEFINED: 1,
  SSHORT: 2,
  SLONG: 4,
  SRATIONAL: 8,
  FLOAT: 4,
  DOUBLE: 8,
  // 13-15
  LONG8: 8, // BigTIFF
  SLONG8: 8, // BigTIFF
  IFD8: 8, // BigTIFF
};

// https://www.awaresystems.be/imaging/tiff/tifftags.html
// https://www.loc.gov/preservation/digital/formats/content/tiff_tags.shtml
export const TAG = {
  // 0-253
  T_254_NewSubfileType: 254, // LONG (1)
  T_255_SubfileType: 255, // SHORT (1)
  T_256_ImageWidth: 256, // SHORT,LONG (1)
  T_257_ImageLength: 257, // SHORT,LONG (1)
  T_258_BitsPerSample: 258, // SHORT (T_277_SamplesPerPixel)
  T_259_Compression: 259, // SHORT (1)
  // 260-261
  T_262_PhotometricInterpretation: 262, // SHORT (1)
  T_263_Threshholding: 263, // SHORT (1)
  T_264_CellWidth: 264, // SHORT (1)
  T_265_CellLength: 265, // SHORT (1)
  T_266_FillOrder: 266, // SHORT (1)
  // 267-268
  T_269_DocumentName: 269, // ASCII
  T_270_ImageDescription: 270, // ASCII
  T_271_Make: 271, // ASCII
  T_272_Model: 272, // ASCII
  T_273_StripOffsets: 273, // SHORT,LONG,LONG8 (StripsPerImage || T_277_SamplesPerPixel * StripsPerImage)
  T_274_Orientation: 274, // SHORT (1)
  // 275-276
  T_277_SamplesPerPixel: 277, // SHORT (1)
  T_278_RowsPerStrip: 278, // SHORT,LONG (1)
  T_279_StripByteCounts: 279, // SHORT,LONG,LONG8 (StripsPerImage || T_277_SamplesPerPixel * StripsPerImage)
  T_280_MinSampleValue: 280, // SHORT (T_277_SamplesPerPixel)
  T_281_MaxSampleValue: 281, // SHORT (T_277_SamplesPerPixel)
  T_282_XResolution: 282, // RATIONAL (1)
  T_283_YResolution: 283, // RATIONAL (1)
  T_284_PlanarConfiguration: 284, // SHORT (1)
  T_285_PageName: 285, // ASCII
  T_286_XPosition: 286, // RATIONAL
  T_287_YPosition: 287, // RATIONAL
  T_288_FreeOffsets: 288, // LONG (1)
  T_289_FreeByteCounts: 289, // LONG (1)
  T_290_GrayResponseUnit: 290, // SHORT (1)
  T_291_GrayResponseCurve: 291, // SHORT (2^T_258_BitsPerSample)
  T_292_T4Options: 292, // LONG (1)
  T_293_T6Options: 293, // LONG (1)
  // 294-295
  T_296_ResolutionUnit: 296, // SHORT (1)
  T_297_PageNumber: 297, // SHORT (2)
  // 298-300
  T_301_TransferFunction: 301, // SHORT ((1,T_277_SamplesPerPixel) * 2^T_258_BitsPerSample)
  // 302-304
  T_305_Software: 305, // ASCII
  T_306_DateTime: 306, // ASCII (20)
  // 307-314
  T_315_Artist: 315, // ASCII
  T_316_HostComputer: 316, // ASCII
  T_317_Predictor: 317, // SHORT (1)
  T_318_WhitePoint: 318, // RATIONAL (2)
  T_319_PrimaryChromaticities: 319, // RATIONAL (6)
  T_320_ColorMap: 320, // SHORT (3 * 2^T_258_BitsPerSample)
  T_321_HalftoneHints: 321, // SHORT (2)
  T_322_TileWidth: 322, // SHORT,LONG (1)
  T_323_TileLength: 323, // SHORT,LONG (1)
  T_324_TileOffsets: 324, // LONG,LONG8 (TilesPerImage)
  T_325_TileByteCounts: 325, // SHORT,LONG,LONG8 (TilesPerImage)
  T_326_BadFaxLines: 326, // SHORT,LONG (1)
  T_327_CleanFaxData: 327, // SHORT (1)
  T_328_ConsecutiveBadFaxLines: 328, // SHORT,LONG (1)
  // 329
  T_330_SubIFDs: 330, // LONG,IFD,IFD8 (number of child IFDs) TODO
  // 331
  T_332_InkSet: 332, // SHORT (1)
  T_333_InkNames: 333, // ASCII (total number of characters in all ink name strings, including zeros)
  T_334_NumberOfInks: 334, // SHORT (1)
  // 335
  T_336_DotRange: 336, // SHORT,BYTE (2, 2 * T_334_NumberOfInks)
  T_337_TargetPrinter: 337, // ASCII
  T_338_ExtraSamples: 338, // SHORT?/BYTE? (number of extra components per pixel)
  T_339_SampleFormat: 339, // SHORT (T_277_SamplesPerPixel)
  T_340_SMinSampleValue: 340, // * (T_277_SamplesPerPixel)
  T_341_SMaxSampleValue: 341, // * (T_277_SamplesPerPixel)
  T_342_TransferRange: 342, // SHORT (6)
  T_343_ClipPath: 343, // BYTE (*) TODO
  T_344_XClipPathUnits: 344, // LONG (1)
  T_345_YClipPathUnits: 345, // LONG (1)
  T_346_Indexed: 346, // SHORT (1)
  T_347_JPEGTables: 347, // * (number of bytes in tables datastream) TODO
  // 348-350
  T_351_OPIProxy: 351, // SHORT (1)
  // 352-399
  T_400_GlobalParametersIFD: 400, // LONG,IFD (1) TODO
  T_401_ProfileType: 401, // LONG (1)
  T_402_FaxProfile: 402, // BYTE (1)
  T_403_CodingMethods: 403, // LONG (1)
  T_404_VersionYear: 404, // BYTE (4)
  T_405_ModeNumber: 405, // BYTE (1)
  // 406-432
  T_433_Decode: 433, // SRATIONAL (2 * T_277_SamplesPerPixel)
  T_434_DefaultImageColor: 434, // SHORT (T_277_SamplesPerPixel)
  // 435-511
  T_512_JPEGProc: 512, // SHORT (1)
  T_513_JPEGInterchangeFormat: 513, // LONG (1)
  T_514_JPEGInterchangeFormatLength: 514, // LONG (1)
  T_515_JPEGRestartInterval: 515, // SHORT (1)
  // 516
  T_517_JPEGLosslessPredictors: 517, // SHORT (T_277_SamplesPerPixel)
  T_518_JPEGPointTransforms: 518, // SHORT (T_277_SamplesPerPixel)
  T_519_JPEGQTables: 519, // LONG (T_277_SamplesPerPixel)
  T_520_JPEGDCTables: 520, // LONG (T_277_SamplesPerPixel)
  T_521_JPEGACTables: 521, // LONG (T_277_SamplesPerPixel)
  // 522-528
  T_529_YCbCrCoefficients: 529, // RATIONAL (3)
  T_530_YCbCrSubSampling: 530, // SHORT (2)
  T_531_YCbCrPositioning: 531, // SHORT (1)
  T_532_ReferenceBlackWhite: 532, // RATIONAL (2 * T_277_SamplesPerPixel)
  // 533-558
  T_559_StripRowCounts: 559, // LONG (number of strips) TODO
  // 560-699
  T_700_XMP: 700, // BYTE (*) TODO
  // 701-32780
  T_32781_ImageID: 32781, // ASCII (*) TODO
  // 32782-33431
  T_33432_Copyright: 33432, // ASCII
  // 33433-34731
  T_34732_ImageLayer: 34732, // SHORT,LONG (2)
  // 34733-65535
};

export const TAG_VALUE = {
  T_254_NewSubfileType: {
    V_1_ReducedImage: 1,
    V_2_Page: 2,
    V_4_Mask: 4,
  },
  T_255_SubfileType: {
    V_1_Image: 1,
    V_2_ReducedImage: 2,
    V_3_Page: 3,
  },
  T_259_Compression: {
    V_1_NONE: 1, // Baseline
    V_2_CCITTRLE: 2, // Baseline
    V_3_CCITTFAX3: 3,
    V_4_CCITTFAX4: 4,
    V_5_LZW: 5,
    V_6_OJPEG: 6,
    V_7_JPEG: 7,
    V_8_ADOBE_DEFLATE: 8,
    V_32766_NEXT: 32766,
    V_32771_CCITTRLEW: 32771,
    V_32773_PACKBITS: 32773, // Baseline
    V_32809_THUNDERSCAN: 32809,
    V_32895_IT8CTPAD: 32895,
    V_32896_IT8LW: 32896,
    V_32897_IT8MP: 32897,
    V_32898_IT8BL: 32898,
    V_32908_PIXARFILM: 32908,
    V_32909_PIXARLOG: 32909,
    V_32946_DEFLATE: 32946,
    V_32947_DCS: 32947,
    V_34661_JBIG: 34661,
    V_34676_SGILOG: 34676,
    V_34677_SGILOG24: 34677,
    V_34712_JP2000: 34712,
  },
  T_262_PhotometricInterpretation: {
    V_0_WhiteIsZero: 0,
    V_1_BlackIsZero: 1,
    V_2_RGB: 2,
    V_3_RGBPalette: 3,
    V_4_TransparencyMask: 4,
    V_5_CMYK: 5,
    V_6_YCbCr: 6,
    V_8_CIELab: 8,
  },
  T_263_Threshholding: {
    V_1_BiLevel: 1,
    V_2_HalfTone: 2,
    V_3_ErrorDiffuse: 3,
  },
  T_266_FillOrder: {
    V_1_MSB2LSB: 1,
    V_2_LSB2MSB: 2,
  },
  T_274_Orientation: {
    V_1_TopLeft: 1,
    V_2_TopRight: 2,
    V_3_BottomRight: 3,
    V_4_BottomLeft: 4,
    V_5_LeftTop: 5,
    V_6_RightTop: 6,
    V_7_RightBottom: 7,
    V_8_LeftBottom: 8,
  },
  T_284_PlanarConfiguration: {
    V_1_Chunky: 1,
    V_2_Planar: 2,
  },
  T_296_ResolutionUnit: {
    V_1_None: 1,
    V_2_Inch: 2,
    V_3_Centimeter: 3,
  },
};
