use excelize_rs::{Cell, Spreadsheet};

fn main() {
    let path = String::from(
        "D:\\1_workplace\\rust\\grzeng\\office_tools\\103__个别报表_2022年度_上海蓝天_V0.66.xlsx",
    );
    let wb = Spreadsheet::open_file(path);
    match wb {
        Ok(ws) => {
            if let Some(s) = ws.worksheets.get("Notes") {
                if let Some(ref rows) = s.sheet_data.row {
                    for (ri, r) in rows.iter().enumerate() {
                        if let Some(ref cells) = r.c {
                            for (ci, c) in cells.iter().enumerate() {
                                print!("{}-{}: {:?}; ", ri, ci, ws.get_value_from(c));
                            }
                        }
                        println!("");
                    }
                }
            }
            /* match ws.get_cell_value("Sheet1", 19, 2) {
                Ok(c) => {
                    let cell = String::from(c);
                    println!("the value of cell A1 is: {}", cell)
                }
                Err(e) => println!("{:?}", e),
            } */
        }
        Err(e) => print!("{:?}", e),
    }
}
