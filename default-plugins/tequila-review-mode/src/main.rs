mod diff;

use std::collections::BTreeMap;
use std::path::PathBuf;
use zellij_tile::prelude::*;

#[derive(Default)]
struct State {
    task_id: String,
    subtasks: Vec<SubtaskInfo>,
    current_index: usize,
    scroll_offset: usize,
    loading: bool,
    no_patches: bool,
    cwd: Option<PathBuf>,
    resize_width_steps: usize,
    resize_up_steps: usize,
}

#[derive(Clone, Default)]
struct SubtaskInfo {
    name: String,
    patch_content: String,
    description: String,
    approved: bool,
    patch_loaded: bool,
    description_loaded: bool,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        self.resize_width_steps = configuration
            .get("resize_width_steps")
            .and_then(|v| v.parse().ok())
            .unwrap_or(15);
        self.resize_up_steps = configuration
            .get("resize_up_steps")
            .and_then(|v| v.parse().ok())
            .unwrap_or(5);
        request_permission(&[
            PermissionType::RunCommands,
            PermissionType::ChangeApplicationState,
            PermissionType::ReadApplicationState,
        ]);
        subscribe(&[
            EventType::Key,
            EventType::RunCommandResult,
            EventType::PermissionRequestResult,
        ]);
        set_self_mouse_selection_support(true);
        self.loading = true;
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::PermissionRequestResult(_) => {
                let plugin_id = get_plugin_ids().plugin_id;
                rename_plugin_pane(plugin_id, "Tequila - Subtask Review Mode");
                // Expand floating pane: width fully, height partially
                for _ in 0..self.resize_width_steps {
                    resize_focused_pane_with_direction(Resize::Increase, Direction::Left);
                    resize_focused_pane_with_direction(Resize::Increase, Direction::Right);
                }
                for _ in 0..self.resize_up_steps {
                    resize_focused_pane_with_direction(Resize::Increase, Direction::Up);
                }
                self.update_cwd();
                self.read_work_file();
                false
            },
            Event::RunCommandResult(exit_code, stdout, _stderr, context) => {
                self.handle_command_result(exit_code, stdout, context)
            },
            Event::Key(key) => self.handle_key(key),
            Event::Mouse(_) => false,
            _ => false,
        }
    }

    fn render(&mut self, rows: usize, cols: usize) {
        if self.loading {
            let msg = "Loading...";
            let x = cols.saturating_sub(msg.len()) / 2;
            let y = rows / 2;
            print!("\x1b[{};{}H{}", y + 1, x + 1, msg);
            return;
        }
        if self.no_patches || self.subtasks.is_empty() {
            let msg = "NO PATCHES TO REVIEW";
            let x = cols.saturating_sub(msg.len()) / 2;
            let y = rows / 2;
            print!("\x1b[{};{}H\x1b[1m{}\x1b[0m", y + 1, x + 1, msg);
            return;
        }
        self.render_diff(rows, cols);
    }
}

// --- Data loading (subtask 002) ---

impl State {
    fn update_cwd(&mut self) {
        if let Ok((_tab_index, pane_id)) = get_focused_pane_info() {
            if let Ok(cwd) = get_pane_cwd(pane_id) {
                self.cwd = Some(cwd);
            }
        }
    }

    fn run_cmd(&self, cmd: &[&str], context: BTreeMap<String, String>) {
        if let Some(cwd) = &self.cwd {
            run_command_with_env_variables_and_cwd(
                cmd,
                BTreeMap::new(),
                cwd.clone(),
                context,
            );
        } else {
            run_command(cmd, context);
        }
    }

    fn read_work_file(&self) {
        let mut ctx = BTreeMap::new();
        ctx.insert("type".to_string(), "work".to_string());
        self.run_cmd(&["cat", ".tequila/work"], ctx);
    }

    fn list_subtasks(&self) {
        let mut ctx = BTreeMap::new();
        ctx.insert("type".to_string(), "ls-subtasks".to_string());
        let path = format!(".tequila/tasks/{}/subtasks", self.task_id);
        self.run_cmd(&["ls", &path], ctx);
    }

    fn read_patch_file(&self, subtask_name: &str) {
        let mut ctx = BTreeMap::new();
        ctx.insert("type".to_string(), "read-patch".to_string());
        ctx.insert("name".to_string(), subtask_name.to_string());
        let path = format!(
            ".tequila/tasks/{}/subtasks/{}/patch",
            self.task_id, subtask_name
        );
        self.run_cmd(&["cat", &path], ctx);
    }

    fn read_description_file(&self, subtask_name: &str) {
        let mut ctx = BTreeMap::new();
        ctx.insert("type".to_string(), "read-description".to_string());
        ctx.insert("name".to_string(), subtask_name.to_string());
        let path = format!(
            ".tequila/tasks/{}/subtasks/{}/description",
            self.task_id, subtask_name
        );
        self.run_cmd(&["cat", &path], ctx);
    }

    fn read_subtask_state(&self, subtask_name: &str) {
        let mut ctx = BTreeMap::new();
        ctx.insert("type".to_string(), "read-state".to_string());
        ctx.insert("name".to_string(), subtask_name.to_string());
        let path = format!(
            ".tequila/tasks/{}/subtasks/{}/state",
            self.task_id, subtask_name
        );
        self.run_cmd(&["cat", &path], ctx);
    }

    fn handle_command_result(
        &mut self,
        exit_code: Option<i32>,
        stdout: Vec<u8>,
        context: BTreeMap<String, String>,
    ) -> bool {
        let output = String::from_utf8_lossy(&stdout).trim().to_string();
        let success = exit_code == Some(0) && !output.is_empty();

        match context.get("type").map(|s| s.as_str()) {
            Some("work") => {
                if success {
                    self.task_id = output.clone();
                    self.list_subtasks();
                } else {
                    self.loading = false;
                    self.no_patches = true;
                    return true;
                }
                false
            },
            Some("ls-subtasks") => {
                if success {
                    let mut dirs: Vec<String> = output
                        .lines()
                        .filter(|l| !l.is_empty())
                        .map(|l| l.to_string())
                        .collect();
                    dirs.sort();
                    self.subtasks = dirs
                        .into_iter()
                        .map(|name| SubtaskInfo {
                            name,
                            ..Default::default()
                        })
                        .collect();
                    if self.subtasks.is_empty() {
                        self.loading = false;
                        self.no_patches = true;
                        return true;
                    }
                    for subtask in &self.subtasks {
                        self.read_patch_file(&subtask.name);
                        self.read_description_file(&subtask.name);
                        self.read_subtask_state(&subtask.name);
                    }
                } else {
                    self.loading = false;
                    self.no_patches = true;
                    return true;
                }
                false
            },
            Some("read-patch") => {
                if let Some(name) = context.get("name") {
                    if let Some(subtask) = self.subtasks.iter_mut().find(|s| &s.name == name) {
                        subtask.patch_loaded = true;
                        if success {
                            subtask.patch_content = output;
                        }
                    }
                    if self.all_files_loaded() {
                        self.finish_loading();
                        return true;
                    }
                }
                false
            },
            Some("read-description") => {
                if let Some(name) = context.get("name") {
                    if let Some(subtask) = self.subtasks.iter_mut().find(|s| &s.name == name) {
                        subtask.description_loaded = true;
                        if success {
                            subtask.description = output;
                        }
                    }
                    if self.all_files_loaded() {
                        self.finish_loading();
                        return true;
                    }
                }
                false
            },
            Some("read-state") => {
                if let Some(name) = context.get("name") {
                    if let Some(subtask) = self.subtasks.iter_mut().find(|s| &s.name == name) {
                        subtask.approved = success && output == "APPROVED";
                    }
                }
                true
            },
            Some("write-approve") => {
                if exit_code == Some(0) {
                    if let Some(name) = context.get("name") {
                        if let Some(subtask) =
                            self.subtasks.iter_mut().find(|s| &s.name == name)
                        {
                            subtask.approved = true;
                        }
                    }
                }
                true
            },
            Some("write-cancel") => {
                if exit_code == Some(0) {
                    if let Some(name) = context.get("name") {
                        if let Some(subtask) =
                            self.subtasks.iter_mut().find(|s| &s.name == name)
                        {
                            subtask.approved = false;
                        }
                    }
                }
                true
            },
            _ => false,
        }
    }

    fn all_files_loaded(&self) -> bool {
        self.subtasks.iter().all(|s| s.patch_loaded && s.description_loaded)
    }

    fn finish_loading(&mut self) {
        self.subtasks.retain(|s| !s.patch_content.is_empty());
        self.loading = false;
        if self.subtasks.is_empty() {
            self.no_patches = true;
        }
    }
}

// --- Keybinding actions (subtask 005) ---

impl State {
    fn approve_current(&self) {
        if let Some(subtask) = self.subtasks.get(self.current_index) {
            let mut ctx = BTreeMap::new();
            ctx.insert("type".to_string(), "write-approve".to_string());
            ctx.insert("name".to_string(), subtask.name.clone());
            let path = format!(
                ".tequila/tasks/{}/subtasks/{}/state",
                self.task_id, subtask.name
            );
            let cmd_str = format!("echo APPROVED > {}", path);
            self.run_cmd(&["sh", "-c", &cmd_str], ctx);
        }
    }

    fn cancel_approve_current(&self) {
        if let Some(subtask) = self.subtasks.get(self.current_index) {
            let mut ctx = BTreeMap::new();
            ctx.insert("type".to_string(), "write-cancel".to_string());
            ctx.insert("name".to_string(), subtask.name.clone());
            let path = format!(
                ".tequila/tasks/{}/subtasks/{}/state",
                self.task_id, subtask.name
            );
            let cmd_str = format!("rm -f {}", path);
            self.run_cmd(&["sh", "-c", &cmd_str], ctx);
        }
    }

    fn handle_key(&mut self, key: KeyWithModifier) -> bool {
        if self.no_patches || self.loading || self.subtasks.is_empty() {
            if key.bare_key == BareKey::Char('q') && key.has_no_modifiers() {
                close_self();
            }
            return false;
        }
        match key.bare_key {
            BareKey::Char('a') if key.has_no_modifiers() => {
                self.approve_current();
                false
            },
            BareKey::Char('c') if key.has_no_modifiers() => {
                self.cancel_approve_current();
                false
            },
            BareKey::Left if key.has_no_modifiers() => {
                if self.current_index > 0 {
                    self.current_index -= 1;
                } else {
                    self.current_index = self.subtasks.len().saturating_sub(1);
                }
                self.scroll_offset = 0;
                true
            },
            BareKey::Right if key.has_no_modifiers() => {
                self.current_index += 1;
                if self.current_index >= self.subtasks.len() {
                    self.current_index = 0;
                }
                self.scroll_offset = 0;
                true
            },
            BareKey::Up if key.has_no_modifiers() => {
                self.scroll_offset = self.scroll_offset.saturating_sub(1);
                true
            },
            BareKey::Down if key.has_no_modifiers() => {
                self.scroll_offset += 1;
                true
            },
            BareKey::PageUp if key.has_no_modifiers() => {
                self.scroll_offset = self.scroll_offset.saturating_sub(10);
                true
            },
            BareKey::PageDown if key.has_no_modifiers() => {
                self.scroll_offset += 10;
                true
            },
            BareKey::Char('q') if key.has_no_modifiers() => {
                close_self();
                false
            },
            _ => false,
        }
    }
}

// --- Renderer (subtask 004) ---

impl State {
    fn render_diff(&self, rows: usize, cols: usize) {
        let subtask = &self.subtasks[self.current_index];
        let parsed = diff::parse_patch(&subtask.patch_content);
        let side_by_side = diff::to_side_by_side(&parsed);

        // Row 1: Header bar (inverted, status right-aligned)
        let left_part = format!(
            " \x1b[1m{}\x1b[22m  ({}/{})",
            subtask.name,
            self.current_index + 1,
            self.subtasks.len(),
        );
        let (status_label, status_visible_len) = if subtask.approved {
            ("\x1b[42;30m APPROVED \x1b[0m", 10)
        } else {
            ("\x1b[43;30m PENDING \x1b[0m", 9)
        };
        let left_visible_len = strip_ansi_len(&left_part);
        let gap = cols.saturating_sub(left_visible_len + status_visible_len);
        let header = format!(
            "\x1b[7m{}{}{}\x1b[0m",
            left_part,
            " ".repeat(gap),
            status_label,
        );
        print!("\x1b[1;1H{}", header);

        // Collect all renderable lines
        let mut lines: Vec<diff::SideBySideLine> = Vec::new();
        for file in &side_by_side {
            lines.push(diff::SideBySideLine {
                left_marker: ' ',
                left: format!("--- {}", file.old_path),
                right_marker: ' ',
                right: format!("+++ {}", file.new_path),
                is_file_header: true,
            });
            lines.extend(file.lines.iter().cloned());
        }

        let visible_rows = rows.saturating_sub(3);
        let max_scroll = lines.len().saturating_sub(visible_rows);
        let scroll = self.scroll_offset.min(max_scroll);

        let half_width = cols.saturating_sub(3) / 2;
        let end = lines.len().min(scroll + visible_rows);
        let visible_lines = &lines[scroll..end];

        for (i, line) in visible_lines.iter().enumerate() {
            let row = i + 2;
            print!("\x1b[{};1H\x1b[K", row);

            if line.is_file_header {
                let left = truncate_or_pad(&line.left, half_width);
                let right = truncate_or_pad(&line.right, half_width);
                print!(
                    "\x1b[1m{}\x1b[0m \u{2502} \x1b[1m{}\x1b[0m",
                    left, right
                );
            } else {
                let left_content =
                    truncate_or_pad(&line.left, half_width.saturating_sub(1));
                let right_content =
                    truncate_or_pad(&line.right, half_width.saturating_sub(1));

                match line.left_marker {
                    '-' => print!("\x1b[31m-{}\x1b[0m", left_content),
                    _ => print!(" {}", left_content),
                }
                print!(" \u{2502} ");
                match line.right_marker {
                    '+' => print!("\x1b[32m+{}\x1b[0m", right_content),
                    _ => print!(" {}", right_content),
                }
            }
        }

        // Clear remaining lines
        for i in visible_lines.len()..visible_rows {
            let row = i + 2;
            print!("\x1b[{};1H\x1b[K", row);
        }

        // Footer (inverted)
        let footer =
            " [a] Approve  [c] Cancel  [\u{2190}/\u{2192}] Prev/Next  [\u{2191}/\u{2193}] Scroll  [q] Quit";
        print!(
            "\x1b[{};1H\x1b[7m{}\x1b[0m",
            rows,
            pad_right(footer, cols)
        );
    }
}

fn truncate_or_pad(s: &str, width: usize) -> String {
    let char_count = s.chars().count();
    if char_count > width {
        s.chars().take(width).collect()
    } else {
        format!("{}{}", s, " ".repeat(width - char_count))
    }
}

fn pad_right(s: &str, width: usize) -> String {
    let visible_len = strip_ansi_len(s);
    if visible_len >= width {
        s.to_string()
    } else {
        format!("{}{}", s, " ".repeat(width - visible_len))
    }
}

fn strip_ansi_len(s: &str) -> usize {
    let mut len = 0;
    let mut in_escape = false;
    for ch in s.chars() {
        if in_escape {
            if ch.is_ascii_alphabetic() {
                in_escape = false;
            }
        } else if ch == '\x1b' {
            in_escape = true;
        } else {
            len += 1;
        }
    }
    len
}
