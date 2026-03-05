# Proposal of *Fix Review-Mode Scroll Offset Clamping*

## Motivation

In the tequila-review-mode plugin, after scrolling to the bottom of the diff view, pressing Up or PageUp does not scroll back up. The view stays stuck at the bottom until the user presses Up enough times for `scroll_offset` to drop below `max_scroll`.

**Root cause:** In `render_diff` (`main.rs:458`), `scroll_offset` is clamped via `.min(max_scroll)` into a local variable `scroll` for rendering, but `self.scroll_offset` is never updated to the clamped value. When Down/PageDown increases `scroll_offset` far beyond `max_scroll`, pressing Up subtracts 1 but `scroll_offset` remains above `max_scroll`, so the rendered view stays stuck at the bottom.

## Summary

- Change `render_diff` to take `&mut self` instead of `&self`
- After computing the clamped scroll value (`let scroll = self.scroll_offset.min(max_scroll)`), write it back: `self.scroll_offset = scroll`
- This ensures `scroll_offset` is always in valid range, so Up/PageUp respond immediately after reaching the bottom

## Impact

- Affected code: `default-plugins/tequila-review-mode/src/main.rs` (`render_diff` method and `render` caller)
