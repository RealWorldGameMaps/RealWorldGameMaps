# Scenario file formats
A scenario consists of an XML data file and, optionally, a PMP data file. The XML format is text and may be viewed in any text editor, while the PMP is a proprietary binary format. Most scenarios are created with Atlas scenario editor, which generates one of each file per map.

The PMP files store a map's elevation and terrain texture references for each tile. The XML files store map objects (entities and actors) as well as information for environment (e.g. lighting and water parameters), camera orientation, and various other settings.

## PMP format
The following describes version 6 of the PMP file format with C-like syntax.

Basic types:
- char = 8-bit character
- u32 = 32-bit unsigned int
- u16 = 16-bit unsigned int

All types are stored in little-endian format. Text is always ASCII.

```
PMP {
  // FILE HEADER
  char magic[4]; // == "PSMP"
  u32 version; // == 6
  u32 data_size; // == filesize-12

  // DATA SECTION
  u32 map_size; // number of patches (16x16 tiles) per side

  u16 heightmap[(mapsize*16 + 1)*(mapsize*16 + 1)]; // vertex heights with lines indexed from
                                                    // bottom to top and columns from left to right
  u32 num_terrain_textures;
  String terrain_textures[num_terrain_textures]; // filenames (no path and file ending), e.g. "cliff1"

  Patch patches[mapsize*mapsize]; // lines indexed from bottom to top, columns from left to right
}

Patch {
  Tile tiles[16*16];
}

Tile {
  u16 texture1; // index into terrain_textures[]
  u16 texture2; // index, or 0xFFFF for 'none'
  u32 priority; // Used for blending between edges of tiles with different textures. A higher priority is blended on top of a lower priority.
}

String {
  u32 length;
  char data[length]; // not NUL-terminated
}
```

## Scenario XML format
The following outlines version 5 of the scenario XML format.

Note: this structure is not rigidly enforced (e.g. by a DTD) - many elements are optional, to preserve backward compatibility with older map formats. However, it should be generated in this latest version of the format whenever possible.
```xml
<?xml version="1.0" encoding="UTF-8"?>
<Scenario version="5">
  <Environment>
    <SkySet>rain</SkySet>
    <SunColor r="0.737255" g="0.592157" b="0.27451"/>
    <SunElevation angle="1.25786"/>
    <SunRotation angle="1.71806"/>
    <TerrainAmbientColor r="0.329412" g="0.419608" b="0.501961"/>
    <UnitsAmbientColor r="0.501961" g="0.501961" b="0.501961"/>
    <Fog>
      <FogFactor>0.00510742</FogFactor>
      <FogThickness>0</FogThickness>
      <FogColor r="0.74902" g="0.74902" b="0.74902"/>
    </Fog>
    <Water>
      <WaterBody>
        <Type>lake</Type>
        <Color r="0.737255" g="0.592157" b="0.27451"/>
        <Tint r="0.439216" g="0.247059" b="0.156863"/>
        <Height>21.823</Height>
        <Waviness>4.99023</Waviness>
        <Murkiness>0.899414</Murkiness>
        <WindAngle>1.1781</WindAngle>
      </WaterBody>
    </Water>
    <Postproc>
      <Brightness>-0.0244141</Brightness>
      <Contrast>1.1084</Contrast>
      <Saturation>0.78125</Saturation>
      <Bloom>0.134961</Bloom>
      <PostEffect>hdr</PostEffect>
    </Postproc>
  </Environment>
  <Camera>
    <Position x="215.538" y="91.1563" z="545.103"/>
    <Rotation angle="0"/>
    <Declination angle="0.610865"/>
  </Camera>
  <ScriptSettings><![CDATA[
{
  "CircularMap": true,
  "Description": "Northwest India. Nearby rivers swell with monsoon rains, allowing for only a few treacherous crossings.\n\nThe rivers are heavily forested, while grasslands carpet the surrounding countryside. Watch out for Tigers in the tall grass! Asian elephants are also a common sight.",
  "Keywords": [],
  "LockTeams": false,
  "Name": "Punjab (2)",
  "PlayerData": [
    {
      "Name": "Player 1"
    },
    {
      "Name": "Player 2"
    }
  ],
  "Preview": "punjab_2.png",
  "RevealMap": false,
  "VictoryConditions": [
    "conquest"
  ]
}
]]></ScriptSettings>
  <Entities>
    <Entity uid="13">
      <Template>actor|props/flora/grass_tropic_field_tall.xml</Template>
      <Position x="154.78473" z="589.32166"/>
      <Orientation y="2.35621"/>
    </Entity>
    <Entity uid="15">
      <Template>actor|props/flora/grass_tropic_field_tall.xml</Template>
      <Position x="154.37449" z="535.5359"/>
      <Orientation y="0.90325"/>
    </Entity>
    ...
  </Entities>
  <Paths/>
</Scenario>
```

### Environment
These elements correspond to settings on the environment tab in Atlas. They affect renderer behavior.

### Camera
Currently unused?

There are optional per-player camera settings in the ScriptSettings element.

### ScriptSettings
A special element containing a JSON object. This object gets parsed during game setup to display information about the map such as its name, description, number of players and their civilizations. It's a flexible format which gets updated as we add new map settings.

This data mostly corresponds to the map and player settings tabs in Atlas.

Note: there is intentional symmetry between this data and that used by the random map generator.

### Entities
This is a list of Entity elements that define all the objects on the map. Each Entity consists of:

- unique entity ID of type u32. Players are also considered entities, so these don't necessarily start with 1. 0 is reserved for INVALID_ENTITY.
- entity template filename. For actors, these are a reference to the actor template instead, prefixed with actor|
- player ID which owns the entity, positive integer of type i32. Gaia is reserved as player 0, typically 1-8 are used for other players, -1 is reserved for INVALID_PLAYER.
- world position. Only X and Z translation are currently supported.
- world orientation. Only rotation in radians about the Y-axis is currently supported.

### Paths
Currently unused.
