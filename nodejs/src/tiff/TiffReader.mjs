import fs from 'fs';
import {MAX_FILE_SIZE, BYTE_ORDER, MAGIC_NUMBER, DATA_FORMAT, DATA_OFFSETS_SIZES, IFD_TAG_TYPE, IFD_TAG_TYPE_SIZE, TAG, TAG_VALUE} from './Constants';
import chalk from 'chalk';

const CONST = {
  FIRST_IFD_ENTRY: 0,
  IFD_NO_MORE_IFDS: 0,
  BYTE_SIZE_OFFSET: 8,
  ALWAYS_0: 0,
  NULL_BYTE: 0,
};

export default class TiffReader {

  constructor(filePath) {
    this.filePath = filePath;
    this.fd = null;
    this.byteOrder = null;
    this.isLittleEndian = null;
    this.magicNumber = null;
    this.isBigTiff = null;
    this.ifdOffsets = [];
    this.ifdEntries = [];
    this.DOS = null;
  }

  openFile() {
    this.fd = fs.openSync(this.filePath);
  }

  closeFile() {
    fs.closeSync(this.fd);
  }

  readHeader() {
    const dataView = this.getDataView(DATA_OFFSETS_SIZES.HEADER.OFFSET, DATA_OFFSETS_SIZES.HEADER.SIZE);

    this.byteOrder = this.getAsciiFromDataView(dataView, DATA_OFFSETS_SIZES.HEADER.BYTE_ORDER.SIZE);
    this.isLittleEndian = this.byteOrder === BYTE_ORDER.LITTLE_ENDIAN;
    this.magicNumber = this.parseData(dataView, DATA_OFFSETS_SIZES.HEADER.MAGIC_NUMBER.DATA_FORMAT, DATA_OFFSETS_SIZES.HEADER.MAGIC_NUMBER.OFFSET);
    this.isBigTiff = this.magicNumber === MAGIC_NUMBER.BIG_TIFF;
    this.DOS = this.isBigTiff ? DATA_OFFSETS_SIZES.BIG_TIFF : DATA_OFFSETS_SIZES.TIFF;
    if (this.isBigTiff) {
      const byteSizeOffset = this.parseData(dataView, this.DOS.HEADER.BYTE_SIZE_OFFSET.DATA_FORMAT, this.DOS.HEADER.BYTE_SIZE_OFFSET.OFFSET);
      const always0 = this.parseData(dataView, this.DOS.HEADER.ALWAYS_0.DATA_FORMAT, this.DOS.HEADER.ALWAYS_0.OFFSET);
      if (byteSizeOffset !== CONST.BYTE_SIZE_OFFSET || always0 !== CONST.ALWAYS_0) {
        throw new Error('Unsupported BigTIFF-Format');
      }
    }
    this.ifdOffsets[CONST.FIRST_IFD_ENTRY] = this.parseData(dataView, this.DOS.HEADER.IFD_OFFSET.DATA_FORMAT, this.DOS.HEADER.IFD_OFFSET.OFFSET);
  }

  readMainIfd() {
    const ifdOffset = this.ifdOffsets[CONST.FIRST_IFD_ENTRY];
    const ifdTagsCount = this.getIfdTagsCount(ifdOffset);
    this.ifdEntries[CONST.FIRST_IFD_ENTRY] = this.readIfdTags(ifdOffset, ifdTagsCount);
  }

  readAllIfds() {
    let ifdNumber = 0;

    while (true) {
      let ifdOffset = this.ifdOffsets[ifdNumber];

      const ifdTagsCount = this.getIfdTagsCount(ifdOffset);
      const nextIfdOffset = this.getNextIfdOffset(ifdOffset + this.DOS.IFD.TAG.OFFSET + this.DOS.IFD.TAG.SIZE * ifdTagsCount);

      this.ifdEntries[ifdNumber] = this.readIfdTags(ifdOffset, ifdTagsCount);

      if (nextIfdOffset === CONST.IFD_NO_MORE_IFDS) {
        break;
      }

      ++ifdNumber;
      this.ifdOffsets[ifdNumber] = nextIfdOffset;
    }
  }

  getIfdTagsCount(ifdOffset) {
    const dataView = this.getDataView(ifdOffset, this.DOS.IFD.TAGS_COUNT.SIZE);
    return this.parseData(dataView, this.DOS.IFD.TAGS_COUNT.DATA_FORMAT);
  }

  getNextIfdOffset(ifdNextIfdOffsetOffset) {
    const dataView = this.getDataView(ifdNextIfdOffsetOffset, this.DOS.IFD.IFD_OFFSET.SIZE);
    return this.parseData(dataView, this.DOS.IFD.IFD_OFFSET.DATA_FORMAT);
  }

  readIfdTags(ifdOffset, ifdTagsCount) {
    const dataView = this.getDataView(ifdOffset + this.DOS.IFD.TAG.OFFSET, this.DOS.IFD.TAG.SIZE * ifdTagsCount);

    const ifdTags = {};
    for (let ifdTagNumber = 0; ifdTagNumber < ifdTagsCount; ifdTagNumber++) {
      const ifdTag = this.readIfdTag(dataView.buffer, ifdTagNumber);
      ifdTags[ifdTag.id] = ifdTag.value;
    }

    return ifdTags;
  }

  readIfdTag(buffer, ifdTagNumber) {
    const ifdTagDataView = new DataView(buffer, this.DOS.IFD.TAG.SIZE * ifdTagNumber, this.DOS.IFD.TAG.SIZE);

    const tagId = this.parseData(ifdTagDataView, this.DOS.IFD.TAG.TAG_ID.DATA_FORMAT, this.DOS.IFD.TAG.TAG_ID.OFFSET);
    const tagType = this.parseData(ifdTagDataView, this.DOS.IFD.TAG.TAG_TYPE.DATA_FORMAT, this.DOS.IFD.TAG.TAG_TYPE.OFFSET);
    const tagCount = this.parseData(ifdTagDataView, this.DOS.IFD.TAG.TAG_COUNT.DATA_FORMAT, this.DOS.IFD.TAG.TAG_COUNT.OFFSET);
    const tagDataDataView = new DataView(ifdTagDataView.buffer, this.DOS.IFD.TAG.SIZE * ifdTagNumber + this.DOS.IFD.TAG.TAG_DATA.OFFSET, this.DOS.IFD.TAG.TAG_DATA.SIZE);

    return {
      id: tagId,
      value: this.getIfdTagValue(tagType, tagCount, tagDataDataView)
    };
  }

  getIfdTagValue(tagType, tagCount, tagDataDataView) {
    const size = TiffReader.getSizeOfTagType(tagType);
    const dataView = this.getTagDataDataView(tagDataDataView, tagCount * size);

    switch (tagType) {
      case IFD_TAG_TYPE.BYTE:
        return this.parseStandardTagValue(dataView, tagCount, size, DATA_FORMAT.U_INT_8);
      case IFD_TAG_TYPE.ASCII:
        return this.parseAsciiTagValue(dataView, tagCount, size);
      case IFD_TAG_TYPE.SHORT:
        return this.parseStandardTagValue(dataView, tagCount, size, DATA_FORMAT.U_INT_16);
      case IFD_TAG_TYPE.LONG:
        return this.parseStandardTagValue(dataView, tagCount, size, DATA_FORMAT.U_INT_32);
      case IFD_TAG_TYPE.RATIONAL:
        return this.parseRationalTagValue(dataView, tagCount, size, DATA_FORMAT.U_INT_32);
      case IFD_TAG_TYPE.SBYTE:
        return this.parseStandardTagValue(dataView, tagCount, size, DATA_FORMAT.S_INT_8);
      case IFD_TAG_TYPE.UNDEFINED:
        return this.parseStandardTagValue(dataView, tagCount, size, DATA_FORMAT.U_INT_8);
      case IFD_TAG_TYPE.SSHORT:
        return this.parseStandardTagValue(dataView, tagCount, size, DATA_FORMAT.S_INT_16);
      case IFD_TAG_TYPE.SLONG:
        return this.parseStandardTagValue(dataView, tagCount, size, DATA_FORMAT.S_INT_32);
      case IFD_TAG_TYPE.SRATIONAL:
        return this.parseRationalTagValue(dataView, tagCount, size, DATA_FORMAT.S_INT_32);
      case IFD_TAG_TYPE.FLOAT:
        return this.parseStandardTagValue(dataView, tagCount, size, DATA_FORMAT.FLOAT_32);
      case IFD_TAG_TYPE.DOUBLE:
        return this.parseStandardTagValue(dataView, tagCount, size, DATA_FORMAT.FLOAT_64);
      // 13-15
      case IFD_TAG_TYPE.LONG8:
        return this.parseStandardTagValue(dataView, tagCount, size, DATA_FORMAT.U_INT_64);
      case IFD_TAG_TYPE.SLONG8:
        return this.parseStandardTagValue(dataView, tagCount, size, DATA_FORMAT.S_INT_64);
      case IFD_TAG_TYPE.IFD8:
        return this.parseStandardTagValue(dataView, tagCount, size, DATA_FORMAT.U_INT_64);
      default:
        console.warn('Unsupported Tag-Type');
        return null;
    }
  }

  static getSizeOfTagType(tagType) {
    switch (tagType) {
      case IFD_TAG_TYPE.BYTE:
        return IFD_TAG_TYPE_SIZE.BYTE;
      case IFD_TAG_TYPE.ASCII:
        return IFD_TAG_TYPE_SIZE.ASCII;
      case IFD_TAG_TYPE.SHORT:
        return IFD_TAG_TYPE_SIZE.SHORT;
      case IFD_TAG_TYPE.LONG:
        return IFD_TAG_TYPE_SIZE.LONG;
      case IFD_TAG_TYPE.RATIONAL:
        return IFD_TAG_TYPE_SIZE.RATIONAL;
      case IFD_TAG_TYPE.SBYTE:
        return IFD_TAG_TYPE_SIZE.SBYTE;
      case IFD_TAG_TYPE.UNDEFINED:
        return IFD_TAG_TYPE_SIZE.UNDEFINED;
      case IFD_TAG_TYPE.SSHORT:
        return IFD_TAG_TYPE_SIZE.SSHORT;
      case IFD_TAG_TYPE.SLONG:
        return IFD_TAG_TYPE_SIZE.SLONG;
      case IFD_TAG_TYPE.SRATIONAL:
        return IFD_TAG_TYPE_SIZE.SRATIONAL;
      case IFD_TAG_TYPE.FLOAT:
        return IFD_TAG_TYPE_SIZE.FLOAT;
      case IFD_TAG_TYPE.DOUBLE:
        return IFD_TAG_TYPE_SIZE.DOUBLE;
      // 13-15
      case IFD_TAG_TYPE.LONG8:
        return IFD_TAG_TYPE_SIZE.LONG8;
      case IFD_TAG_TYPE.SLONG8:
        return IFD_TAG_TYPE_SIZE.SLONG8;
      case IFD_TAG_TYPE.IFD8:
        return IFD_TAG_TYPE_SIZE.IFD8;
      default:
        console.warn('Unsupported Tag-Type');
    }
  }

  getTagDataDataView(dataView, dataSize) {
    if (dataSize > this.DOS.IFD.TAG.TAG_DATA.SIZE) {
      const offset = this.parseData(dataView, this.DOS.IFD.TAG.TAG_DATA.DATA_FORMAT);
      return this.getDataView(offset, dataSize);
    }
    return dataView;
  }

  parseStandardTagValue(dataView, tagCount, size, dataFormat) {
    const ifdTagValue = new Array(tagCount);
    for (let i = 0; i < tagCount; i++) {
      ifdTagValue[i] = this.parseData(dataView, dataFormat, size * i);
    }
    return ifdTagValue;
  }
  
  parseAsciiTagValue(dataView, tagCount, size) {
    const allAsciiCharCodes = this.parseStandardTagValue(dataView, tagCount, size, DATA_FORMAT.U_INT_8);

    const asciiCharCodesArray = [];
    let asciiCharCodesStartIndex = 0;
    let asciiCharCodesNullByteIndex = allAsciiCharCodes.indexOf(CONST.NULL_BYTE);
    while (asciiCharCodesNullByteIndex > -1) {
      asciiCharCodesArray.push(allAsciiCharCodes.slice(asciiCharCodesStartIndex, asciiCharCodesNullByteIndex + 1));
      asciiCharCodesStartIndex = asciiCharCodesNullByteIndex + 1;
      asciiCharCodesNullByteIndex = allAsciiCharCodes.indexOf(CONST.NULL_BYTE, asciiCharCodesStartIndex);
    }

    return asciiCharCodesArray.map((asciiCharCodes) => (TiffReader.getAsciiFromCharCodes(asciiCharCodes)));
  }

  parseRationalTagValue(dataView, tagCount, size, dataFormat) {
    const ifdTagValue = new Array(tagCount);
    for (let i = 0; i < tagCount; i++) {
      const numerator = this.parseData(dataView, dataFormat, size * i);
      const denominator = this.parseData(dataView, dataFormat, size * i + size / 2);
      ifdTagValue[i] = numerator / denominator;
    }
    return ifdTagValue;
  }

  parseData(dataView, dataFormat, offset = 0, isLittleEndian = this.isLittleEndian) {
    switch (dataFormat) {
      case DATA_FORMAT.U_INT_8:
        return dataView.getUint8(offset);
      case DATA_FORMAT.U_INT_16:
        return dataView.getUint16(offset, isLittleEndian);
      case DATA_FORMAT.U_INT_32:
        return dataView.getUint32(offset, isLittleEndian);
      case DATA_FORMAT.U_INT_64: // https://windows-hexerror.linestarve.com/q/so53103695-How-to-Read-64-bit-Integer-from-an-ArrayBuffer-DataView-in-JavaScript
        return Number(dataView.getBigUint64(offset, isLittleEndian));
      case DATA_FORMAT.S_INT_8:
        return dataView.getInt8(offset);
      case DATA_FORMAT.S_INT_16:
        return dataView.getInt16(offset, isLittleEndian);
      case DATA_FORMAT.S_INT_32:
        return dataView.getInt32(offset, isLittleEndian);
      case DATA_FORMAT.S_INT_64: // https://windows-hexerror.linestarve.com/q/so53103695-How-to-Read-64-bit-Integer-from-an-ArrayBuffer-DataView-in-JavaScript
        return Number(dataView.getBigInt64(offset, isLittleEndian));
      case DATA_FORMAT.FLOAT_32:
        return dataView.getFloat32(offset, isLittleEndian);
      case DATA_FORMAT.FLOAT_64:
        return dataView.getFloat64(offset, isLittleEndian);
      default:
        throw new Error('Invalid Data-Format');
    }
  }

  getAsciiFromDataView(dataView, length) {
    const asciiCharCodes = [];
    for (let byteOffset = 0; byteOffset < length; byteOffset++) {
      asciiCharCodes.push(this.parseData(dataView, DATA_FORMAT.U_INT_8, byteOffset));
    }
    return TiffReader.getAsciiFromCharCodes(asciiCharCodes, false);
  }

  static getAsciiFromCharCodes(asciiCharCodes, checkNullByte = true) {
    if (checkNullByte) {
      const nullByte = asciiCharCodes.pop();
      if (nullByte !== CONST.NULL_BYTE) {
        throw new Error('Invalid ASCII-Data');
      }
    }
    return asciiCharCodes.map((asciiCharCode) => (String.fromCharCode(asciiCharCode))).join('');
  }

  getDataView(offset, length) {
    const dataView = new DataView(new ArrayBuffer(length));
    fs.readSync(this.fd, dataView, 0, dataView.byteLength, offset);
    return dataView;
  }

  getImageMetaData(imageNumber = 0) {
    const ifdEntry = this.ifdEntries[imageNumber];

    const metaData = {
      "ImageWidth": ifdEntry[TAG.T_256_ImageWidth][0],
      "ImageHeight": ifdEntry[TAG.T_257_ImageLength][0],
    };
    if (ifdEntry[TAG.T_270_ImageDescription]) {
      metaData["ImageDescription"] = ifdEntry[TAG.T_270_ImageDescription][0];
    }
    if (ifdEntry[TAG.T_271_Make]) {
      metaData["Maker"] = ifdEntry[TAG.T_271_Make][0];
    }
    if (ifdEntry[TAG.T_272_Model]) {
      metaData["Model"] = ifdEntry[TAG.T_272_Model][0];
    }
    if (ifdEntry[TAG.T_280_MinSampleValue]) {
      metaData["MinSampleValue"] = ifdEntry[TAG.T_280_MinSampleValue][0];
    }
    if (ifdEntry[TAG.T_281_MaxSampleValue]) {
      metaData["MaxSampleValue"] = ifdEntry[TAG.T_281_MaxSampleValue][0];
    }
    if (ifdEntry[TAG.T_282_XResolution]) {
      metaData["XResolution"] = ifdEntry[TAG.T_282_XResolution][0];
    }
    if (ifdEntry[TAG.T_283_YResolution]) {
      metaData["YResolution"] = ifdEntry[TAG.T_283_YResolution][0];
    }
    if (ifdEntry[TAG.T_296_ResolutionUnit] && ifdEntry[TAG.T_296_ResolutionUnit][0] !== TAG_VALUE.T_296_ResolutionUnit.V_1_None) {
      metaData["ResolutionUnit"] = (ifdEntry[TAG.T_296_ResolutionUnit][0] === TAG_VALUE.T_296_ResolutionUnit.V_3_Centimeter) ? 'cm' : 'in';
    }
    if (ifdEntry[TAG.T_305_Software]) {
      metaData["Software"] = ifdEntry[TAG.T_305_Software][0];
    }
    if (ifdEntry[TAG.T_306_DateTime]) {
      metaData["DateTime"] = ifdEntry[TAG.T_306_DateTime][0];
    }
    if (ifdEntry[TAG.T_315_Artist]) {
      metaData["Artist"] = ifdEntry[TAG.T_315_Artist][0];
    }
    if (ifdEntry[TAG.T_316_HostComputer]) {
      metaData["HostComputer"] = ifdEntry[TAG.T_316_HostComputer][0];
    }
    if (ifdEntry[TAG.T_33432_Copyright]) {
      metaData["Copyright"] = ifdEntry[TAG.T_33432_Copyright][0];
    }

    return metaData;
  }

  getImage(imageNumber = 0) {
    const ifdEntry = this.ifdEntries[imageNumber];

    const imageWidth = ifdEntry[TAG.T_256_ImageWidth][0];
    const imageHeight = ifdEntry[TAG.T_257_ImageLength][0];

    const rowsPerStrip = ifdEntry[TAG.T_278_RowsPerStrip] ? ifdEntry[TAG.T_278_RowsPerStrip][0] : imageHeight;
    // const rowsPerStrip = ifdEntry[TAG.T_278_RowsPerStrip] ? Math.min(ifdEntry[TAG.T_278_RowsPerStrip][0], imageHeight) : imageHeight; // TODO necessary?
    const stripOffsets = ifdEntry[TAG.T_273_StripOffsets]; // array-length of "stripsPerImage" or "T_277_SamplesPerPixel * stripsPerImage"
    const stripByteCounts = ifdEntry[TAG.T_279_StripByteCounts]; // array-length of "stripsPerImage" or "T_277_SamplesPerPixel * stripsPerImage"

    const stripsPerImage = Math.ceil(imageHeight / rowsPerStrip);
    // const stripsPerImage = Math.floor((imageHeight + rowsPerStrip - 1) / rowsPerStrip); // TODO check: https://www.awaresystems.be/imaging/tiff/tifftags/rowsperstrip.html

    const samplesPerPixel = ifdEntry[TAG.T_277_SamplesPerPixel] ? ifdEntry[TAG.T_277_SamplesPerPixel][0] : 1;

    const planarConfiguration = (ifdEntry[TAG.T_284_PlanarConfiguration] && samplesPerPixel !== 1) ? ifdEntry[TAG.T_284_PlanarConfiguration][0] : TAG_VALUE.T_284_PlanarConfiguration.V_1_Chunky;

    const bitsPerSample = ifdEntry[TAG.T_258_BitsPerSample] || new Array(samplesPerPixel).fill(1); // array-length of "samplesPerPixel"

    const threshholding = ifdEntry[TAG.T_263_Threshholding] ? ifdEntry[TAG.T_263_Threshholding][0] : TAG_VALUE.T_263_Threshholding.V_1_BiLevel;
    const fillOrder = ifdEntry[TAG.T_266_FillOrder] ? ifdEntry[TAG.T_266_FillOrder][0] : TAG_VALUE.T_266_FillOrder.V_1_MSB2LSB;

    const compression = ifdEntry[TAG.T_259_Compression] ? ifdEntry[TAG.T_259_Compression][0] : TAG_VALUE.T_259_Compression.V_1_NONE;

    console.log({
      imageWidth,
      imageHeight,
      rowsPerStrip,
      stripOffsets,
      stripByteCounts,
      stripsPerImage,
      samplesPerPixel,
      planarConfiguration,
      bitsPerSample,
      threshholding,
      fillOrder,
      compression
    });

    stripOffsets.forEach((stripOffset) => {
      const dataView = this.getDataView(stripOffset, stripByteCounts[0]);
      const data = [];
      let x = 0;
      let y = 0;
      for (let i = 0; i < dataView.byteLength; i += samplesPerPixel) {
        if (!data[y]) {
          data[y] = [];
        }
        const samples = new Array(samplesPerPixel).fill(0);
        data[y][x] = samples.map((sample, index) => (dataView.getUint8(i + index)));

        if (x === imageWidth - 1) {
          x = 0;
          y++;
        } else {
          x++;
        }
      }
      this.renderImageToConsole(data);
    });

    // console.log([imageWidth, imageHeight], rowsPerStrip, stripOffsets, stripByteCounts, stripsPerImage, samplesPerPixel, bitsPerSample, threshholding, fillOrder, compression);

    // const bitsPerPixel = ifdEntry[TAG.T_258_BitsPerSample]
    //   ? Math.min(32, ifdEntry[TAG.T_258_BitsPerSample][0]) * ifdEntry[TAG.T_258_BitsPerSample].length
    //   : ifdEntry[TAG.T_277_SamplesPerPixel] ? ifdEntry[TAG.T_277_SamplesPerPixel][0] : 1;
    // const bitsPerPixelLine = Math.ceil(ifdEntry[TAG.T_256_ImageWidth] * bitsPerPixel / 8) * 8;
    //
    // const bytes = new Uint8Array(ifdEntry[TAG.T_257_ImageLength] * (bitsPerPixelLine>>>3));
    // let bilen = 0;
    //
    // for (let i = 0; i < soff.length; i++) {
    //   UTIF.decode._decompress(img, ifds, data, soff[i], bcnt[i], cmpr, bytes, Math.ceil(bilen/8)|0, fo);
    //   bilen += bipl * rowsPerStrip;
    // }
    // bilen = Math.min(bilen, bytes.length * 8);
    //
    // const data = new Uint8Array(bytes.buffer, 0, Math.ceil(bilen / 8) | 0);
    // console.log(bitsPerPixel, bitsPerPixelLine, bytes, bilen, data);
  }

  renderImageToConsole(data) {
    console.log(
      data.map((y) => (
        y.map((x) => (
          chalk.rgb(x[0], x[1], x[2])('█')
        )).join('')
      )).join('\n')
    );
  }

}
