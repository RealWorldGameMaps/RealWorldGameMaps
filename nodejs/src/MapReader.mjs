import fs from 'fs';
import MapObject from './MapObject.mjs';

export default class MapReader {

  constructor(mapFilePath) {
    this.HEADER_BYTES = 12;
    this.TILES_PER_PATCH_SIDE = 16;

    this.mapFilePath = mapFilePath;
    this.mapObject = new MapObject();
    this.arrayBuffer = null;
  }

  loadMapFile(callback) {
    fs.readFile(this.mapFilePath, null, (err, data) => {
      if (err) {
        callback(err, null);
      } else {
        this.checkValidHeader(data.buffer, (error) => {
          if (error) {
            callback(error, null);
          } else {
            this.arrayBuffer = data.buffer;
            callback(null, data.buffer);
          }
        });
      }
    });
  }

  checkValidHeader(buffer, callback) {
    const header = this.getHeaderData(buffer);
    if (header.magic === this.mapObject.MAGIC && header.version === this.mapObject.VERSION) {
      fs.stat(this.mapFilePath, (err, stat) => {
        if (err) {
          callback(err);
        } else {
          if (header.dataSize === stat.size - this.HEADER_BYTES) {
            this.mapObject.setDataSize(header.dataSize);
            callback(null);
          } else {
            callback(header);
          }
        }
      });
    } else {
      callback(header);
    }
  }

  getHeaderData(buffer) {
    const dataViewHeader = new DataView(buffer, 0, this.HEADER_BYTES);
    const magic = [
      dataViewHeader.getUint8(0),
      dataViewHeader.getUint8(1),
      dataViewHeader.getUint8(2),
      dataViewHeader.getUint8(3)
    ].map((byte) => (String.fromCharCode(byte))).join('');
    const version = dataViewHeader.getUint32(4, true);
    const dataSize = dataViewHeader.getUint32(8, true);

    return {magic, version, dataSize};
  }

  getMapObject() {
    this.mapObject.setMapSize(this.getMapSize());
    this.mapObject.setHeightMap(this.getHeightMap().heightMap);
    this.mapObject.setTextures(this.getTextureStrings());
    this.mapObject.setTextureMap(this.getTextureMap().textureMap);

    return this.mapObject;
  }

  getMapSize() {
    const dataViewMapSize = new DataView(this.arrayBuffer, this.HEADER_BYTES, 4);
    return dataViewMapSize.getUint32(0, true);
  }

  getMapLength() {
    const mapSize = this.getMapSize();
    return mapSize * this.TILES_PER_PATCH_SIDE + 1;
  }

  getHeightMapBytes() {
    const mapLength = this.getMapLength();
    return mapLength * mapLength * 2;
  }

  getHeightMap() {
    const mapLength = this.getMapLength();
    const heightMapFields = mapLength * mapLength;
    const dataViewHeightMap = new DataView(this.arrayBuffer, this.HEADER_BYTES + 4, heightMapFields * 2);
    let minValue = Infinity;
    let maxValue = 0;
    let offset = 0;
    const heightMap = MapReader.getInitMap(mapLength);
    for (let y = mapLength - 1; y >= 0; y--) {
      for (let x = 0; x < mapLength; x++) {
        const value = dataViewHeightMap.getUint16(offset, true);
        heightMap[y][x] = value;
        offset += 2;
        if (minValue > value) {
          minValue = value;
        }
        if (maxValue < value) {
          maxValue = value;
        }
      }
    }
    return {mapLength, minValue, maxValue, heightMap};
  }

  static getInitMap(mapLength) {
    const rows = new Array(mapLength);
    for (let i = 0; i < mapLength; i++) {
      rows[i] = new Array(mapLength);
    }
    return rows
  }

  getTexturesCountBytesOffset() {
    const heightMapBytes = this.getHeightMapBytes();
    return this.HEADER_BYTES + 4 + heightMapBytes;
  }

  getTextureCount() {
    const texturesCountBytesOffset = this.getTexturesCountBytesOffset();
    const dataViewTexturesCount = new DataView(this.arrayBuffer, texturesCountBytesOffset, 4);
    return dataViewTexturesCount.getUint32(0, true);
  }

  getTextureStringsBytesOffset() {
    const textureCountBytesOffset = this.getTexturesCountBytesOffset();
    return textureCountBytesOffset + 4;
  }

  getStringLength(textureStringsBytesOffset) {
    const dataViewStringLength = new DataView(this.arrayBuffer, textureStringsBytesOffset, 4);
    return dataViewStringLength.getUint32(0, true);
  }

  getTextureStringChar(textureStringsBytesOffset) {
    const dataViewTextureStringChar = new DataView(this.arrayBuffer, textureStringsBytesOffset, 1);
    const asciiCode = dataViewTextureStringChar.getUint8(0);
    return String.fromCharCode(asciiCode);
  }

  getTextureStrings() {
    const texturesCount = this.getTextureCount();
    let textureStringsBytesOffset = this.getTextureStringsBytesOffset();

    const textureStrings = [];
    for (let i = 0; i < texturesCount; i++) {
      const stringLength = this.getStringLength(textureStringsBytesOffset);
      textureStringsBytesOffset += 4;
      let textureString = '';
      for (let j = 0; j < stringLength; j++) {
        textureString += this.getTextureStringChar(textureStringsBytesOffset);
        textureStringsBytesOffset += 1;
      }
      textureStrings.push(textureString);
    }

    return textureStrings;
  }

  getPatchesBytesOffset() {
    const texturesCount = this.getTextureCount();
    let textureStringsBytesOffset = this.getTextureStringsBytesOffset();
    for (let i = 0; i < texturesCount; i++) {
      const stringLength = this.getStringLength(textureStringsBytesOffset);
      textureStringsBytesOffset += 4 + stringLength;
    }
    return textureStringsBytesOffset;
  }

  getTextureTile(dataViewPatches, offset) {
    const texture1 = dataViewPatches.getUint16(offset + 0, true);
    const texture2 = dataViewPatches.getUint16(offset + 2, true);
    const priority = dataViewPatches.getUint32(offset + 4, true);
    return {texture1, texture2, priority};
  }

  getTextureMap() {
    const mapSize = this.getMapSize();
    const patchesBytesOffset = this.getPatchesBytesOffset();
    const dataViewPatches = new DataView(this.arrayBuffer, patchesBytesOffset);
    const textureMap = MapReader.getInitMap(mapSize * this.TILES_PER_PATCH_SIDE);

    let maxValue = 0;
    let offset = 0;
    for (let py = mapSize - 1; py >= 0; py--) {
      for (let px = 0; px < mapSize; px++) {
        for (let ty = this.TILES_PER_PATCH_SIDE - 1; ty >= 0; ty--) {
          for (let tx = 0; tx < this.TILES_PER_PATCH_SIDE; tx++) {
            const textureTile = this.getTextureTile(dataViewPatches, offset);
            offset += 8;
            const x = px * this.TILES_PER_PATCH_SIDE + tx;
            const y = py * this.TILES_PER_PATCH_SIDE + ty;
            textureMap[y][x] = textureTile;
            if (textureTile.texture1 > maxValue) {
              maxValue = textureTile.texture1;
            }
          }
        }
      }
    }
    return {mapSize, textureMap, maxValue};
  }

}