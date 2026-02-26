use std::collections::BTreeMap;
use std::path::PathBuf;
use zellij_tile::prelude::*;

#[derive(Default)]
struct State {
    visible: bool,
    task_id: String,
    ticket: String,
    change_state: String,
    git_branch: String,
    git_commit: String,
    mode_info: ModeInfo,
    cwd: Option<PathBuf>,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        self.visible = true;
        set_selectable(false);
        subscribe(&[
            EventType::Timer,
            EventType::RunCommandResult,
            EventType::ModeUpdate,
            EventType::FileSystemCreate,
            EventType::FileSystemUpdate,
            EventType::PaneUpdate,
        ]);
        set_timeout(10.0);
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::Timer(_) => {
                if self.visible {
                    self.refresh_all();
                }
                set_timeout(10.0);
                false
            },
            Event::PaneUpdate(_) => {
                if self.visible {
                    self.refresh_all();
                }
                false
            },
            Event::FileSystemCreate(paths) | Event::FileSystemUpdate(paths) => {
                if self.visible {
                    let tequila = Self::has_tequila_path(&paths);
                    let git = Self::has_git_path(&paths);
                    if tequila && git {
                        self.refresh_all();
                    } else if tequila {
                        self.refresh_tequila();
                    } else if git {
                        self.refresh_git();
                    }
                }
                false
            },
            Event::RunCommandResult(exit_code, stdout, _stderr, context) => {
                let output = String::from_utf8_lossy(&stdout).trim().to_string();
                let success = exit_code == Some(0) && !output.is_empty();
                let mut changed = false;
                match context.get("type").map(|s| s.as_str()) {
                    Some("work") => {
                        if success {
                            if self.task_id != output {
                                changed = true;
                                self.task_id = output.clone();
                            }
                            self.read_ticket_file(&output);
                            self.read_state_file(&output);
                        } else {
                            changed = self.task_id != "-"
                                || self.ticket != "-"
                                || self.change_state != "-";
                            self.task_id = "-".to_string();
                            self.ticket = "-".to_string();
                            self.change_state = "-".to_string();
                        }
                    },
                    Some("ticket") => {
                        let new_val = if success { output } else { "-".to_string() };
                        if self.ticket != new_val {
                            changed = true;
                            self.ticket = new_val;
                        }
                    },
                    Some("state") => {
                        let new_val = if success { output } else { "-".to_string() };
                        if self.change_state != new_val {
                            changed = true;
                            self.change_state = new_val;
                        }
                    },
                    Some("git_branch") => {
                        let new_val = if success { output } else { "-".to_string() };
                        if self.git_branch != new_val {
                            changed = true;
                            self.git_branch = new_val;
                        }
                    },
                    Some("git_commit") => {
                        let new_val = if success { output } else { "-".to_string() };
                        if self.git_commit != new_val {
                            changed = true;
                            self.git_commit = new_val;
                        }
                    },
                    _ => {},
                }
                changed
            },
            Event::ModeUpdate(mode_info) => {
                if self.mode_info != mode_info {
                    self.mode_info = mode_info;
                    return true;
                }
                false
            },
            _ => false,
        }
    }

    fn pipe(&mut self, pipe_message: PipeMessage) -> bool {
        if pipe_message.name == "toggle" {
            self.visible = !self.visible;
            if self.visible {
                self.refresh_all();
            }
            return true;
        }
        false
    }

    fn render(&mut self, _rows: usize, cols: usize) {
        if !self.visible {
            return;
        }

        // Left side: Task + tequila ribbons
        let zzz_label = " Task";
        let id_text = format!("ID {}", self.task_id);
        let ticket_text = format!("Ticket {}", self.ticket);
        let state_text = format!("State {}", self.change_state);
        let left_width = zzz_label.chars().count()
            + id_text.chars().count() + 4
            + ticket_text.chars().count() + 4
            + state_text.chars().count() + 4;
        let left = format!(
            "{}{}",
            serialize_text(&Text::new(zzz_label).opaque()),
            serialize_ribbon_line(vec![
                Text::new(id_text).color_range(0, 3..),
                Text::new(ticket_text).color_range(0, 7..),
                Text::new(state_text).color_range(0, 6..),
            ]),
        );

        // Right side: Git + Branch/Commit ribbons
        let git_label = " Git";
        let branch_text = format!("Branch {}", self.git_branch);
        let commit_text = format!("Commit {}", self.git_commit);
        let right_width = git_label.chars().count()
            + branch_text.chars().count() + 4
            + commit_text.chars().count() + 4;
        let right = format!(
            "{}{}",
            serialize_text(&Text::new(git_label).opaque()),
            serialize_ribbon_line(vec![
                Text::new(branch_text).color_range(0, 7..),
                Text::new(commit_text).color_range(0, 7..),
            ]),
        );

        // Padding between left and right
        let padding_len = cols.saturating_sub(left_width + right_width);
        let padding = " ".repeat(padding_len);
        let padding = serialize_text(&Text::new(padding).opaque());

        print!("{}{}{}", left, padding, right);
    }
}

impl State {
    fn has_tequila_path(paths: &[(PathBuf, Option<FileMetadata>)]) -> bool {
        paths.iter().any(|(p, _)| {
            let s = p.to_string_lossy();
            s.contains(".tequila/work") || s.contains(".tequila/tasks/")
        })
    }

    fn has_git_path(paths: &[(PathBuf, Option<FileMetadata>)]) -> bool {
        paths.iter().any(|(p, _)| {
            p.to_string_lossy().ends_with(".git/HEAD")
        })
    }

    fn refresh_all(&mut self) {
        self.update_cwd();
        self.read_work_file();
        self.read_git_branch();
        self.read_git_commit();
    }

    fn refresh_tequila(&mut self) {
        self.update_cwd();
        self.read_work_file();
    }

    fn refresh_git(&mut self) {
        self.update_cwd();
        self.read_git_branch();
        self.read_git_commit();
    }

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

    fn read_ticket_file(&self, task_id: &str) {
        let mut ctx = BTreeMap::new();
        ctx.insert("type".to_string(), "ticket".to_string());
        let path = format!(".tequila/tasks/{}/ticket", task_id);
        self.run_cmd(&["cat", &path], ctx);
    }

    fn read_state_file(&self, task_id: &str) {
        let mut ctx = BTreeMap::new();
        ctx.insert("type".to_string(), "state".to_string());
        let path = format!(".tequila/tasks/{}/state", task_id);
        self.run_cmd(&["cat", &path], ctx);
    }

    fn read_git_branch(&self) {
        let mut ctx = BTreeMap::new();
        ctx.insert("type".to_string(), "git_branch".to_string());
        self.run_cmd(&["git", "rev-parse", "--abbrev-ref", "HEAD"], ctx);
    }

    fn read_git_commit(&self) {
        let mut ctx = BTreeMap::new();
        ctx.insert("type".to_string(), "git_commit".to_string());
        self.run_cmd(&["git", "rev-parse", "--short", "HEAD"], ctx);
    }
}
