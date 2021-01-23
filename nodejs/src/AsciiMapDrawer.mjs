import chalk from 'chalk';

const MAX_HUE = 360;
const MAX_SATURATION = 100;
const MAX_VALUE = 100;

export default class AsciiMapDrawer {

  constructor(mapData) {
    this.mapData = mapData;
    this.setBasicData();

    this.setBasicData = this.setBasicData.bind(this);
    this.getMinMax = this.getMinMax.bind(this);
    this.getSize = this.getSize.bind(this);
    this.getMap = this.getMap.bind(this);
    this.buildMap = this.buildMap.bind(this);
    this.mapColorRange = this.mapColorRange.bind(this);
    this.mapAsciiRange = this.mapAsciiRange.bind(this);
    this.mapBlockRange = this.mapBlockRange.bind(this);
    this.mapColorCategory = this.mapColorCategory.bind(this);
    this.getCharFromScaledValuesString = this.getCharFromScaledValuesString.bind(this);
    this.scale = this.scale.bind(this);
  }

  setBasicData() {
    const values = this.mapData.reduce((a, b) => a.concat(b), []);
    this.minValue = values.reduce((min, v) => ((v < min) ? v : min), values[0]);
    this.maxValue = values.reduce((min, v) => ((v > min) ? v : min), values[0]);
    this.size = [this.mapData[0].length, this.mapData.length];
  }

  getMinMax() {
    return [this.minValue, this.maxValue];
  }

  getSize() {
    return this.size;
  }

  getMap(mode = 'value', options = {}) {
    switch (mode) {
      case 'color-range':
        return this.buildMap(this.mapColorRange, options);
      case 'ascii-range':
        return this.buildMap(this.mapAsciiRange, options);
      case 'block-range':
        return this.buildMap(this.mapBlockRange, options);
      case 'color-category':
        return this.buildMap(this.mapColorCategory, options);
      default:
        return this.buildMap((v) => (v), options);
    }
  }

  buildMap(mapFunc, options) {
    const rows = options.bbox ? this.mapData.slice(options.bbox.top, options.bbox.bottom + 1) : this.mapData;
    return rows.map((row, y) => (
      (options.bbox ? row.slice(options.bbox.left, options.bbox.right + 1) : row)
        .map((col, x) => (mapFunc(col, options, [x, y]))).join('')
    )).join('\n');
  }

  mapColorRange(value, options, coor) {
    const hue = (this.scale(value, 0, 300) + 240) % MAX_HUE;
    return chalk.hsv(hue, MAX_SATURATION, MAX_VALUE)('█');
  }

  mapAsciiRange(value, options, coor) {
    const valuesString = '$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,"^`\'. ';
    const char = this.getCharFromScaledValuesString(value, valuesString);
    return chalk.hex('ffffff')(char);
  }

  mapBlockRange(value, options, coor) {
    const valuesString = '█▓▒░ ';
    const char = this.getCharFromScaledValuesString(value, valuesString);
    return chalk.hex('ffffff')(char);
  }

  mapColorCategory(value, options, coor) {
    // const blockRange = '█▓▒░ ';
    // const blockIndex = this.scale(value, 0, blockRange.length - 1);
    // const block = blockRange[blockIndex];
    // return chalk.hex('ffffff')(block);
    // const greyRange = '$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,"^`\'. ';
    // const greyIndex = this.scale(value, 0, greyRange.length - 1);
    // const grey = greyRange[greyIndex];
    const hue = (this.scale(value, 0, 300) + 240) % MAX_HUE;
    return chalk.hsv(hue, MAX_SATURATION, MAX_VALUE)('█');
  }

  getCharFromScaledValuesString(value, valuesString) {
    const asciiIndex = this.scale(value, 0, valuesString.length - 1);
    return valuesString[asciiIndex];
  }

  scale(value, newMin, newMax) {
    return Math.round(newMin + (newMax - newMin) * (value - this.minValue) / (this.maxValue - this.minValue));
  }

}
