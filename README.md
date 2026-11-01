# Serene Seasons Horizon

A configurable four-season calendar prototype for Horizon 26.3.

## Included

- Four seasons with configurable season length
- Sub-season calendar calculations
- HUD calendar and `/season` commands
- Seasonal fertility calculation helper

## Install

Download [SereneSeasons-Horizon-0.1.0.hnmod](SereneSeasons-Horizon-0.1.0.hnmod) and add that .hnmod file through Horizon's native mod manager.

## Compatibility

- Minecraft Java Edition 26.3
- Horizon revision 46ce6898bd72016e061aaec1afc1375b384deac
- Native hook API 2

## Status

This is an early prototype, not a complete recreation of the Fabric mod. It has not been compiled or tested in the Horizon launcher. See the source and manifest in this repository. Horizon native mods are unrestricted native code; review before installing.

## Build

Use Horizon's 
ative/tools/native.py from the official horizon-rust/horizon-mods repository:

`powershell
python native/tools/native.py check .
python native/tools/native.py package .
`

The source project and bundle are provided here. The original Tough As Nails and Serene Seasons JARs are All Rights Reserved and are not included.

