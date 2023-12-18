use std::collections::HashSet;

pub fn main() {
    let input = "L 7 (#2ac8e2)
D 5 (#14b771)
L 14 (#302602)
U 5 (#14b773)
L 4 (#55b782)
D 6 (#535223)
L 3 (#3bf082)
D 8 (#7be433)
L 9 (#4c1060)
D 12 (#3a69b3)
L 4 (#5f1760)
D 6 (#6832c3)
L 6 (#2f0d60)
U 3 (#0f6c51)
L 5 (#2a1042)
U 7 (#464ee1)
R 5 (#2a1040)
U 5 (#4ce141)
L 2 (#1261c0)
U 11 (#1a05f3)
L 3 (#22a220)
U 6 (#6f3d13)
L 6 (#677110)
D 6 (#38ccd3)
L 6 (#1a3370)
D 3 (#4f3b43)
L 12 (#0086e0)
D 3 (#68f4b1)
L 4 (#5c97f0)
D 5 (#4bee81)
R 8 (#556760)
U 5 (#447f53)
R 3 (#2a7580)
D 5 (#7063e3)
R 7 (#2c84b0)
D 9 (#193141)
L 7 (#1ae260)
D 10 (#4e32c1)
L 3 (#4f4a70)
U 10 (#356251)
L 8 (#7ea0b2)
D 4 (#580771)
R 4 (#7ea0b0)
D 10 (#002e51)
L 12 (#0f93b0)
D 5 (#024911)
L 9 (#16ca20)
U 2 (#14e323)
L 3 (#77c5d0)
U 7 (#4db283)
L 8 (#7b4722)
U 5 (#1f8db3)
R 8 (#0a5fd2)
U 3 (#3dd443)
L 11 (#5ad430)
U 11 (#33f903)
R 5 (#5ad432)
D 7 (#1f0493)
R 14 (#4e8c52)
U 7 (#28bcc3)
R 3 (#4f1792)
U 5 (#31ad71)
L 11 (#196172)
U 6 (#6707d1)
L 11 (#556db2)
U 3 (#6707d3)
L 6 (#457dc2)
U 3 (#31ad73)
L 8 (#01d4d2)
U 4 (#525cb3)
L 6 (#225382)
U 12 (#0fc663)
L 4 (#701292)
U 5 (#0fc661)
L 7 (#0398a2)
U 8 (#525cb1)
L 2 (#46ac42)
U 5 (#6c89c3)
L 8 (#320b70)
D 3 (#319c33)
L 5 (#7b0110)
D 11 (#319c31)
R 5 (#4286b0)
D 4 (#2db113)
L 8 (#806100)
U 14 (#5c7523)
L 4 (#1a96c0)
U 8 (#40b8d3)
R 8 (#5bab20)
U 6 (#43c6f3)
L 8 (#3a4b90)
U 5 (#3e3133)
L 3 (#679780)
U 9 (#584113)
L 8 (#559c02)
U 7 (#60bf53)
L 4 (#5aca62)
U 8 (#4d0163)
R 9 (#2237e2)
D 5 (#106f13)
R 12 (#5c2362)
D 8 (#80b7d3)
R 7 (#26b6b2)
D 6 (#08d3b3)
R 14 (#59afa2)
U 4 (#241333)
R 9 (#0c0eb2)
U 3 (#28a8c3)
R 4 (#47b022)
U 12 (#279703)
R 7 (#47b020)
U 5 (#4e01a3)
L 5 (#6c4722)
U 5 (#607023)
R 5 (#609b62)
U 5 (#6ec7a3)
L 9 (#152722)
U 6 (#022cf1)
L 14 (#481a02)
U 5 (#7ce951)
L 5 (#4316f2)
U 6 (#0f1061)
L 2 (#2b14e2)
U 3 (#3e10c1)
L 12 (#435d72)
U 2 (#6d75e3)
L 8 (#4a4142)
U 8 (#5ec183)
L 12 (#5a9f02)
U 8 (#390f33)
R 8 (#655e52)
U 11 (#757e23)
R 10 (#031912)
U 7 (#10ea33)
R 14 (#4a55c0)
D 6 (#1d2fc3)
R 9 (#7aa260)
D 12 (#07c993)
R 9 (#315c00)
U 8 (#88b083)
R 5 (#52fb50)
U 5 (#2b4ed3)
L 9 (#428cf0)
U 6 (#0225f3)
R 9 (#36f490)
U 6 (#0225f1)
R 7 (#595110)
D 4 (#374603)
L 4 (#44efe0)
D 6 (#2b1b03)
R 4 (#664590)
D 7 (#2b1b01)
R 5 (#1bfe40)
U 6 (#2c7a81)
R 2 (#3bc8f0)
U 9 (#416101)
R 10 (#003f10)
D 7 (#3a4a51)
R 2 (#003f12)
D 5 (#4ba511)
R 3 (#5190e0)
D 3 (#400191)
R 12 (#1a09a0)
D 8 (#3aef23)
R 11 (#0cb780)
D 10 (#6a6393)
R 9 (#561c80)
U 10 (#124da3)
R 6 (#2b0af0)
U 8 (#563841)
R 12 (#73b520)
U 3 (#199441)
R 5 (#73b522)
U 4 (#47d3d1)
R 4 (#70a9a0)
U 8 (#18f861)
L 4 (#18de92)
U 3 (#5bc8a1)
L 12 (#59e082)
U 2 (#4aacb1)
R 12 (#0b5a82)
U 6 (#708aa1)
L 5 (#3902b2)
U 5 (#171153)
R 5 (#3745f2)
U 5 (#081ad3)
R 5 (#45f1c0)
D 8 (#527f53)
R 6 (#45f1c2)
D 6 (#47e013)
L 6 (#3745f0)
D 4 (#01abd3)
R 8 (#51f1e2)
D 8 (#6aa421)
L 5 (#6114f2)
D 7 (#786041)
R 5 (#31b050)
D 3 (#4f62d1)
R 10 (#5ba7a0)
U 8 (#14c261)
R 4 (#2d27d0)
U 7 (#2cd791)
R 4 (#2db660)
U 5 (#7a4801)
R 9 (#19e300)
U 5 (#17adf1)
L 10 (#19e302)
U 10 (#63c5a1)
L 2 (#6881e0)
U 6 (#2ba711)
R 12 (#148ef0)
U 6 (#1a9ba3)
R 9 (#0597f0)
D 8 (#4f1ee3)
R 10 (#4dd080)
D 4 (#33a753)
L 4 (#3fb6d0)
D 3 (#1ebdb3)
R 10 (#0b0872)
D 8 (#0e1853)
L 10 (#3547f2)
D 5 (#21e0e3)
R 4 (#52cee2)
D 5 (#3b66c3)
L 10 (#5b91a0)
D 6 (#26bac3)
R 11 (#222710)
D 2 (#2d5211)
R 7 (#44f020)
U 12 (#505ed1)
R 3 (#358270)
D 12 (#505ed3)
R 5 (#16c910)
U 7 (#36a931)
R 12 (#849ea0)
U 6 (#36a933)
R 10 (#1e8c20)
U 10 (#55b611)
L 11 (#2cded0)
D 5 (#5f5281)
L 12 (#00e5b2)
U 5 (#4a2981)
L 3 (#379182)
U 8 (#3e19a1)
R 6 (#644222)
U 9 (#0f4fe1)
R 6 (#570b02)
U 7 (#551e33)
R 7 (#07ea32)
D 5 (#35bb93)
R 4 (#0a12d2)
D 4 (#0cb943)
L 4 (#3b7242)
D 7 (#28d521)
R 7 (#2d9d60)
U 10 (#7332e1)
R 3 (#44d090)
D 4 (#7baf51)
R 7 (#01b8e0)
U 4 (#1b4e01)
R 6 (#0b27b2)
U 3 (#642a61)
L 4 (#0b27b0)
U 6 (#21beb1)
R 4 (#64df80)
U 7 (#08c4b3)
R 10 (#626c00)
D 6 (#34e6b3)
R 5 (#06b262)
U 4 (#6e38c3)
R 5 (#06b260)
D 4 (#710243)
R 5 (#05c140)
D 4 (#1e5571)
L 15 (#54bee0)
D 6 (#0a4df1)
R 6 (#1f4bd0)
U 3 (#0a4df3)
R 13 (#4057c0)
U 11 (#701d41)
R 6 (#16f352)
U 4 (#449eb3)
L 15 (#69f132)
U 6 (#449eb1)
R 15 (#337df2)
U 6 (#2a2871)
L 6 (#3946a0)
U 6 (#112121)
R 15 (#107a70)
D 5 (#66a2e1)
R 2 (#4b4f50)
D 12 (#277781)
R 8 (#32cc40)
D 3 (#1184e1)
R 5 (#439160)
D 8 (#43e2d1)
R 3 (#496410)
D 2 (#0fb6d1)
R 9 (#12a250)
D 12 (#2abc23)
R 4 (#123e22)
D 15 (#834853)
R 4 (#123e20)
U 4 (#453473)
R 8 (#0281c0)
U 12 (#27c921)
R 7 (#543410)
U 3 (#2cd3a3)
R 5 (#3bdd30)
U 3 (#73d693)
R 13 (#2bfb40)
U 6 (#3f93d1)
L 13 (#1f66f0)
U 8 (#3e3911)
R 6 (#1f66f2)
U 10 (#137901)
L 8 (#3ea9b0)
U 3 (#0f6451)
L 10 (#2ea050)
U 8 (#450f13)
R 14 (#1758c2)
U 4 (#21cd73)
R 2 (#1758c0)
U 9 (#39b8e3)
R 14 (#0255c0)
D 4 (#290d23)
R 4 (#0ad710)
D 9 (#0a59f3)
R 8 (#3b9950)
D 4 (#7bae53)
R 11 (#3e54a0)
D 8 (#753163)
R 9 (#4dc1b0)
D 6 (#2c5541)
L 2 (#564600)
D 13 (#292593)
L 8 (#5b16b0)
D 4 (#292591)
L 4 (#21f3f0)
U 4 (#2c5543)
L 8 (#0bafd0)
D 4 (#4cda43)
L 10 (#6241a0)
D 5 (#5e31a3)
R 10 (#1c5f90)
D 3 (#4e18a1)
R 7 (#2445d0)
D 9 (#24fc71)
R 6 (#299cc0)
D 6 (#37a491)
R 7 (#299cc2)
U 6 (#43e8d1)
R 10 (#2445d2)
U 9 (#410ad1)
R 10 (#67e350)
U 8 (#5d9801)
R 9 (#12bd80)
D 8 (#30b691)
R 15 (#2ebe00)
D 2 (#6062d1)
R 5 (#113860)
D 13 (#55fa81)
R 8 (#657e80)
U 11 (#6be1c1)
R 9 (#3d83c0)
U 2 (#83eee3)
R 11 (#0755a0)
U 10 (#389d53)
R 6 (#669170)
U 10 (#246853)
R 13 (#26ef30)
D 10 (#362743)
R 6 (#0a7380)
D 8 (#0b2353)
R 8 (#370cd0)
D 10 (#2d3c43)
R 14 (#72d862)
D 8 (#5dc733)
L 5 (#72d860)
D 8 (#034b23)
L 6 (#1d5e70)
D 7 (#163871)
L 4 (#085d00)
D 9 (#5c3901)
R 3 (#5c2d00)
D 8 (#1eead1)
R 12 (#189f80)
D 4 (#12f0c1)
R 5 (#612270)
D 3 (#24db21)
R 9 (#120470)
D 12 (#645371)
R 3 (#4c8030)
D 11 (#28ed73)
R 3 (#491000)
D 10 (#3e0563)
R 8 (#181d10)
D 6 (#139513)
R 6 (#171f80)
D 3 (#467203)
R 7 (#171f82)
D 12 (#3f62a3)
R 5 (#181d12)
D 4 (#2d1f13)
L 9 (#333730)
D 2 (#2eef11)
L 10 (#84f660)
D 4 (#386021)
L 3 (#491a90)
D 7 (#849781)
R 6 (#491a92)
D 3 (#2e2731)
L 6 (#3403a0)
D 6 (#588431)
L 10 (#6dde22)
D 11 (#4b8081)
L 4 (#0770b2)
D 4 (#525a71)
R 7 (#106a92)
D 10 (#12b753)
R 4 (#06ce12)
U 10 (#2da5b3)
R 5 (#7eb4d2)
D 6 (#2da5b1)
R 3 (#21deb2)
D 8 (#12b751)
R 12 (#333f42)
D 12 (#4df5d1)
L 7 (#1e5502)
D 3 (#3b11d1)
L 8 (#2add42)
D 5 (#2a2461)
L 6 (#2add40)
D 7 (#3332c1)
L 3 (#0e2cd2)
U 7 (#756e81)
L 7 (#4756b2)
D 5 (#4d0ca1)
L 3 (#56ef42)
D 11 (#3d05a1)
L 4 (#46a2c2)
U 6 (#5bdcb3)
L 5 (#2e3fe2)
U 11 (#781b83)
R 5 (#4f8f62)
U 3 (#21dae3)
L 3 (#3e9a92)
U 2 (#1add33)
L 9 (#3732e2)
D 12 (#07f113)
L 2 (#0c88b2)
D 10 (#853513)
L 8 (#0c88b0)
D 8 (#091c53)
L 4 (#326e32)
D 3 (#2a2593)
R 4 (#642632)
D 13 (#6a4c83)
L 4 (#6a1472)
D 6 (#082b33)
L 3 (#47d362)
D 8 (#48ba61)
L 7 (#300792)
D 5 (#6148c1)
L 8 (#0100d0)
U 11 (#20efd3)
L 6 (#377920)
D 11 (#7895f3)
L 4 (#3a8d50)
D 3 (#7895f1)
L 12 (#26a8b0)
D 7 (#20efd1)
L 7 (#0e6f20)
D 4 (#6390c1)
L 9 (#03cd40)
U 8 (#0ffa71)
L 3 (#151ed2)
U 12 (#147cd3)
L 7 (#843392)
U 6 (#147cd1)
R 10 (#1299f2)
U 11 (#0736e1)
R 11 (#654672)
U 8 (#2465b3)
L 4 (#38ffb0)
D 4 (#0b0b53)
L 13 (#2a0df2)
U 4 (#72d2c3)
L 4 (#2a0df0)
U 3 (#0f7b73)
L 12 (#38ffb2)
U 2 (#2a4ba3)
L 11 (#66dd22)
D 3 (#2577b1)
L 5 (#6a82b2)
D 3 (#516653)
L 7 (#093060)
D 12 (#79ad63)
L 6 (#093062)
D 5 (#348f83)
L 4 (#83e322)
U 8 (#0da4d3)
L 4 (#251e42)
D 6 (#356721)
L 15 (#060892)
U 6 (#304a61)
L 3 (#226e72)
U 5 (#021bb3)
R 9 (#7123c2)
U 4 (#021bb1)
R 13 (#1a4822)
U 6 (#304a63)
L 3 (#157e22)
U 14 (#56fb41)
R 7 (#4a75b0)
U 7 (#4578a1)
R 6 (#7eeb50)
U 6 (#3b6d01)
L 11 (#0ba9e2)
U 5 (#8a0ba1)
R 11 (#3ac130)
U 6 (#3faf21)
R 6 (#6a9062)
D 7 (#3460e1)
R 15 (#6a9060)
D 3 (#421101)
L 15 (#3838a0)
D 7 (#58cf13)
R 6 (#715670)
D 7 (#6b3d83)
R 4 (#715672)
D 10 (#366933)
R 8 (#591620)
U 10 (#2f6453)
R 4 (#8295e0)
U 10 (#3cb3f1)
R 9 (#117f00)
U 10 (#497781)
R 5 (#1ce3c2)
U 6 (#14c9a1)
L 13 (#1ce3c0)
U 4 (#3060a1)
L 8 (#2f4880)
U 5 (#3b2431)
L 4 (#2f4882)
U 12 (#236031)
L 7 (#251ac0)
D 5 (#0052b1)
L 4 (#3d5a30)
D 4 (#372e11)
L 12 (#58bf50)
D 10 (#5db081)
L 2 (#1582a2)
D 4 (#60b921)
L 7 (#39fe02)
D 8 (#657e23)
L 3 (#291c72)
D 7 (#657e21)
L 2 (#393a72)
D 7 (#3196a1)
L 9 (#10c4f2)
D 8 (#650ba1)
L 10 (#052452)
D 4 (#0980a1)
L 13 (#301942)
D 10 (#18a341)
L 4 (#60a8f2)
D 10 (#73c9b1)
R 6 (#006630)
D 6 (#0c04c1)
R 5 (#35f0f0)
U 6 (#7db103)
R 7 (#34c280)
D 9 (#7db101)
L 6 (#25a890)
D 3 (#1bf0a1)
R 12 (#17e9c2)
D 8 (#018d01)
L 12 (#7c9bc2)
D 6 (#5e24b3)
L 4 (#0c6ce2)
U 5 (#4fa793)
L 4 (#5a26b2)
U 7 (#14b5d3)
R 4 (#330d12)
U 5 (#1b3311)
L 8 (#20c112)
D 9 (#1c8ed1)
L 10 (#0f11d2)
U 11 (#5f2d41)
R 4 (#76d4f2)
U 11 (#2b92f1)
L 4 (#184052)
U 8 (#63a4f3)
L 6 (#24fea2)
U 3 (#4fc613)
R 6 (#453b70)
U 7 (#52e7c3)
L 3 (#307370)
U 10 (#1edca3)
L 7 (#75aee2)
D 3 (#1d4c43)
L 3 (#6e3de2)
U 13 (#10edc3)
L 8 (#293ec2)
D 13 (#68f773)
L 8 (#373872)
D 6 (#6a7d73)
L 3 (#5f5212)
D 5 (#649e01)
R 6 (#53fed2)
D 6 (#6bc451)
R 4 (#04d680)
U 6 (#25e6f1)
R 8 (#1e60d0)
D 3 (#456061)
R 4 (#1e03f0)
D 6 (#398451)
L 11 (#648390)
D 2 (#15d5f1)
L 7 (#369602)
D 10 (#5054a1)
L 7 (#53bcc2)
U 15 (#5054a3)
L 5 (#0fd012)
D 15 (#2915a1)
L 4 (#0b9c02)
D 4 (#0f1fd1)
R 11 (#79edf2)
D 5 (#63a4f1)
R 7 (#011ec2)
D 5 (#3452d1)
R 8 (#2ccf12)
U 5 (#790d01)
R 8 (#3f7272)
D 8 (#06e261)
L 7 (#28e202)
D 15 (#4c0ee1)
L 3 (#1161b2)
U 15 (#433e91)
L 6 (#74d6b0)
D 5 (#45c0f1)
L 14 (#74d6b2)
D 7 (#339011)
L 3 (#038fa2)
U 2 (#2a3983)
L 10 (#06bc22)
U 3 (#10e1f3)
L 10 (#27a8d2)
U 9 (#19d373)
L 4 (#748b22)
D 6 (#65bbe3)
L 9 (#04b122)
D 4 (#3d31f1)
L 13 (#010b22)
U 10 (#05d651)
L 4 (#74a772)
D 14 (#05d653)
L 8 (#14e752)
U 3 (#4534a1)
L 9 (#8a99e0)
U 11 (#0e0ab1)
R 5 (#421542)
U 7 (#2a3981)
R 8 (#6c1e52)
U 6 (#1d91e1)
L 13 (#4a4082)
U 3 (#3e5c83)
R 6 (#0ebd62)
U 10 (#21a223)
L 5 (#153ec2)
U 6 (#502eb3)
R 5 (#1299a2)
U 13 (#461143)
L 6 (#691cb2)
U 3 (#1d6533)
L 4 (#495722)
U 2 (#3c61e3)
L 11 (#0cc8b0)
D 4 (#1fd653)
L 7 (#0cc8b2)
D 5 (#340273)
R 7 (#2c7ec2)
D 8 (#699123)
L 4 (#2c7ec0)
D 2 (#69da23)
L 6 (#39c2c2)
U 15 (#18e693)
L 5 (#3d0762)
U 4 (#043ac3)
L 8 (#49ca22)
U 7 (#057783)
L 6 (#57e6a2)
U 12 (#2d7e63)"
        .lines()
        .map(|s| s.split_whitespace().collect::<Vec<&str>>())
        .map(|v| {
            let u = v[1].parse::<i64>().unwrap_or(0);
            (v[0].chars().collect::<Vec<char>>()[0], u, v[2])
        })
        .collect::<Vec<(char, i64, &str)>>();

    let (input, p2_input) = input
        .iter()
        .fold((Vec::new(), Vec::new()), |(mut v1, mut v2), t| {
            v1.push((t.0, t.1));
            v2.push(t.2);
            (v1, v2)
        });

    let p2_input = p2_input
        .iter()
        .map(|s| {
            let mut s = s.chars().filter(|c| c.is_digit(16)).collect::<Vec<char>>();
            let d = match s.pop().unwrap() {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                _ => 'U',
            };

            let s = s.iter().fold("".to_string(), |mut acc, c| {
                acc.push(*c);
                acc
            });
            let i = i64::from_str_radix(&s, 16).unwrap();
            (d, i)
        })
        .collect::<Vec<(char, i64)>>();

    for input in [input, p2_input].iter() {
        let mut cur_row = 0;
        let mut cur_col = 0;
        let mut highest_row = i64::MIN;
        let mut lowest_row = i64::MAX;
        let mut walls: Vec<(i64, i64, i64)> = Vec::new();
        let mut bottom_corners: HashSet<(i64, i64)> = HashSet::new();
        let mut horizontal_walls: HashSet<(i64, i64, i64)> = HashSet::new();
        let mut total = 0;
        for (r, v) in input.iter() {
            total += v;
            match r {
                'U' => bottom_corners.insert((cur_row, cur_col)),
                'D' => bottom_corners.insert((cur_row + v, cur_col)),
                'L' => horizontal_walls.insert((cur_row, cur_col - v, cur_col)),
                _ => horizontal_walls.insert((cur_row, cur_col, cur_col + v)),
            };
            match r {
                'U' => walls.push((cur_col, cur_row - v, cur_row)),
                'D' => walls.push((cur_col, cur_row, cur_row + v)),
                _ => (),
            }
            (cur_row, cur_col) = match r {
                'U' => (cur_row - v, cur_col),
                'D' => (cur_row + v, cur_col),
                'L' => (cur_row, cur_col - v),
                _ => (cur_row, cur_col + v),
            };
            highest_row = highest_row.max(cur_row);
            lowest_row = lowest_row.min(cur_row);
        }

        walls.sort_by(|a, b| a.2.cmp(&b.2));
        for row in lowest_row..highest_row {
            let mut in_the_loop = false;
            let mut cols: Vec<(i64, bool)> = Vec::new();
            for (col, top, bottom) in &walls {
                if top <= &row && bottom >= &row {
                    cols.push((*col, bottom_corners.contains(&(row, *col))));
                }
            }
            cols.sort_by(|a, b| a.0.cmp(&b.0));
            for i in 0..(cols.len() - 1) {
                let (first, bottom_corner) = cols[i];
                if !bottom_corner {
                    in_the_loop = !in_the_loop;
                }
                let (second, _) = cols[i + 1];
                let is_horizontal_wall = horizontal_walls.contains(&(row, first, second));
                if in_the_loop && !is_horizontal_wall {
                    total += second - first - 1;
                }
            }
        }
        println!("total {total}");
    }
}
