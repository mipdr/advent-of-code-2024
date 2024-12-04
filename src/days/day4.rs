use std::{fs::{read_to_string, File}, io::Write, path::Path};

fn repeating_char(c: char, n: usize) -> String {
    return std::iter::repeat(c).take(n).collect::<String>()
}

pub fn day4(input: &Path, result: &Path) {
    let lines: Vec<String> = read_to_string(input) 
        .unwrap()  // panic on possible file-reading errors
        .lines()
        .map(String::from)
        .collect();

    let mut lines_to_search: Vec<String> = [].to_vec();

    // Row search
    for line in &lines {
        // Forwards
        lines_to_search.push(line.clone());
        // Backwards
        lines_to_search.push(line.chars().rev().collect());
    }

    /*
        Skewed search space for diagonals to become verticals

        For diagonals going up and right:

            Line -> Skew

        For diagonals going down and right:

            Line -> Reverse -> Skew

        Skew transformation ilustration:
    
        ....XXMAS.
        .SAMXMS...
        ...S..A...
        ..A.A.MS.X
        XMASAMX.MM
        X.....XA.A
        S.S.S.S.SS
        .A.A.A.A.A
        ..M.M.M.MM
        .X.X.XMASX


        ....XXMAS.
         .SAMXMS...
          ...S..A...
           ..A.A.MS.X
            XMASAMX.MM
             X.....XA.A
              S.S.S.S.SS
               .A.A.A.A.A
                ..M.M.M.MM
                 .X.X.XMASX
            
    */

    // Column search on line space and skewed line space

    let mut skewed_lines: Vec<String> = [].to_vec();
    let mut reverse_skewed_lines: Vec<String> = [].to_vec();
    let mut skewed_line: String;
    let mut reverse_skewed_line: String;
    let mut reversed_line: String;
    let ncols = lines[0].chars().count();
    for (i, line) in (&lines).iter().enumerate() {
        skewed_line = repeating_char('.', i);
        skewed_line.push_str(line);
        skewed_line.push_str(repeating_char('.',ncols - 1 - i).as_str());

        
        reversed_line = line.chars().rev().collect();
        
        reverse_skewed_line = repeating_char('.', i);
        reverse_skewed_line.push_str(reversed_line.as_str());
        reverse_skewed_line.push_str(repeating_char('.', ncols - 1 - i).as_str());

        skewed_lines.push(skewed_line);
        reverse_skewed_lines.push(reverse_skewed_line);
    }

    // Column search on skewed lines
    for lines_for_column_search in [lines, skewed_lines, reverse_skewed_lines] {
        let mut column: String = "".to_string();
        let mut c: String;
        for (i, _char) in lines_for_column_search[0].chars().enumerate() {
            for line in &lines_for_column_search {
                c = line.chars().nth(i).unwrap().to_string();
                column += &c;
            }
    
            lines_to_search.push(column.clone());
            lines_to_search.push(column.chars().rev().collect());
            column = "".to_string();
        }
    }

      
    let mut acc = 0;
    for line_to_search in &lines_to_search {
        acc += line_to_search.matches("XMAS").count();
    }
 
    // Write to output file
    let mut file = File::create(result).expect("Unable to create file");
    let text: String = format!("Solution to part 1: {}\nSolution to part 2: TO DO", acc.to_string());
    file.write_all(text.as_bytes()).unwrap();
    
}