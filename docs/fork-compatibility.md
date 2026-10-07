# fork compatibility
(authored by agents unless marked 🧑)

- tracks upstream mdBook 0.5.4
- fork package names avoid collisions with upstream crates
  - builds `mdbook_fork4ls`; release archives expose it as `mdbook` for guide compatibility
  - guide deployment reads the fork package version
- thin mutable chapter traversal supports parallel chapter processing
  - borrowed chapter fields remain disjoint across nested chapters
- preprocessors and renderers retain thread-safe trait bounds
