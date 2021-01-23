export default class MapObject {

  constructor() {
    this.MAGIC = 'PSMP';
    this.VERSION = 6;
    this.dataSize = null;
    this.mapSize = 0;
    this.heightMap = [[]];
    this.textures = [];
    this.textureMap = [[]];

    this.MAX_HEIGHT_RANGE = 712;
    this.seaLevel = 0;
    this.heightRange = [0, this.MAX_HEIGHT_RANGE];

    this.TERRAIN_TILE_WIDTH_METERS = 4;
    this.HEIGHT_UNITS_PER_METRE = 92;
    this.MAP_TILES_BOARDER_WIDTH = 3;
  }

  calcDataSize() {
    const mapSizeBytes = 4;
    const heightMapLength = this.mapSize * 16 + 1;
    const heightMapBytes = heightMapLength * heightMapLength * 2;
    const texturesCountBytes = 4;
    const texturesBytes = this.textures.map((texture) => (4 + texture.length)).reduce((acc, value) => (acc + value), 0);
    const textureMapLength = this.mapSize * 16;
    const textureMapBytes = textureMapLength * textureMapLength * 8;
    return mapSizeBytes + heightMapBytes + texturesCountBytes + texturesBytes + textureMapBytes;
  }

  getDataSize() {
    return this.dataSize;
  }
  setDataSize(dataSize) {
    this.dataSize = dataSize;
  }

  getMapSize() {
    return this.mapSize;
  }
  setMapSize(mapSize) {
    this.mapSize = mapSize;
  }

  getHeightMap() {
    return this.heightMap;
  }
  setHeightMap(heightMap) {
    this.heightMap = heightMap;
  }

  getTextures() {
    return this.textures;
  }
  setTextures(textures) {
    this.textures = textures;
  }

  getTextureMap() {
    return this.textureMap;
  }
  setTextureMap(textureMap) {
    this.textureMap = textureMap;
  }

  getSeaLevel() {
    return this.seaLevel;
  }
  setSeaLevel(seaLevel) {
    this.seaLevel = seaLevel;
    this.heightRange = [0 - seaLevel, this.MAX_HEIGHT_RANGE - seaLevel];
  }

  getMinMaxHeight() {
    const minMaxHeight = [Infinity, -Infinity];

    this.heightMap.forEach((row) => (
      row.forEach((col) => {
        minMaxHeight[0] = Math.min(minMaxHeight[0], col);
        minMaxHeight[1] = Math.max(minMaxHeight[1], col);
      })
    ));

    return minMaxHeight.map((x) => (x / this.HEIGHT_UNITS_PER_METRE));
  }

}