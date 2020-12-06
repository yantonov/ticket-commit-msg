use std::fs;
use std::path::Path;

pub fn read_file(path: &Path) -> Result<Vec<String>, String> {
    let file_content = fs::read_to_string(path)
        .map_err(|_| "Cannot read commit message file".to_string())?;
    let lines: Vec<String> = file_content.lines()
        .into_iter()
        .map(|x| x.to_string())
        .collect();
    Ok(lines)
}

pub fn write_file(path: &Path, lines: &Vec<String>) -> Result<(), String> {
    let eoln = "\n";
    fs::write(path, lines.join(eoln))
        .map_err(|_| "cannot write commit message".to_string())?;
    Ok(())
}