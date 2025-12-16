use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::fs::File;
use std::io::{Read, BufReader};

#[derive(Debug)]
#[allow(dead_code)]
pub enum ProcessingError {
    IoError(String),
    AnalysisError(String),
}

#[derive(Debug)]
pub struct FileStats {
    pub word_count: usize,
    pub line_count: usize,
    pub char_frequencies: HashMap<char, usize>,
    pub size_bytes: u64,
}

#[derive(Debug)]
pub struct FileAnalysis {
    pub filename: String,
    pub stats: Option<FileStats>,
    pub errors: Vec<ProcessingError>,
    pub processing_time: Duration,
}

pub fn analyze_file(filepath: String) -> FileAnalysis {
    let start_time = Instant::now();
    let mut errors = Vec::new();
    let filename_clone = filepath.clone();

    let file = match File::open(&filepath) {
        Ok(f) => f,
        Err(e) => {
            errors.push(ProcessingError::IoError(format!("Failed to open file: {}", e)));
            return FileAnalysis {
                filename: filename_clone,
                stats: None,
                errors,
                processing_time: start_time.elapsed(),
            };
        }
    };
    
    let size_bytes = file.metadata().map(|m| m.len()).unwrap_or(0);
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    
    let stats = if let Err(e) = reader.read_to_end(&mut buffer) {
        errors.push(ProcessingError::IoError(format!("Failed to read file: {}", e)));
        None
    } else {
        let contents = String::from_utf8_lossy(&buffer);
        
        let mut word_count = 0;
        let mut line_count = 0;
        let mut char_frequencies = HashMap::new();
        
        for line in contents.lines() {
            line_count += 1;
            word_count += line.split_whitespace().count();
        }

        for c in contents.chars() {
            if c.is_whitespace() { continue; } 
            let lower_c = c.to_lowercase().next().unwrap_or(c);
            *char_frequencies.entry(lower_c).or_insert(0) += 1;
        }

        Some(FileStats {
            word_count,
            line_count,
            char_frequencies,
            size_bytes,
        })
    };

    FileAnalysis {
        filename: filename_clone,
        stats,
        errors,
        processing_time: start_time.elapsed(),
    }
}