use std::usize;

pub fn dayfour(s: String) {
    partone(s.clone());
    parttwo(s);
    fn partone(s: String) {
        let mut charvec: Vec<Vec<char>> = Vec::new();
        for line in s.lines() {
            charvec.push(line.chars().collect());
        }
        let mut vertical: String = String::new();
        let mut firstdiagonal: String = String::new();
        let mut seconddiagonal: String = String::new();
        for row in 0..charvec[0].len() {
            //Vertical
            for col in 0..charvec.len() {
                vertical += &charvec[col][row].to_string();
            }
            //First diagonal 1/2
            let mut j = 1;
            while row + j < charvec.len() && j < charvec[0].len() {
                firstdiagonal += &charvec[row + j][charvec[0].len() - j].to_string();
                j += 1;
            }
            //Second diagonal 1/2
            j = 0;
            while row + j < charvec.len() && j < charvec[0].len() {
                seconddiagonal += &charvec[row + j][j].to_string();
                j += 1;
            }
            seconddiagonal += &"\n".to_string();
            firstdiagonal += &"\n".to_string();
            vertical += &"\n".to_string();
        }
        for col in 0..charvec[0].len() {
            //First diagonal 2/2
            let mut i = 0;
            while i < col {
                firstdiagonal += &charvec[i][col - i - 1].to_string();
                i += 1;
            }
            //Second diagonal 2/2
            i = 0;
            while col + i < charvec[0].len() {
                seconddiagonal += &charvec[i][col + i].to_string();
                i += 1;
            }
            seconddiagonal += &"\n".to_string();
            firstdiagonal += &"\n".to_string();
        }
        let result = s.matches("XMAS").collect::<Vec<&str>>().len()
            + s.matches("SAMX").collect::<Vec<&str>>().len()
            + vertical.matches("XMAS").collect::<Vec<&str>>().len()
            + vertical.matches("SAMX").collect::<Vec<&str>>().len()
            + firstdiagonal.matches("XMAS").collect::<Vec<&str>>().len()
            + firstdiagonal.matches("SAMX").collect::<Vec<&str>>().len()
            + seconddiagonal.matches("XMAS").collect::<Vec<&str>>().len()
            + seconddiagonal.matches("SAMX").collect::<Vec<&str>>().len();
        println!("{}", result);
    }

    //==========================================================
    //==========================================================
    //==========================================================
    fn parttwo(s: String) {
        let mut initial_pos: Vec<Vec<(usize, usize)>> = Vec::new();
        let mut charvec: Vec<Vec<char>> = Vec::new();
        for line in s.lines() {
            charvec.push(line.chars().collect());
        }
        //============================================
        initial_pos.push(Vec::new());
        initial_pos.push(Vec::new());
        let mut seconddiagonal: String = String::new();
        let mut firstdiagonal: String = String::new();
        for col in 0..charvec[0].len() {
            let mut i = 0;
            while i < col {
                initial_pos[0].push((i, col - i - 1));
                firstdiagonal += &charvec[i][col - i - 1].to_string();
                i += 1;
            }
            i = 0;
            while col + i < charvec[0].len() {
                initial_pos[1].push((i, col + i));
                seconddiagonal += &charvec[i][col + i].to_string();
                i += 1;
            }
            initial_pos[0].push((0, 0));
            firstdiagonal += &".".to_string();
            initial_pos[1].push((0, 0));
            seconddiagonal += &".".to_string();
        }
        //============================================
        for row in 0..charvec.len() {
            let mut j = 0;
            while row + j < charvec.len() && j < charvec[0].len() {
                initial_pos[0].push((row + j, charvec[0].len() - j - 1));
                firstdiagonal += &charvec[row + j][charvec[0].len() - j - 1].to_string();
                j += 1;
            }
            j = 0;
            while row + j < charvec.len() && j < charvec[0].len() {
                initial_pos[1].push((row + j, j));
                seconddiagonal += &charvec[row + j][j].to_string();
                j += 1;
            }
            initial_pos[1].push((0, 0));
            seconddiagonal += &".".to_string();
            initial_pos[0].push((0, 0));
            firstdiagonal += &".".to_string();
        }
        let mut firstdiagonalpos: Vec<(usize, usize)> = Vec::new();
        let mut seconddiagonalpos: Vec<(usize, usize)> = Vec::new();
        let mut indices1: Vec<Vec<(usize, &str)>> = Vec::new();
        let mut indices2: Vec<Vec<(usize, &str)>> = Vec::new();
        indices1.push(firstdiagonal.match_indices("MAS").collect());
        indices1.push(firstdiagonal.match_indices("SAM").collect());
        indices2.push(seconddiagonal.match_indices("MAS").collect());
        indices2.push(seconddiagonal.match_indices("SAM").collect());
        for vec in indices1 {
            for item in vec {
                firstdiagonalpos.push(initial_pos[0][item.0 + 1]);
            }
        }
        for vec in indices2 {
            for item in vec {
                seconddiagonalpos.push(initial_pos[1][item.0 + 1]);
            }
        }
        let mut result = 0;
        for item in &firstdiagonalpos {
            if seconddiagonalpos.contains(&item) {
                result += 1;
            }
        }
        println!("{}", result);
    }
}
