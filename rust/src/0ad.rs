const TILES_PER_PATCH_SIDE: u32 = 16;

struct PMP {
  // Header
  magic: [char; 4], // "PSMP"
  version: u32, // 6
  data_size: u32, // filesize - header_bytes (header_bytes = 12)

  // Data
  map_size: u32, // patches per map side
  heightmap: [u16; (map_size * TILES_PER_PATCH_SIDE + 1).pow(2)], // vertex heights with lines indexed from bottom to top and columns from left to right
  num_terrain_textures: u32,
  terrain_textures: [TerrainTexture; num_terrain_textures],
  patches: [Patch; map_size.pow(2)], // lines indexed from bottom to top, columns from left to right
}

struct Patch {
  tiles: [Tile; TILES_PER_PATCH_SIDE.pow(2)],
}

struct Tile {
  texture1: u16, // index into terrain_textures[]
  texture2: u16, // index, or 0xFFFF for 'none'
  priority: u32, // Used for blending between edges of tiles with different textures. A higher priority is blended on top of a lower priority.
}

struct TerrainTexture {
  name_length: u32,
  name: [char; name_length], // filenames without path and extension
}
