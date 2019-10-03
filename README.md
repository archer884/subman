# Sub Manager

> Honestly, `subman` is a terrible name for this program, because it doesn't really manage submarines at all. It's intended to manage traffic and mission files. Look, I just write the software; naming is more your problem than mine.

This software is still in progress. At present, it's capable of reading vessel files from override and combining those with a set of "polyfill" records I've written to cover the default vessels. It should, in theory, also be able to read a mission file. Next step is probably to make it read a campaign file.

Objectives:

- Be able to write traffic files on the basis of campaign/mission/traffic file key association.
- Be able to update mission files on the basis of date/region.

This will probably be two different commands, of course. By default, files should be written to the override directory and will overwrite any other files found in the override directory. Optionally, it should also be possible to target any other output directory.
