---
title: "Bevy Alchemy"
date: 2025-12-26
description: "An experimental, status effects-as-entities system for Bevy."
extra:
  source: "https://github.com/AlephCubed/bevy_alchemy"
---

There are many ways to achieve status effects in an ECS.
Like most ECS problems, the best solutions often leverage the power of the ECS itself.

Bevy Alchemy is my attempt at an ergonomic status effects-as-entities system, based on my work
with [Immediate Stats](../immediate-stats).

## The Problem

At its core, status effects-as-entities can be achieved with just a
simple [relationship](https://docs.rs/bevy/latest/bevy/ecs/relationship/).
This works perfectly fine for effects that should stack. Multiple instances can exist at once, and each is processed
separately.

But in many cases, an effect should only be applied once per target.

## My Solution

The solution I ended up using was to
use [custom commands](https://bevy-cheatbook.github.io/programming/commands.html#custom-commands)
to process and uphold effect invariants.

This behaviour can be controlled using the `MergeMode` enum component.

| Mode   | Behaviour                                                                               |
|--------|-----------------------------------------------------------------------------------------|
| Stack  | Multiple of the same effect can exist at once.                                          |
| Insert | New applications will overwrite the existing one.                                       |
| Merge  | New applications are merged with the existing one, using a configurable merge function. |

## Current Status

There are many details that still need to be sorted out, and this project hasn't been tested in a large scale project,
but overall, I am happy with the direction it is heading.

It feels good to actually be releasing projects recently.
I guess smaller scoped projects are easier to finish? Who could have guessed! 