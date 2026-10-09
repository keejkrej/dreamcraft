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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum NumberFormat {
    #[default]
    General,
    Integer,
    Decimal(usize),
    Currency { symbol: String, decimals: usize },
    Percentage { decimals: usize },
    Date(String),
}

impl NumberFormat {
    pub fn format_value(&self, val: &CellValue) -> String {
        match (self, val) {
            (NumberFormat::General, _) => val.display_string(),
            (NumberFormat::Integer, CellValue::Number(n)) => format!("{:.0}", n),
            (NumberFormat::Decimal(d), CellValue::Number(n)) => format!("{:.*}", d, n),
            (NumberFormat::Currency { symbol, decimals }, CellValue::Number(n)) => {
                if *n < 0.0 {
                    format!("-{}{:.*}", symbol, decimals, n.abs())
                } else {
                    format!("{}{:.*}", symbol, decimals, n)
                }
            }
            (NumberFormat::Percentage { decimals }, CellValue::Number(n)) => {
                format!("{:.*}%", decimals, n * 100.0)
            }
            (NumberFormat::Date(_), CellValue::Text(s)) => s.clone(),
            (_, other) => other.display_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum CellValue {
    #[default]
    Empty,
    Number(f64),
    Text(String),
    Boolean(bool),
    Formula(String),
    Error(String),
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

    pub fn as_str(&self) -> &str {
        match self {
            CellValue::Text(s) => s.as_str(),
            CellValue::Formula(f) => f.as_str(),
            CellValue::Error(e) => e.as_str(),
            _ => "",
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
            CellValue::Error(e) => format!("#{}!", e),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cell {
    pub raw: CellValue,
    pub computed: CellValue,
    pub bold: bool,
    pub italic: bool,
    pub format: NumberFormat,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            raw: CellValue::Empty,
            computed: CellValue::Empty,
            bold: false,
            italic: false,
            format: NumberFormat::General,
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
    pub col_widths: HashMap<u32, f32>,
    pub row_heights: HashMap<u32, f32>,
}

impl Sheet {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Id::new(),
            name: name.into(),
            cells: HashMap::new(),
            max_col: 0,
            max_row: 0,
            col_widths: HashMap::new(),
            row_heights: HashMap::new(),
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

    pub fn set_cell_value(&mut self, coord: CellCoord, val: CellValue) {
        if coord.col >= self.max_col {
            self.max_col = coord.col + 1;
        }
        if coord.row >= self.max_row {
            self.max_row = coord.row + 1;
        }
        let cell = self.cells.entry(coord).or_default();
        cell.raw = val.clone();
        if !matches!(val, CellValue::Formula(_)) {
            cell.computed = val;
        }
    }

    pub fn set_format(&mut self, coord: CellCoord, format: NumberFormat) {
        self.cells.entry(coord).or_default().format = format;
    }

    pub fn set_style(&mut self, coord: CellCoord, bold: bool, italic: bool) {
        let c = self.cells.entry(coord).or_default();
        c.bold = bold;
        c.italic = italic;
    }

    pub fn get_cell(&self, coord: &CellCoord) -> &Cell {
        static DEFAULT_CELL: Cell = Cell {
            raw: CellValue::Empty,
            computed: CellValue::Empty,
            bold: false,
            italic: false,
            format: NumberFormat::General,
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
        let (start, end) = self.parse_range_coords(range_str)?;
        let min_col = start.col.min(end.col);
        let max_col = start.col.max(end.col);
        let min_row = start.row.min(end.row);
        let max_row = start.row.max(end.row);

        let mut res = Vec::new();
        for r in min_row..=max_row {
            let mut row_vals = Vec::new();
            for c in min_col..=max_col {
                let cell = self.get_cell(&CellCoord::new(c, r));
                row_vals.push(cell.format.format_value(&cell.computed));
            }
            res.push(row_vals);
        }
        Ok(res)
    }

    pub fn parse_range_coords(&self, range_str: &str) -> Result<(CellCoord, CellCoord)> {
        let parts: Vec<&str> = range_str.split(':').collect();
        if parts.len() == 1 {
            let c = CellCoord::parse(parts[0])
                .ok_or_else(|| DreamError::Grid(format!("Invalid coord: {}", parts[0])))?;
            Ok((c, c))
        } else if parts.len() == 2 {
            let c1 = CellCoord::parse(parts[0])
                .ok_or_else(|| DreamError::Grid(format!("Invalid start coord: {}", parts[0])))?;
            let c2 = CellCoord::parse(parts[1])
                .ok_or_else(|| DreamError::Grid(format!("Invalid end coord: {}", parts[1])))?;
            Ok((c1, c2))
        } else {
            Err(DreamError::Grid(format!("Invalid range: {}", range_str)))
        }
    }

    /// Sort a rectangular range by a specified column index (0-relative to range start)
    pub fn sort_range(&mut self, range_str: &str, sort_col_offset: u32, ascending: bool, has_header: bool) -> Result<()> {
        let (start, end) = self.parse_range_coords(range_str)?;
        let min_col = start.col.min(end.col);
        let max_col = start.col.max(end.col);
        let start_row = if has_header { start.row + 1 } else { start.row };
        let end_row = end.row;

        if start_row >= end_row {
            return Ok(());
        }

        let mut rows: Vec<Vec<(CellCoord, Cell)>> = Vec::new();
        for r in start_row..=end_row {
            let mut row_cells = Vec::new();
            for c in min_col..=max_col {
                let coord = CellCoord::new(c, r);
                row_cells.push((coord, self.get_cell(&coord).clone()));
            }
            rows.push(row_cells);
        }

        let key_col_idx = sort_col_offset as usize;
        rows.sort_by(|a, b| {
            let val_a = a.get(key_col_idx).map(|(_, cell)| &cell.computed);
            let val_b = b.get(key_col_idx).map(|(_, cell)| &cell.computed);

            let ord = match (val_a, val_b) {
                (Some(CellValue::Number(na)), Some(CellValue::Number(nb))) => {
                    na.partial_cmp(nb).unwrap_or(std::cmp::Ordering::Equal)
                }
                (Some(ca), Some(cb)) => {
                    ca.display_string().cmp(&cb.display_string())
                }
                _ => std::cmp::Ordering::Equal,
            };

            if ascending { ord } else { ord.reverse() }
        });

        for (r_offset, row) in rows.into_iter().enumerate() {
            let target_row = start_row + r_offset as u32;
            for (c_offset, (_, cell)) in row.into_iter().enumerate() {
                let target_coord = CellCoord::new(min_col + c_offset as u32, target_row);
                self.cells.insert(target_coord, cell);
            }
        }

        self.recalculate();
        Ok(())
    }

    /// Evaluates spreadsheet formulas like SUM(A1:A5), AVERAGE(B1:B10), IF, VLOOKUP, etc.
    pub fn recalculate(&mut self) {
        let formulas: Vec<(CellCoord, String)> = self
            .cells
            .iter()
            .filter_map(|(coord, cell)| match &cell.raw {
                CellValue::Formula(f) => Some((*coord, f.clone())),
                _ => None,
            })
            .collect();

        // Perform calculation passes (supports dependent formulas up to 4 iterations)
        for _ in 0..4 {
            for (coord, formula) in &formulas {
                let computed = self.eval_formula_str(formula);
                if let Some(cell) = self.cells.get_mut(coord) {
                    cell.computed = computed;
                }
            }
        }
    }

    pub fn eval_formula_str(&self, formula: &str) -> CellValue {
        let trimmed = formula.trim();
        let upper = trimmed.to_uppercase();

        // 1. Check IF function: IF(condition, true_val, false_val)
        if let Some(args) = self.extract_fn_args(trimmed, "IF") {
            if args.len() >= 2 {
                let cond_res = self.eval_condition(&args[0]);
                let true_expr = &args[1];
                let false_expr = args.get(2).map(|s| s.as_str()).unwrap_or("0");
                return if cond_res {
                    self.eval_atom(true_expr)
                } else {
                    self.eval_atom(false_expr)
                };
            }
        }

        // 2. VLOOKUP: VLOOKUP(needle, range, col_index, [exact_match])
        if let Some(args) = self.extract_fn_args(trimmed, "VLOOKUP") {
            if args.len() >= 3 {
                let needle = self.eval_atom(&args[0]);
                let range_str = &args[1];
                let col_idx = args[2].parse::<usize>().unwrap_or(1).saturating_sub(1);

                if let Ok((start, end)) = self.parse_range_coords(range_str) {
                    let min_col = start.col.min(end.col);
                    let max_col = start.col.max(end.col);
                    let target_col = min_col + col_idx as u32;

                    if target_col <= max_col {
                        for r in start.row..=end.row {
                            let key_cell = self.get_cell(&CellCoord::new(min_col, r));
                            if self.values_equal(&key_cell.computed, &needle) {
                                let match_cell = self.get_cell(&CellCoord::new(target_col, r));
                                return match_cell.computed.clone();
                            }
                        }
                    }
                }
                return CellValue::Error("N/A".into());
            }
        }

        // 3. INDEX & MATCH: INDEX(range, row, [col]), MATCH(needle, range, [match_type])
        if let Some(args) = self.extract_fn_args(&upper, "INDEX") {
            if args.len() >= 2 {
                let range_str = &args[0];
                let row_idx = self.eval_atom(&args[1]).as_number().unwrap_or(1.0) as u32;
                let col_idx = if args.len() >= 3 {
                    self.eval_atom(&args[2]).as_number().unwrap_or(1.0) as u32
                } else {
                    1
                };

                if let Ok((start, end)) = self.parse_range_coords(range_str) {
                    let coord = CellCoord::new(
                        start.col + col_idx.saturating_sub(1),
                        start.row + row_idx.saturating_sub(1),
                    );
                    if coord.col <= end.col && coord.row <= end.row {
                        return self.get_cell(&coord).computed.clone();
                    }
                }
                return CellValue::Error("REF".into());
            }
        }

        if let Some(args) = self.extract_fn_args(&upper, "MATCH") {
            if args.len() >= 2 {
                let needle = self.eval_atom(&args[0]);
                let range_str = &args[1];
                if let Ok((start, end)) = self.parse_range_coords(range_str) {
                    let is_horizontal = start.row == end.row;
                    if is_horizontal {
                        for (idx, c) in (start.col..=end.col).enumerate() {
                            let cell = self.get_cell(&CellCoord::new(c, start.row));
                            if self.values_equal(&cell.computed, &needle) {
                                return CellValue::Number((idx + 1) as f64);
                            }
                        }
                    } else {
                        for (idx, r) in (start.row..=end.row).enumerate() {
                            let cell = self.get_cell(&CellCoord::new(start.col, r));
                            if self.values_equal(&cell.computed, &needle) {
                                return CellValue::Number((idx + 1) as f64);
                            }
                        }
                    }
                }
                return CellValue::Error("N/A".into());
            }
        }

        // 4. SUMIF(range, criteria, [sum_range])
        if let Some(args) = self.extract_fn_args(&upper, "SUMIF") {
            if args.len() >= 2 {
                let cond_range = &args[0];
                let criteria = args[1].trim_matches('"');
                let sum_range = args.get(2).unwrap_or(&args[0]);

                if let (Ok((c_start, c_end)), Ok((s_start, _))) = (
                    self.parse_range_coords(cond_range),
                    self.parse_range_coords(sum_range),
                ) {
                    let mut total = 0.0;
                    for (offset, r) in (c_start.row..=c_end.row).enumerate() {
                        let c_cell = self.get_cell(&CellCoord::new(c_start.col, r));
                        if self.match_criteria(&c_cell.computed, criteria) {
                            let s_cell = self.get_cell(&CellCoord::new(s_start.col, s_start.row + offset as u32));
                            if let Some(num) = s_cell.computed.as_number() {
                                total += num;
                            }
                        }
                    }
                    return CellValue::Number(total);
                }
            }
        }

        // 5. COUNTIF(range, criteria)
        if let Some(args) = self.extract_fn_args(&upper, "COUNTIF") {
            if args.len() >= 2 {
                let range_str = &args[0];
                let criteria = args[1].trim_matches('"');
                if let Ok((start, end)) = self.parse_range_coords(range_str) {
                    let mut count = 0;
                    for r in start.row..=end.row {
                        for c in start.col..=end.col {
                            let cell = self.get_cell(&CellCoord::new(c, r));
                            if self.match_criteria(&cell.computed, criteria) {
                                count += 1;
                            }
                        }
                    }
                    return CellValue::Number(count as f64);
                }
            }
        }

        // 6. Text functions: CONCAT, LEFT, RIGHT, MID, LEN, UPPER, LOWER, TRIM
        if let Some(args) = self.extract_fn_args(&upper, "CONCAT") {
            let mut s = String::new();
            for a in args {
                s.push_str(&self.eval_atom(&a).display_string());
            }
            return CellValue::Text(s);
        }

        if let Some(args) = self.extract_fn_args(&upper, "LEFT") {
            if let Some(first) = args.first() {
                let s = self.eval_atom(first).display_string();
                let n = args.get(1).and_then(|v| v.parse::<usize>().ok()).unwrap_or(1);
                return CellValue::Text(s.chars().take(n).collect());
            }
        }

        if let Some(args) = self.extract_fn_args(&upper, "RIGHT") {
            if let Some(first) = args.first() {
                let s = self.eval_atom(first).display_string();
                let n = args.get(1).and_then(|v| v.parse::<usize>().ok()).unwrap_or(1);
                let len = s.chars().count();
                return CellValue::Text(s.chars().skip(len.saturating_sub(n)).collect());
            }
        }

        if let Some(args) = self.extract_fn_args(&upper, "LEN") {
            if let Some(first) = args.first() {
                let s = self.eval_atom(first).display_string();
                return CellValue::Number(s.chars().count() as f64);
            }
        }

        if let Some(args) = self.extract_fn_args(&upper, "UPPER") {
            if let Some(first) = args.first() {
                return CellValue::Text(self.eval_atom(first).display_string().to_uppercase());
            }
        }

        if let Some(args) = self.extract_fn_args(&upper, "LOWER") {
            if let Some(first) = args.first() {
                return CellValue::Text(self.eval_atom(first).display_string().to_lowercase());
            }
        }

        if let Some(args) = self.extract_fn_args(&upper, "TRIM") {
            if let Some(first) = args.first() {
                return CellValue::Text(self.eval_atom(first).display_string().trim().to_string());
            }
        }

        // 7. Standard Aggregations: SUM, AVERAGE, COUNT, MIN, MAX, PRODUCT, MEDIAN
        if let Some(inner) = upper.strip_prefix("SUM(").and_then(|s| s.strip_suffix(')')) {
            let sum: f64 = self.resolve_range_numbers(inner).iter().sum();
            return CellValue::Number(sum);
        } else if let Some(inner) = upper.strip_prefix("AVERAGE(").and_then(|s| s.strip_suffix(')')) {
            let nums = self.resolve_range_numbers(inner);
            return if nums.is_empty() {
                CellValue::Number(0.0)
            } else {
                CellValue::Number(nums.iter().sum::<f64>() / nums.len() as f64)
            };
        } else if let Some(inner) = upper.strip_prefix("COUNT(").and_then(|s| s.strip_suffix(')')) {
            let nums = self.resolve_range_numbers(inner);
            return CellValue::Number(nums.len() as f64);
        } else if let Some(inner) = upper.strip_prefix("MAX(").and_then(|s| s.strip_suffix(')')) {
            let nums = self.resolve_range_numbers(inner);
            let max = nums.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            return CellValue::Number(if max.is_infinite() { 0.0 } else { max });
        } else if let Some(inner) = upper.strip_prefix("MIN(").and_then(|s| s.strip_suffix(')')) {
            let nums = self.resolve_range_numbers(inner);
            let min = nums.iter().cloned().fold(f64::INFINITY, f64::min);
            return CellValue::Number(if min.is_infinite() { 0.0 } else { min });
        } else if let Some(inner) = upper.strip_prefix("PRODUCT(").and_then(|s| s.strip_suffix(')')) {
            let nums = self.resolve_range_numbers(inner);
            let prod: f64 = nums.iter().product();
            return CellValue::Number(prod);
        } else if let Some(inner) = upper.strip_prefix("MEDIAN(").and_then(|s| s.strip_suffix(')')) {
            let mut nums = self.resolve_range_numbers(inner);
            nums.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            if nums.is_empty() {
                return CellValue::Number(0.0);
            }
            let mid = nums.len() / 2;
            let med = if nums.len() % 2 == 0 {
                (nums[mid - 1] + nums[mid]) / 2.0
            } else {
                nums[mid]
            };
            return CellValue::Number(med);
        } else if let Some(inner) = upper.strip_prefix("ROUND(").and_then(|s| s.strip_suffix(')')) {
            let parts: Vec<&str> = inner.split(',').collect();
            if let Some(val_str) = parts.first() {
                let n = self.eval_atom(val_str).as_number().unwrap_or(0.0);
                let digits = parts.get(1).and_then(|d| d.trim().parse::<i32>().ok()).unwrap_or(0);
                let factor = 10f64.powi(digits);
                return CellValue::Number((n * factor).round() / factor);
            }
        } else if let Some(inner) = upper.strip_prefix("ABS(").and_then(|s| s.strip_suffix(')')) {
            let n = self.eval_atom(inner).as_number().unwrap_or(0.0);
            return CellValue::Number(n.abs());
        } else if let Some(inner) = upper.strip_prefix("SQRT(").and_then(|s| s.strip_suffix(')')) {
            let n = self.eval_atom(inner).as_number().unwrap_or(0.0);
            return CellValue::Number(n.sqrt());
        }

        // 8. Basic arithmetic expressions (+, -, *, /)
        if upper.contains('+') {
            let parts: Vec<&str> = upper.splitn(2, '+').collect();
            let a = self.eval_atom(parts[0]).as_number().unwrap_or(0.0);
            let b = self.eval_atom(parts[1]).as_number().unwrap_or(0.0);
            return CellValue::Number(a + b);
        }
        if upper.contains('-') {
            let parts: Vec<&str> = upper.splitn(2, '-').collect();
            let a = self.eval_atom(parts[0]).as_number().unwrap_or(0.0);
            let b = self.eval_atom(parts[1]).as_number().unwrap_or(0.0);
            return CellValue::Number(a - b);
        }
        if upper.contains('*') {
            let parts: Vec<&str> = upper.splitn(2, '*').collect();
            let a = self.eval_atom(parts[0]).as_number().unwrap_or(0.0);
            let b = self.eval_atom(parts[1]).as_number().unwrap_or(0.0);
            return CellValue::Number(a * b);
        }
        if upper.contains('/') {
            let parts: Vec<&str> = upper.splitn(2, '/').collect();
            let a = self.eval_atom(parts[0]).as_number().unwrap_or(0.0);
            let b = self.eval_atom(parts[1]).as_number().unwrap_or(1.0);
            return if b == 0.0 {
                CellValue::Error("DIV/0".into())
            } else {
                CellValue::Number(a / b)
            };
        }

        // 9. Single Cell Reference (e.g. "A1")
        if let Some(coord) = CellCoord::parse(&upper) {
            return self.get_cell(&coord).computed.clone();
        }

        CellValue::Error("VALUE".into())
    }

    fn extract_fn_args(&self, expr: &str, fn_name: &str) -> Option<Vec<String>> {
        let prefix = format!("{}(", fn_name);
        if expr.len() >= prefix.len() + 1 && expr[..prefix.len()].eq_ignore_ascii_case(&prefix) && expr.ends_with(')') {
            let inner = &expr[prefix.len()..expr.len() - 1];
            let mut args = Vec::new();
            let mut depth = 0;
            let mut cur = String::new();

            for c in inner.chars() {
                match c {
                    '(' => {
                        depth += 1;
                        cur.push(c);
                    }
                    ')' => {
                        depth -= 1;
                        cur.push(c);
                    }
                    ',' if depth == 0 => {
                        args.push(cur.trim().to_string());
                        cur.clear();
                    }
                    _ => cur.push(c),
                }
            }
            if !cur.is_empty() {
                args.push(cur.trim().to_string());
            }
            Some(args)
        } else {
            None
        }
    }

    fn eval_atom(&self, expr: &str) -> CellValue {
        let trimmed = expr.trim();
        if let Ok(num) = trimmed.parse::<f64>() {
            CellValue::Number(num)
        } else if (trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2)
            || (trimmed.starts_with('\'') && trimmed.ends_with('\'') && trimmed.len() >= 2)
        {
            CellValue::Text(trimmed[1..trimmed.len() - 1].to_string())
        } else if let Some(coord) = CellCoord::parse(trimmed) {
            self.get_cell(&coord).computed.clone()
        } else if trimmed.eq_ignore_ascii_case("true") {
            CellValue::Boolean(true)
        } else if trimmed.eq_ignore_ascii_case("false") {
            CellValue::Boolean(false)
        } else if !trimmed.contains('(') && !trimmed.contains('+') && !trimmed.contains('-') && !trimmed.contains('*') && !trimmed.contains('/') {
            CellValue::Text(trimmed.to_string())
        } else {
            self.eval_formula_str(trimmed)
        }
    }

    fn eval_condition(&self, cond: &str) -> bool {
        let ops = [">=", "<=", "!=", "=", ">", "<"];
        for op in ops {
            if cond.contains(op) {
                let parts: Vec<&str> = cond.splitn(2, op).collect();
                let left = self.eval_atom(parts[0]);
                let right = self.eval_atom(parts[1]);

                let (ln, rn) = (left.as_number(), right.as_number());
                return match op {
                    ">=" => ln.unwrap_or(0.0) >= rn.unwrap_or(0.0),
                    "<=" => ln.unwrap_or(0.0) <= rn.unwrap_or(0.0),
                    ">" => ln.unwrap_or(0.0) > rn.unwrap_or(0.0),
                    "<" => ln.unwrap_or(0.0) < rn.unwrap_or(0.0),
                    "=" => self.values_equal(&left, &right),
                    "!=" => !self.values_equal(&left, &right),
                    _ => false,
                };
            }
        }
        false
    }

    fn values_equal(&self, a: &CellValue, b: &CellValue) -> bool {
        match (a, b) {
            (CellValue::Number(na), CellValue::Number(nb)) => (na - nb).abs() < f64::EPSILON,
            (CellValue::Text(ta), CellValue::Text(tb)) => ta.eq_ignore_ascii_case(tb),
            (CellValue::Boolean(ba), CellValue::Boolean(bb)) => ba == bb,
            _ => a.display_string().eq_ignore_ascii_case(&b.display_string()),
        }
    }

    fn match_criteria(&self, val: &CellValue, criteria: &str) -> bool {
        let c = criteria.trim();
        if let Some(rest) = c.strip_prefix(">=") {
            val.as_number().unwrap_or(0.0) >= rest.parse::<f64>().unwrap_or(0.0)
        } else if let Some(rest) = c.strip_prefix("<=") {
            val.as_number().unwrap_or(0.0) <= rest.parse::<f64>().unwrap_or(0.0)
        } else if let Some(rest) = c.strip_prefix('>') {
            val.as_number().unwrap_or(0.0) > rest.parse::<f64>().unwrap_or(0.0)
        } else if let Some(rest) = c.strip_prefix('<') {
            val.as_number().unwrap_or(0.0) < rest.parse::<f64>().unwrap_or(0.0)
        } else if let Some(rest) = c.strip_prefix("<>") {
            !val.display_string().eq_ignore_ascii_case(rest)
        } else {
            val.display_string().eq_ignore_ascii_case(c)
        }
    }

    fn resolve_range_numbers(&self, range_or_list: &str) -> Vec<f64> {
        let mut numbers = Vec::new();
        let items: Vec<&str> = range_or_list.split(',').collect();

        for item in items {
            let item = item.trim();
            if item.contains(':') {
                let parts: Vec<&str> = item.split(':').collect();
                if parts.len() == 2 {
                    if let (Some(c1), Some(c2)) = (CellCoord::parse(parts[0]), CellCoord::parse(parts[1])) {
                        let min_col = c1.col.min(c2.col);
                        let max_col = c1.col.max(c2.col);
                        let min_row = c1.row.min(c2.row);
                        let max_row = c1.row.max(c2.row);

                        for r in min_row..=max_row {
                            for c in min_col..=max_col {
                                if let Some(n) = self.get_cell(&CellCoord::new(c, r)).computed.as_number() {
                                    numbers.push(n);
                                }
                            }
                        }
                    }
                }
            } else if let Some(coord) = CellCoord::parse(item) {
                if let Some(n) = self.get_cell(&coord).computed.as_number() {
                    numbers.push(n);
                }
            } else if let Ok(n) = item.parse::<f64>() {
                numbers.push(n);
            }
        }

        numbers
    }

    pub fn to_csv(&self) -> String {
        let mut out = String::new();
        for r in 0..self.max_row {
            let mut row_vals = Vec::new();
            for c in 0..self.max_col {
                let cell = self.get_cell(&CellCoord::new(c, r));
                let s = cell.format.format_value(&cell.computed);
                if s.contains(',') || s.contains('"') || s.contains('\n') {
                    row_vals.push(format!("\"{}\"", s.replace('"', "\"\"")));
                } else {
                    row_vals.push(s);
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
    pub active_sheet_index: usize,
}

impl Default for Workbook {
    fn default() -> Self {
        Self::new("Untitled Workbook")
    }
}

impl Workbook {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            id: Id::new(),
            title: title.into(),
            sheets: vec![Sheet::new("Sheet1")],
            active_sheet_index: 0,
        }
    }

    pub fn active_sheet(&self) -> &Sheet {
        self.sheets.get(self.active_sheet_index).unwrap_or(&self.sheets[0])
    }

    pub fn active_sheet_mut(&mut self) -> &mut Sheet {
        let idx = self.active_sheet_index;
        &mut self.sheets[idx]
    }

    pub fn add_sheet(&mut self, name: impl Into<String>) -> usize {
        let s = Sheet::new(name);
        self.sheets.push(s);
        self.sheets.len() - 1
    }

    pub fn select_sheet(&mut self, index: usize) -> Result<()> {
        if index < self.sheets.len() {
            self.active_sheet_index = index;
            Ok(())
        } else {
            Err(DreamError::Grid(format!("Sheet index {} out of bounds", index)))
        }
    }

    pub fn get_sheet_by_name(&self, name: &str) -> Option<&Sheet> {
        self.sheets.iter().find(|s| s.name.eq_ignore_ascii_case(name))
    }

    pub fn get_sheet_by_name_mut(&mut self, name: &str) -> Option<&mut Sheet> {
        self.sheets.iter_mut().find(|s| s.name.eq_ignore_ascii_case(name))
    }

    pub fn recalculate_all(&mut self) {
        for sheet in &mut self.sheets {
            sheet.recalculate();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_formulas_and_calc() {
        let mut sheet = Sheet::new("Test");
        sheet.set_cell(CellCoord::new(0, 0), "10");
        sheet.set_cell(CellCoord::new(0, 1), "20");
        sheet.set_cell(CellCoord::new(0, 2), "30");
        sheet.set_cell(CellCoord::new(0, 3), "=SUM(A1:A3)");
        sheet.set_cell(CellCoord::new(0, 4), "=AVERAGE(A1:A3)");
        sheet.set_cell(CellCoord::new(0, 5), "=MAX(A1:A3)");
        sheet.set_cell(CellCoord::new(0, 6), "=MIN(A1:A3)");
        sheet.set_cell(CellCoord::new(0, 7), "=COUNT(A1:A3)");

        sheet.recalculate();

        assert_eq!(sheet.get_cell(&CellCoord::new(0, 3)).computed, CellValue::Number(60.0));
        assert_eq!(sheet.get_cell(&CellCoord::new(0, 4)).computed, CellValue::Number(20.0));
        assert_eq!(sheet.get_cell(&CellCoord::new(0, 5)).computed, CellValue::Number(30.0));
        assert_eq!(sheet.get_cell(&CellCoord::new(0, 6)).computed, CellValue::Number(10.0));
        assert_eq!(sheet.get_cell(&CellCoord::new(0, 7)).computed, CellValue::Number(3.0));
    }

    #[test]
    fn test_grid_if_and_lookup() {
        let mut sheet = Sheet::new("Lookup");
        // A1: "Apple", B1: 100
        // A2: "Banana", B2: 200
        sheet.set_cell(CellCoord::new(0, 0), "Apple");
        sheet.set_cell(CellCoord::new(1, 0), "100");
        sheet.set_cell(CellCoord::new(0, 1), "Banana");
        sheet.set_cell(CellCoord::new(1, 1), "200");

        sheet.set_cell(CellCoord::new(0, 2), "=VLOOKUP(\"Banana\", A1:B2, 2)");
        sheet.set_cell(CellCoord::new(0, 3), "=IF(A1=Apple, \"Yes\", \"No\")");

        sheet.recalculate();

        assert_eq!(sheet.get_cell(&CellCoord::new(0, 2)).computed, CellValue::Number(200.0));
        assert_eq!(sheet.get_cell(&CellCoord::new(0, 3)).computed, CellValue::Text("Yes".into()));
    }

    #[test]
    fn test_grid_sort_range() {
        let mut sheet = Sheet::new("Sort");
        sheet.set_cell(CellCoord::new(0, 0), "Name");
        sheet.set_cell(CellCoord::new(1, 0), "Score");

        sheet.set_cell(CellCoord::new(0, 1), "Charlie");
        sheet.set_cell(CellCoord::new(1, 1), "85");

        sheet.set_cell(CellCoord::new(0, 2), "Alice");
        sheet.set_cell(CellCoord::new(1, 2), "95");

        sheet.set_cell(CellCoord::new(0, 3), "Bob");
        sheet.set_cell(CellCoord::new(1, 3), "90");

        sheet.sort_range("A1:B4", 1, false, true).unwrap();

        // Should be sorted by score descending: Alice (95), Bob (90), Charlie (85)
        assert_eq!(sheet.get_cell(&CellCoord::new(0, 1)).computed, CellValue::Text("Alice".into()));
        assert_eq!(sheet.get_cell(&CellCoord::new(0, 2)).computed, CellValue::Text("Bob".into()));
        assert_eq!(sheet.get_cell(&CellCoord::new(0, 3)).computed, CellValue::Text("Charlie".into()));
    }
}
