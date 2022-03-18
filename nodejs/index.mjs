import MapReader from './src/MapReader.mjs';
import MapWriter from './src/MapWriter.mjs';
import AsciiMapDrawer from './src/AsciiMapDrawer.mjs';

const MAP_FILES_PATH = '../data/0ad/';

const mapFilePath = MAP_FILES_PATH + 'Sporades Islands (2).pmp';
// const mapFilePath = MAP_FILES_PATH + 'Treasure Islands.pmp';

const mapReader = new MapReader(mapFilePath);
mapReader.loadMapFile((error) => {
  if (error) {
    return console.error(error);
  }

  const mapObject = mapReader.getMapObject();

  const mapWriter = new MapWriter(mapObject);
  mapWriter.writeFile(MAP_FILES_PATH + 'island.pmp', (error) => {
    if (error) {
      return console.error(error);
    }
    const mapReader = new MapReader(MAP_FILES_PATH + 'island.pmp');
    mapReader.loadMapFile((error) => {
      if (error) {
        return console.error(error);
      }
      const mapObject = mapReader.getMapObject();

      const options = {
        // bbox: {left: 100, right: 300, top: 200, bottom: 300}
      };

      const heightMap = mapObject.getHeightMap();
      const asciiMapDrawer1 = new AsciiMapDrawer(heightMap);
      console.log(asciiMapDrawer1.getMap('color-range', options));

      const textureMap = mapObject.getTextureMap();
      const asciiMapDrawer2 = new AsciiMapDrawer(textureMap.map((row) => (row.map((col) => (col.texture1)))));
      console.log(asciiMapDrawer2.getMap('color-range', options));

      console.log(mapObject.getMinMaxHeight());
    });
  });
});
