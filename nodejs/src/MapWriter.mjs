import fs from 'fs';

export default class MapWriter {

  constructor(mapObject) {
    this.HEADER_BYTES = 12;
    this.TILES_PER_PATCH_SIDE = 16;

    this.mapObject = mapObject;
    this.arrayBuffer = null;
  }

  setBuffer() {
    this.initBuffer();
    this.setHeaderData();
    this.setMapSize();
    this.setHeightMap();
    this.setTexturesCount();
    this.setTextures();
    this.setTextureMap();
  }

  initBuffer() {
    const length = this.HEADER_BYTES + this.mapObject.getDataSize();
    this.arrayBuffer = new ArrayBuffer(length);
  }

  setHeaderData() {
    const dataViewHeader = new DataView(this.arrayBuffer, 0, this.HEADER_BYTES);
    dataViewHeader.setUint8(0, this.mapObject.MAGIC.charCodeAt(0));
    dataViewHeader.setUint8(1, this.mapObject.MAGIC.charCodeAt(1));
    dataViewHeader.setUint8(2, this.mapObject.MAGIC.charCodeAt(2));
    dataViewHeader.setUint8(3, this.mapObject.MAGIC.charCodeAt(3));
    dataViewHeader.setUint32(4, this.mapObject.VERSION, true);
    dataViewHeader.setUint32(8, this.mapObject.getDataSize(), true);
  }

  setMapSize() {
    const dataViewMapSize = new DataView(this.arrayBuffer, this.HEADER_BYTES, 4);
    dataViewMapSize.setUint32(0, this.mapObject.getMapSize(), true);
  }

  getHeightMapBytes() {
    const mapSize = this.mapObject.getMapSize();
    const mapLength = mapSize * this.TILES_PER_PATCH_SIDE + 1;
    return mapLength * mapLength * 2;
  }

  setHeightMap() {
    const mapLength = this.mapObject.getMapSize() * this.TILES_PER_PATCH_SIDE + 1;
    const heightMap = this.mapObject.getHeightMap();
    const dataViewHeightMap = new DataView(this.arrayBuffer, this.HEADER_BYTES + 4, this.getHeightMapBytes());
    let offset = 0;
    for (let y = mapLength - 1; y >= 0; y--) {
      for (let x = 0; x < mapLength; x++) {
        const value = heightMap[y][x];
        dataViewHeightMap.setUint16(offset, value, true);
        offset += 2;
      }
    }
  }

  setTexturesCount() {
    const texturesCountBytesOffset = this.HEADER_BYTES + 4 + this.getHeightMapBytes();
    const dataViewTexturesCount = new DataView(this.arrayBuffer, texturesCountBytesOffset, 4);
    return dataViewTexturesCount.setUint32(0, this.mapObject.getTextures().length, true);
  }

  setTextures() {
    let texturesBytesOffset = this.HEADER_BYTES + 4 + this.getHeightMapBytes() + 4;
    const textures = this.mapObject.getTextures();
    textures.forEach((texture) => {
      const dataViewTexture = new DataView(this.arrayBuffer, texturesBytesOffset, 4 + texture.length);
      dataViewTexture.setUint32(0, texture.length, true);
      const textureChars = texture.split('');
      textureChars.forEach((textureChar, index) => (
        dataViewTexture.setUint8(4 + index, textureChar.charCodeAt(0))
      ));
      texturesBytesOffset += 4 + texture.length;
    });
  }

  getTextureMapBytesOffset() {
    const texturesBytesOffset = this.HEADER_BYTES + 4 + this.getHeightMapBytes() + 4;
    const texturesBytes = this.mapObject.getTextures().map((texture) => (4 + texture.length)).reduce((acc, value) => (acc + value), 0);
    return texturesBytesOffset + texturesBytes;
  }

  setTextureMap() {
    const mapSize = this.mapObject.getMapSize();
    const textureMap = this.mapObject.getTextureMap();
    const dataViewPatches = new DataView(this.arrayBuffer, this.getTextureMapBytesOffset());
    let offset = 0;
    for (let py = mapSize - 1; py >= 0; py--) {
      for (let px = 0; px < mapSize; px++) {
        for (let ty = this.TILES_PER_PATCH_SIDE - 1; ty >= 0; ty--) {
          for (let tx = 0; tx < this.TILES_PER_PATCH_SIDE; tx++) {
            const x = px * this.TILES_PER_PATCH_SIDE + tx;
            const y = py * this.TILES_PER_PATCH_SIDE + ty;
            const textureTile = textureMap[y][x];
            dataViewPatches.setUint16(offset + 0, textureTile.texture1, true);
            dataViewPatches.setUint16(offset + 2, textureTile.texture2, true);
            dataViewPatches.setUint32(offset + 4, textureTile.priority, true);
            offset += 8;
          }
        }
      }
    }
  }

  writeFile(mapFilePath, callback) {
    this.setBuffer();
    fs.writeFile(mapFilePath, Buffer.from(this.arrayBuffer), (err) => {
      callback(err);
    });
  }

}