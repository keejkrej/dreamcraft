use dreamcraft_core::{DreamError, Id, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CellCoord {
    pub col: u32,
    pub row: u32,
}

impl CellCoord {
    pub fn new(col: u32, row: u32) -> Self {
        Self { col, row }
    }

    /// Convert "A1" -> CellCoord(0, 0), "Z26" -> CellCoord(25, 25), "AA1" -> CellCoord(26, 0)
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim().to_uppercase();
        let letters: String = s.chars().take_while(|c| c.is_ascii_alphabetic()).collect();
        let digits: String = s.chars().skip_while(|c| c.is_ascii_alphabetic()).collect();

        if letters.is_empty() || digits.is_empty() {
            return None;
        }

        let mut col: u32 = 0;
        for c in letters.chars() {
            col = col * 26 + (c as u32 - 'A' as u32 + 1);
        }
        col = col.saturating_sub(1);

        let row: u32 = digits.parse::<u32>().ok()?.saturating_sub(1);
        Some(Self { col, row })
    }

    /// Convert CellCoord(0, 0) -> "A1", CellCoord(26, 0) -> "AA1"
    pub fn to_string_coord(&self) -> String {
        let mut col = self.col + 1;
        let mut letters = String::new();
        while col > 0 {
            let rem = (col - 1) % 26;
            letters.insert(0, (b'A' + rem as u8) as char);
            col = (col - 1) / 26;
        }
        format!("{}{}", letters, self.row + 1)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CellValue {
    Empty,
    Number(f64),
    Text(String),
    Boolean(bool),
    Formula(String),
}

impl CellValue {
    pub fn as_number(&self) -> Option<f64> {
        match self {
            CellValue::Number(n) => Some(*n),
            CellValue::Text(s) => s.trim().parse::<f64>().ok(),
            CellValue::Boolean(b) => Some(if *b { 1.0 } else { 0.0 }),
            _ => None,
        }
    }

    pub fn display_string(&self) -> String {
        match self {
            CellValue::Empty => String::new(),
            CellValue::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    format!("{:.2}", n)
                }
            }
            CellValue::Text(s) => s.clone(),
            CellValue::Boolean(b) => if *b { "TRUE".into() } else { "FALSE".into() },
            CellValue::Formula(f) => format!("={}", f),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cell {
    pub raw: CellValue,
    pub computed: CellValue,
    pub bold: bool,
    pub italic: bool,
    pub format: Option<String>,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            raw: CellValue::Empty,
            computed: CellValue::Empty,
            bold: false,
            italic: false,
            format: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sheet {
    pub id: Id,
    pub name: String,
    pub cells: HashMap<CellCoord, Cell>,
    pub max_col: u32,
    pub max_row: u32,
}

impl Sheet {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Id::new(),
            name: name.into(),
            cells: HashMap::new(),
            max_col: 0,
            max_row: 0,
        }
    }

    pub fn set_cell(&mut self, coord: CellCoord, input: &str) {
        let raw = if let Some(stripped) = input.strip_prefix('=') {
            CellValue::Formula(stripped.trim().to_string())
        } else if let Ok(num) = input.parse::<f64>() {
            CellValue::Number(num)
        } else if input.eq_ignore_ascii_case("true") {
            CellValue::Boolean(true)
        } else if input.eq_ignore_ascii_case("false") {
            CellValue::Boolean(false)
        } else if input.is_empty() {
            CellValue::Empty
        } else {
            CellValue::Text(input.to_string())
        };

        if coord.col >= self.max_col {
            self.max_col = coord.col + 1;
        }
        if coord.row >= self.max_row {
            self.max_row = coord.row + 1;
        }

        let cell = self.cells.entry(coord).or_default();
        cell.raw = raw.clone();
        if !matches!(raw, CellValue::Formula(_)) {
            cell.computed = raw;
        }
    }

    pub fn get_cell(&self, coord: &CellCoord) -> &Cell {
        static DEFAULT_CELL: Cell = Cell {
            raw: CellValue::Empty,
            computed: CellValue::Empty,
            bold: false,
            italic: false,
            format: None,
        };
        self.cells.get(coord).unwrap_or(&DEFAULT_CELL)
    }

    pub fn write_range(&mut self, start_str: &str, rows: Vec<Vec<String>>) -> Result<()> {
        let start = CellCoord::parse(start_str)
            .ok_or_else(|| DreamError::Grid(format!("Invalid coord: {}", start_str)))?;
        for (r_idx, row) in rows.into_iter().enumerate() {
            for (c_idx, val) in row.into_iter().enumerate() {
                let coord = CellCoord::new(start.col + c_idx as u32, start.row + r_idx as u32);
                self.set_cell(coord, &val);
            }
        }
        self.recalculate();
        Ok(())
    }

    pub fn read_range(&self, range_str: &str) -> Result<Vec<Vec<String>>> {
        let parts: Vec<&str> = range_str.split(':').collect();
        let (start, end) = if parts.len() == 1 {
            let c = CellCoord::parse(parts[0])
                .ok_or_else(|| DreamError::Grid(format!("Invalid coord: {}", parts[0])))?;
            (c, c)
        } else if parts.len() == 2 {
            let c1 = CellCoord::parse(parts[0])
                .ok_or_else(|| DreamError::Grid(format!("Invalid start coord: {}", parts[0])))?;
            let c2 = CellCoord::parse(parts[1])
                .ok_or_else(|| DreamError::Grid(format!("Invalid end coord: {}", parts[1])))?;
            (c1, c2)
        } else {
            return Err(DreamError::Grid(format!("Invalid range: {}", range_str)));
        };

        let min_col = start.col.min(end.col);
        let max_col = start.col.max(end.col);
        let min_row = start.row.min(end.row);
        let max_row = start.row.max(end.row);

        let mut res = Vec::new();
        for r in min_row..=max_row {
            let mut row_vals = Vec::new();
            for c in min_col..=max_col {
                let cell = self.get_cell(&CellCoord::new(c, r));
                row_vals.push(cell.computed.display_string());
            }
            res.push(row_vals);
        }
        Ok(res)
    }

    /// Evaluates spreadsheet formulas like SUM(A1:A5), AVERAGE(B1:B10), etc.
    pub fn recalculate(&mut self) {
        let formulas: Vec<(CellCoord, String)> = self
            .cells
            .iter()
            .filter_map(|(coord, cell)| match &cell.raw {
                CellValue::Formula(f) => Some((*coord, f.clone())),
                _ => None,
            })
            .collect();

        for (coord, formula) in formulas {
            let computed = self.eval_formula_str(&formula);
            if let Some(cell) = self.cells.get_mut(&coord) {
                cell.computed = computed;
            }
        }
    }

    fn eval_formula_str(&self, formula: &str) -> CellValue {
        let trimmed = formula.trim();
        let upper = trimmed.to_uppercase();

        if let Some(inner) = upper.strip_prefix("SUM(").and_then(|s| s.strip_suffix(')')) {
            let sum: f64 = self.resolve_range_numbers(inner).iter().sum();
            CellValue::Number(sum)
        } else if let Some(inner) = upper.strip_prefix("AVERAGE(").and_then(|s| s.strip_suffix(')')) {
            let nums = self.resolve_range_numbers(inner);
            if nums.is_empty() {
                CellValue::Number(0.0)
            } else {
                CellValue::Number(nums.iter().sum::<f64>() / nums.len() as f64)
            }
        } else if let Some(inner) = upper.strip_prefix("COUNT(").and_then(|s| s.strip_suffix(')')) {
            let nums = self.resolve_range_numbers(inner);
            CellValue::Number(nums.len() as f64)
        } else if let Some(inner) = upper.strip_prefix("MAX(").and_then(|s| s.strip_suffix(')')) {
            let nums = self.resolve_range_numbers(inner);
            let max = nums.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            CellValue::Number(if max.is_infinite() { 0.0 } else { max })
        } else if let Some(inner) = upper.strip_prefix("MIN(").and_then(|s| s.strip_suffix(')')) {
            let nums = self.resolve_range_numbers(inner);
            let min = nums.iter().cloned().fold(f64::INFINITY, f64::min);
            CellValue::Number(if min.is_infinite() { 0.0 } else { min })
        } else if let Some(coord) = CellCoord::parse(trimmed) {
            self.get_cell(&coord).computed.clone()
        } else {
            // Simple arithmetic like 10 + 20
            CellValue::Text(format!("={}", formula))
        }
    }

    fn resolve_range_numbers(&self, range_expr: &str) -> Vec<f64> {
        let mut nums = Vec::new();
        for part in range_expr.split(',') {
            let part = part.trim();
            if let Ok(grid) = self.read_range(part) {
                for row in grid {
                    for val in row {
                        if let Ok(n) = val.parse::<f64>() {
                            nums.push(n);
                        }
                    }
                }
            }
        }
        nums
    }

    pub fn to_csv(&self) -> String {
        let mut out = String::new();
        for r in 0..self.max_row {
            let mut row_vals = Vec::new();
            for c in 0..self.max_col {
                let cell = self.get_cell(&CellCoord::new(c, r));
                let val = cell.computed.display_string();
                if val.contains(',') || val.contains('"') || val.contains('\n') {
                    row_vals.push(format!("\"{}\"", val.replace('"', "\"\"")));
                } else {
                    row_vals.push(val);
                }
            }
            out.push_str(&row_vals.join(","));
            out.push('\n');
        }
        out
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Workbook {
    pub id: Id,
    pub title: String,
    pub sheets: Vec<Sheet>,
    pub active_sheet: usize,
}

impl Workbook {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            id: Id::new(),
            title: title.into(),
            sheets: vec![Sheet::new("Sheet1")],
            active_sheet: 0,
        }
    }

    pub fn active_sheet_mut(&mut self) -> &mut Sheet {
        &mut self.sheets[self.active_sheet]
    }

    pub fn active_sheet(&self) -> &Sheet {
        &self.sheets[self.active_sheet]
    }

    pub fn add_sheet(&mut self, name: impl Into<String>) -> usize {
        self.sheets.push(Sheet::new(name));
        self.sheets.len() - 1
    }
}
