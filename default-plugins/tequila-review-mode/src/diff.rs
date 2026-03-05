#[derive(Clone, Debug)]
pub struct DiffFile {
    pub old_path: String,
    pub new_path: String,
    pub hunks: Vec<DiffHunk>,
}

#[derive(Clone, Debug)]
pub struct DiffHunk {
    pub lines: Vec<DiffLine>,
}

#[derive(Clone, Debug)]
pub struct DiffLine {
    pub kind: LineKind,
    pub content: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LineKind {
    Context,
    Add,
    Delete,
}

#[derive(Clone, Debug)]
pub struct SideBySideFile {
    pub old_path: String,
    pub new_path: String,
    pub lines: Vec<SideBySideLine>,
}

#[derive(Clone, Debug)]
pub struct SideBySideLine {
    pub left_marker: char,
    pub left: String,
    pub right_marker: char,
    pub right: String,
    pub is_file_header: bool,
}

pub fn parse_patch(input: &str) -> Vec<DiffFile> {
    let mut files: Vec<DiffFile> = Vec::new();
    let mut current_file: Option<DiffFile> = None;
    let mut current_hunk: Option<DiffHunk> = None;
    let mut old_path = String::new();

    for line in input.lines() {
        if line.starts_with("--- ") {
            if let Some(ref mut file) = current_file {
                if let Some(hunk) = current_hunk.take() {
                    file.hunks.push(hunk);
                }
                files.push(file.clone());
            }
            old_path = line.strip_prefix("--- ").unwrap_or("").to_string();
            if let Some(stripped) = old_path.strip_prefix("a/") {
                old_path = stripped.to_string();
            }
            current_file = None;
            current_hunk = None;
        } else if line.starts_with("+++ ") {
            let mut new_path = line.strip_prefix("+++ ").unwrap_or("").to_string();
            if let Some(stripped) = new_path.strip_prefix("b/") {
                new_path = stripped.to_string();
            }
            current_file = Some(DiffFile {
                old_path: old_path.clone(),
                new_path,
                hunks: Vec::new(),
            });
        } else if line.starts_with("@@ ") {
            if let Some(ref mut file) = current_file {
                if let Some(hunk) = current_hunk.take() {
                    file.hunks.push(hunk);
                }
            }
            current_hunk = Some(DiffHunk { lines: Vec::new() });
        } else if let Some(ref mut hunk) = current_hunk {
            if line.starts_with('+') {
                hunk.lines.push(DiffLine {
                    kind: LineKind::Add,
                    content: line[1..].to_string(),
                });
            } else if line.starts_with('-') {
                hunk.lines.push(DiffLine {
                    kind: LineKind::Delete,
                    content: line[1..].to_string(),
                });
            } else if line.starts_with(' ') {
                hunk.lines.push(DiffLine {
                    kind: LineKind::Context,
                    content: line[1..].to_string(),
                });
            } else if line == "\\ No newline at end of file" {
                // skip
            } else {
                hunk.lines.push(DiffLine {
                    kind: LineKind::Context,
                    content: line.to_string(),
                });
            }
        }
    }

    if let Some(ref mut file) = current_file {
        if let Some(hunk) = current_hunk.take() {
            file.hunks.push(hunk);
        }
        files.push(file.clone());
    }

    files
}

pub fn to_side_by_side(files: &[DiffFile]) -> Vec<SideBySideFile> {
    files
        .iter()
        .map(|file| {
            let mut lines = Vec::new();

            for hunk in &file.hunks {
                let mut i = 0;
                let hunk_lines = &hunk.lines;
                while i < hunk_lines.len() {
                    match hunk_lines[i].kind {
                        LineKind::Context => {
                            lines.push(SideBySideLine {
                                left_marker: ' ',
                                left: hunk_lines[i].content.clone(),
                                right_marker: ' ',
                                right: hunk_lines[i].content.clone(),
                                is_file_header: false,
                            });
                            i += 1;
                        },
                        LineKind::Delete => {
                            let del_start = i;
                            while i < hunk_lines.len()
                                && hunk_lines[i].kind == LineKind::Delete
                            {
                                i += 1;
                            }
                            let add_start = i;
                            while i < hunk_lines.len()
                                && hunk_lines[i].kind == LineKind::Add
                            {
                                i += 1;
                            }
                            let deletes = &hunk_lines[del_start..add_start];
                            let adds = &hunk_lines[add_start..i];
                            let max_len = deletes.len().max(adds.len());
                            for j in 0..max_len {
                                lines.push(SideBySideLine {
                                    left_marker: if j < deletes.len() {
                                        '-'
                                    } else {
                                        ' '
                                    },
                                    left: deletes
                                        .get(j)
                                        .map(|l| l.content.clone())
                                        .unwrap_or_default(),
                                    right_marker: if j < adds.len() { '+' } else { ' ' },
                                    right: adds
                                        .get(j)
                                        .map(|l| l.content.clone())
                                        .unwrap_or_default(),
                                    is_file_header: false,
                                });
                            }
                        },
                        LineKind::Add => {
                            lines.push(SideBySideLine {
                                left_marker: ' ',
                                left: String::new(),
                                right_marker: '+',
                                right: hunk_lines[i].content.clone(),
                                is_file_header: false,
                            });
                            i += 1;
                        },
                    }
                }
            }

            SideBySideFile {
                old_path: file.old_path.clone(),
                new_path: file.new_path.clone(),
                lines,
            }
        })
        .collect()
}
