# Subtasks of *Fix Review-Mode Scroll Offset Clamping*

## Bug Fix

- [x] `001-clamp-scroll-offset-in-render`: Clamp `self.scroll_offset` to `max_scroll` inside `render_diff` so that scroll-up works immediately after reaching the bottom
