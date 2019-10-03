# Sub Manager

> Honestly, `subman` is a terrible name for this program, because it doesn't really manage submarines at all. It's intended to manage traffic and mission files. Look, I just write the software; naming is more your problem than mine.

This software is still in progress. At present, it's capable of reading vessel files from override and combining those with a set of "polyfill" records I've written to cover the default vessels. It should, in theory, also be able to read a mission file. Next step is probably to make it read a campaign file.

## Objectives

- Be able to write traffic files on the basis of campaign/mission/traffic file key association.
- Be able to update mission files on the basis of date/region.

This will probably be two different commands, of course. By default, files should be written to the override directory and will overwrite any other files found in the override directory. Optionally, it should also be possible to target any other output directory.

Long term, the intent is not to provide a "cheat" for the game, but to provide a better balance of "challenge" and "difficulty." Instead of scrolling through dozens of ships that you know damn well *cannot* appear in any mission in your current campaign (how many times have you encountered USS *Nimitz* in the Norwegian Sea while driving a 688?), this program should allow you to customize your traffic files such that ships which exist in a given location and at a given time will appear in your sonar listing. Can you encounter an Akula in the 1968 campaign? No; they hadn't invented it yet, so let's not put it in the list.

Primarily, this should be a tool for mod authors to more easily incorporate their ships into existing missions and new campaigns. That said, there's no reason individual users can't use the same program to fine tune their own setup.

## Contributing

This program is being written in [Rust](https://www.rust-lang.org/). The biggest reason for that is just that Rust is a language I write my hobby stuff in, but it's also nice in that it is 1) cross-platform, 2) strongly-typed, and 3) produces a single, easily-downloaded executable file (as opposed to, say, C# with tons of dll files).

If you're interested in helping, that's awesome! Check out [rustup](https://rustup.rs/) to install a toolchain on your computer (note that windows users will either need to use the GNU toolchain or have a version of Visual Studio with C++ installed) and have at it.

**If programming is not your bag, I'd still appreciate your thoughts.** Specifically, the file [poly.rs](https://github.com/archer884/subman/blob/master/src/cw/poly.rs) contains metadata for the default ships and submarines in the game. I've assigned each one a "year" on the basis of 1) a little bit of research and 2) which traffic files they appear in, but I welcome feedback on the years chosen for each. The same goes for the the campaign regions listed for each one.

> `CampaignRegion` should tell the program where the vessel can appear in either enemy or neutral traffic. For example, the 688 can appear in the China Sea region if you are playing as PLAN, or in the Norwegian Sea region if you are playing as Warsaw Pact.
