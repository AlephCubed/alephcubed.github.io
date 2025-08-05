---
title: "Immediate Stats"
date: 2025-03-23
description: "A Rust crate for game stats that reset every frame, inspired by immediate mode GUI."
source: "https://github.com/AlephCubed/immediate_stats"
---

This is my attempt at solving the issue of game stat tracking for a Bevy project
I was working on. I wanted a way to add temporary bonuses, including
multipliers, without either

1. requiring separate setup/teardown logic,
2. nor making reading stats more complex.

[Immediate mode GUI](https://en.wikipedia.org/wiki/Immediate_mode_(computer_graphics)#Immediate_mode_GUI)
seemed to offer a solution.

# Similar Projects

After starting on this project, I have come across two other libraries that seem
to be solving the same problem,
[Bevy Stat Query](https://github.com/mintlu8/bevy_stat_query) and
[Bevy Gauge](https://github.com/DEMIURGE-studio/bevy_gauge). I haven't looked
into either in detail, but both of these seem much more advanced/mature.
