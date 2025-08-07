---
title: "PhotonBS"
date: 2025-05-25
description: "A Beatsaber map editor with support for V3/group lighting."
---

Made in Rust using the Bevy game engine.

I will expand on this page eventually, but for now I post updates and showcases
on [Bluesky](https://bsky.app/profile/alephcubed.com).

## Past Attempts

This is actually my third attempt at making a V3 beatmap editor.

My first attempt was a naive implementation made in Godot using GDScript. This
did not go well. The performance was so bad that it couldn't even load an entire
map without crashing.

The second attempt was my first time using Rust, Bevy, and the ECS paradigm.
This attempt performed much better; however, because of my lack of knowledge, it
was poorly structured.

My current (and hopefully final) attempt also uses Rust and Bevy, but this time
with a much more experienced developer. Originally, this used the same parsing
library I wrote for my second attempt, but that has since been replaced with my
new crate, [BSRU](https://github.com/AlephCubed/BSRU).
