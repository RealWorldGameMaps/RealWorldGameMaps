import TiffReader from './src/tiff/TiffReader';
import fs from 'fs';

const TIFF_FILES_PATH = '../data/tiff/';

// const filePath = TIFF_FILES_PATH + 'small.tif';
const filePath = TIFF_FILES_PATH + 'BigTIFF/Classic.tif';
// const filePath = TIFF_FILES_PATH + 'BigTIFF/BigTIFF.tif';

const benchmark = (worker, iterations = 1) => {
  const start = process.hrtime.bigint();

  for (let i = 0; i < iterations; i++) {
    worker();
  }

  const end = process.hrtime.bigint();
  console.log(`Duration: ${Number(end - start) / 1000000} ms`);
};

const stats = fs.statSync(filePath);
console.log(`File Size: ${stats.size / 1000000} MB`);

benchmark(() => {
  const tiffReader = new TiffReader(filePath);
  tiffReader.openFile();
  tiffReader.readHeader();
  // tiffReader.readMainIfd();
  tiffReader.readAllIfds();
  tiffReader.getImage();
  // console.log(JSON.stringify(tiffReader.getImageMetaData(), null, 2));
  // console.log(tiffReader);
  // console.log(tiffReader.ifdOffsets);
  console.log(tiffReader.ifdEntries);
});


// import Utif from 'utif';
// // import chalk from 'chalk';
//
// benchmark(() => {
//   const data = fs.readFileSync(filePath);
//   const ifds = Utif.decode(data.buffer);
//   // ifds.forEach((ifd) => {
//   Utif.decodeImage(data.buffer, ifds);
//   // const rgba = Utif.toRGBA8(ifd);
//   // console.log(ifds);
//
//   // const arr = Array.prototype.slice.call(ifd.data);
//   // const greys = [];
//   // for (let i = 0; i < arr.length; i += 4) {
//   //   const bytes = [arr[i], arr[i+1], arr[i+2], arr[i+3]];
//   //   const uInt8Bytes = Uint8Array.from(bytes);
//   //   const dataView = new DataView(uInt8Bytes.buffer);
//   //   const int32le = dataView.getInt32(0, true);
//   //   greys.push(int32le);
//   // }
//   // const min = Math.min(...greys);
//   // const max = Math.max(...greys);
//   // let i = 0;
//   // const rows = [];
//   // for (let y = 0; y < ifd.height; y++) {
//   //   rows[y] = [];
//   //   for (let x = 0; x < ifd.width; x++) {
//   //     // rows[y][x] = scale(greys[i], min, max, 0, 255);
//   //     rows[y][x] = (scale(greys[i], min, max, 0, 300) + 240) % 360;
//   //     i++;
//   //   }
//   // }
//   // console.log(rows.map((cols) => (cols.map((v) => (chalk.rgb(v, v, v)('█'))).join(''))).join('\n'));
//   // const minimizer = 4;
//   // console.log(rows.filter((row, rIndex) => (rIndex % minimizer === 0)).map((cols) => (cols.filter((col, cIndex) => (cIndex % minimizer === 0)).map((v) => (chalk.hsv(v, 100, 100)('█'))).join(''))).join('\n'));
//   // console.log(arr, [ifd.width, ifd.height], i, arr.length);
//   // });
// });

// const scale = (value, oldMin, oldMax, newMin, newMax) => (Math.round(newMin + (newMax - newMin) * (value - oldMin) / (oldMax - oldMin)));
