# ModderCli

ModderCli is a command line tool to help you manage your modding projects. I am making this to help me mod sekiro, but it should work for any game so long as you tweak things around.

## Table of Contents

- [ModderCli](#moddercli)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
    - [Why does this exist?](#why-does-this-exist)
  - [Target Workflow](#target-workflow)
    - [What is this?](#what-is-this)
    - [What is this not?](#what-is-this-not)
  - [Installation](#installation)
    - [Current Prerequisites](#current-prerequisites)
    - [How to install](#how-to-install)
  - [Roadmap](#roadmap)

## Introduction

### Why does this exist?

I hate repetetive workflows. In the case of Sekiro Modding the usual experience to make a mod for me is as follows:

1. Get the files you want to mod
2. Create a backup folder
3. Make a backup of the files
4. Unpack the files
5. Make the changes to the files
6. Repack the files
7. Update Yabber Config for tpf if needed, Verify those typos!
8. Make sure you actually repacked that tpf folder you've been editing for the last 30 minutes wondering why the game isn't showing the changes.
9. Avoid the original .dcx.bak files
10. Copy the files to the mods folder
11. Open the game and test the changes
    - If the changes are not good or destroys something
        1. Copy the backup files to the mods folder
    - If the changes are good
        1. Make new folder in the backup folder
        2. Copy the files to the backup folder 
12. Repeat steps 3-7 until you are happy with the changes


After a while, this gets really annoying. So I made a few things to make my life easier.

My first tool was just a .bat that repacked the files and copied them to the mods folder and started the game. This was good, but I still had to manually make backups and unpack the files. But it required manually copying the file names and paths for Yabber.

My second tool was making a CLI tool that could backup files by version and allow restore to a specific version. With that the experience was much better as I had reduced the amount of manual work I had to do.

My third tool was an attempt at making 4K textures with [upscayl-ncnn](https://github.com/upscayl/upscayl-ncnn) that I instantly realized it was a bad idea in general, so instead I made it simply convert images from png to dds using [ImageMagick](https://github.com/ImageMagick/ImageMagick). As a bonus, it can automatically pack and unpack files using [Yabber](https://github.com/JKAnderson/Yabber).

My fourth tool just generated a list of textures to paste into the Yabber config. It only made textures with 0x00 flags, but it was a start.

Now, I decided that having that many tools scattered around wasn't for the best, so I decided to make a single tool that could do all of these things and more and this time share it instead of letting pick dust.

The caveat? Most tools in the DS realm are made in C# or C++, but I am making this in Rust. Why? Because I wanted to learn more about it. I also want to make it cross platform, so that's a plus. A C# rewrite could come later if I feel like it that way this tool could maybe turn in a GUI tool.


## Target Workflow

The final worflow with this tool should be as follows:

1. Initialise a workspace
2. Copy your files to the src folder
3. Unpack the dcx and tpf files with the tool  
4. Do changes, add textures and focus on your work
5. Pack everything in one go even adding the needed in the new textures Yabber config and removing the deleted ones
6. Export and automatically launch the game to test the changes
7. If the changes are good, Save the current state of the src folder to the branch backup

If I manage to do all of that, I will be happy with the tool.
This will be a slow project as I am severely limited on time, but I will try to make it as good as I can.

### What is this?

This is a command line tool to help you manage your modding projects. It is made in Rust soon hopping to add TOKYOOOOOOOO but unlikely.

### What is this not?

This is not a mod manager. This is not a mod installer. This is not a mod loader. This is not a mod. This is modding worklow tool.

## Installation

### Current Prerequisites
- None

### How to install

1. Download the latest release from the releases page
2. Extract the files to a folder
3. Add the folder to your PATH
4. Open a terminal and type `moddercli` to see if it works


## Roadmap

- [x] Initialise a workspace
- [x] Create branches
- [X] Switch branches
  - [X] Switch to a specific branch
  - [X] Save the current state of the src folder after switching
- [x] Save current state of src to current branch
  - [X] Save recursively
  - [X] Save per version
  - [X] Use .ignore file to ignore files
  - [X] Adding ! to file allows to include the file
  - [ ] Avoid Repeating files by keeping a hashcheck of the files and only saving new files (should be a setting) (optional)
- [X] Delete branches
  - [X] Don't delete current branch
- [X] Restore branches
  - [X] Restore to latest version 
  - [X] Restore to a specific version
  - [ ] Restore a specific file to a specific version
- [ ] Unpack/Pack files
  - [ ] Taget specific files types 
  - [ ] Work with [Yabber](https://github.com/JKAnderson/Yabber)
  - [ ] Work with [WitchBND](https://github.com/ividyon/WitchyBND)
  - [ ] Use .workfiles to target what files to unpack/pack
  - [ ] Use .ignore to ignore files
- [ ] Convert Images to DDS using [ImageMagick](https://github.com/ImageMagick/ImageMagick)
- [ ] Generate TPF Configs
  - [ ] Figure out if the flags mean anything or they don't matter 
  - [ ] Generate TPF Configs for textures (with both Yabber and WitchBND)
- [ ] Publish to publish folder
    - [ ] Automatically Pack files into .zip and .7z
- [ ] Export to mods folder
    - [ ] use .targets to decide what files go where in the mods folder (use pattern matching to decide) (optional)
- [ ] Launch game
- [ ] On first launch, Do a config walkthrough to set up the paths for the extra software that is required to be used
- [ ] Better Documentation