use std::collections::{HashMap, HashSet};

#[derive(Clone)]
struct Brick {
    id: usize,
    x_low: i32,
    x_high: i32,
    y_low: i32,
    y_high: i32,
    z_low: i32,
    z_high: i32,
    supports: HashSet<usize>,
}

impl Brick {
    fn of_string(s: &str, id: usize) -> Self {
        let (low, high) = s.split_once('~').unwrap();
        let lows = low
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let highs = high
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        Self {
            id,
            x_low: lows[0],
            x_high: highs[0],
            y_low: lows[1],
            y_high: highs[1],
            z_low: lows[2],
            z_high: highs[2],
            supports: HashSet::new(),
        }
    }
}

pub fn main() {
    let mut input = "3,7,269~3,7,270
2,8,240~4,8,240
9,6,123~9,6,124
9,3,30~9,6,30
5,2,165~5,2,166
7,2,294~7,4,294
4,1,265~4,2,265
0,4,173~0,6,173
6,3,179~6,5,179
1,4,247~1,4,249
4,7,211~6,7,211
2,0,133~2,3,133
2,6,179~2,9,179
5,6,295~7,6,295
4,8,237~5,8,237
7,2,153~7,5,153
6,0,274~6,3,274
1,6,247~1,8,247
7,3,290~9,3,290
9,1,177~9,1,179
3,6,176~5,6,176
1,6,92~1,6,95
1,1,101~4,1,101
8,0,161~8,2,161
6,9,198~8,9,198
5,7,163~5,9,163
2,3,232~2,3,232
6,2,38~9,2,38
1,1,86~1,1,89
6,4,275~9,4,275
1,4,112~2,4,112
5,3,130~5,6,130
9,0,170~9,2,170
1,3,73~1,4,73
9,4,208~9,4,210
5,6,142~5,7,142
2,4,141~3,4,141
3,4,86~3,5,86
6,4,233~6,6,233
3,9,2~5,9,2
4,3,194~4,6,194
2,3,171~4,3,171
5,1,36~7,1,36
5,3,184~7,3,184
8,5,121~8,5,123
6,7,48~8,7,48
2,0,115~2,2,115
7,0,2~7,2,2
4,5,139~4,8,139
7,2,127~7,5,127
9,6,9~9,7,9
2,5,84~2,7,84
3,5,168~5,5,168
4,6,144~4,6,148
1,0,137~3,0,137
1,2,224~1,4,224
8,4,2~8,6,2
3,9,169~5,9,169
5,1,148~7,1,148
3,6,171~5,6,171
2,2,207~2,3,207
7,4,218~7,5,218
2,2,229~5,2,229
3,1,232~3,4,232
2,0,269~4,0,269
3,6,195~6,6,195
0,4,24~0,7,24
5,4,67~5,6,67
0,5,95~2,5,95
0,7,116~2,7,116
4,6,51~4,9,51
8,7,215~8,8,215
1,1,20~4,1,20
9,6,33~9,8,33
3,5,271~5,5,271
4,1,139~4,4,139
8,3,39~8,5,39
5,3,28~5,5,28
1,0,257~1,2,257
4,3,73~4,3,75
4,5,278~4,9,278
2,5,4~2,7,4
7,9,117~7,9,117
0,5,48~3,5,48
2,5,74~2,8,74
7,3,223~7,6,223
6,1,277~6,1,277
0,4,220~0,6,220
2,1,33~2,3,33
1,0,6~3,0,6
8,8,217~9,8,217
3,1,161~3,4,161
2,9,180~5,9,180
7,5,208~9,5,208
5,7,103~8,7,103
3,5,2~5,5,2
8,6,185~8,8,185
3,0,81~3,0,84
8,5,9~9,5,9
1,4,223~1,6,223
3,6,113~4,6,113
0,6,105~2,6,105
4,3,168~7,3,168
1,5,102~1,7,102
2,6,243~4,6,243
8,7,273~9,7,273
8,7,296~9,7,296
1,6,285~1,8,285
5,0,97~7,0,97
8,4,119~8,4,121
1,1,175~3,1,175
3,0,77~5,0,77
8,4,4~8,6,4
7,1,154~9,1,154
1,0,111~1,3,111
9,0,105~9,0,105
3,4,65~6,4,65
0,3,175~0,6,175
7,1,260~9,1,260
9,4,270~9,7,270
6,9,169~6,9,171
5,8,189~5,8,191
4,6,108~7,6,108
0,4,54~0,6,54
2,2,105~5,2,105
0,9,114~3,9,114
2,0,211~2,0,214
8,3,118~8,5,118
2,7,239~3,7,239
8,0,211~8,3,211
3,4,51~3,4,54
0,0,89~1,0,89
1,2,227~2,2,227
5,5,268~8,5,268
9,0,77~9,2,77
1,1,119~4,1,119
7,0,122~7,0,124
5,7,12~7,7,12
0,5,93~0,7,93
4,1,112~4,3,112
1,5,221~3,5,221
8,6,193~8,9,193
7,8,168~7,8,170
0,8,174~0,9,174
8,4,301~8,5,301
4,1,250~4,4,250
8,6,63~8,9,63
7,6,159~7,6,162
5,5,174~5,7,174
1,1,149~4,1,149
6,6,101~8,6,101
7,0,235~8,0,235
2,1,147~4,1,147
1,0,158~1,0,160
3,5,197~3,8,197
7,3,43~7,6,43
6,3,121~6,5,121
3,5,233~3,6,233
7,6,239~9,6,239
4,5,15~4,5,17
2,1,209~2,1,210
5,6,198~5,6,200
5,2,3~6,2,3
6,5,3~6,6,3
6,4,228~6,6,228
2,4,138~2,6,138
0,7,127~3,7,127
2,7,199~4,7,199
4,3,242~7,3,242
4,4,61~6,4,61
7,1,70~7,3,70
3,7,49~4,7,49
5,2,246~5,2,248
9,5,64~9,5,66
7,1,202~8,1,202
2,4,85~2,4,87
4,1,113~4,3,113
0,9,156~2,9,156
5,2,230~5,5,230
9,7,196~9,7,199
7,5,304~8,5,304
2,8,91~2,9,91
0,9,262~2,9,262
7,0,196~7,2,196
0,8,109~2,8,109
4,2,255~4,4,255
2,0,279~2,2,279
0,3,240~0,6,240
6,3,272~6,5,272
1,0,13~1,2,13
5,0,109~5,0,111
5,2,92~5,2,92
0,6,236~3,6,236
3,0,3~3,1,3
1,7,261~1,9,261
0,4,151~0,7,151
3,0,133~3,1,133
4,3,167~7,3,167
4,1,273~4,3,273
3,0,98~3,3,98
3,2,252~3,3,252
2,0,122~2,1,122
0,5,239~0,8,239
0,2,171~2,2,171
4,3,175~4,6,175
5,7,45~6,7,45
4,4,160~6,4,160
6,4,270~6,6,270
5,0,150~5,0,153
5,8,96~7,8,96
0,4,227~2,4,227
2,4,15~4,4,15
7,6,271~7,7,271
9,2,173~9,3,173
1,2,242~1,5,242
4,0,96~5,0,96
2,7,100~3,7,100
6,8,8~6,9,8
6,0,32~6,3,32
6,8,242~8,8,242
5,6,39~7,6,39
8,3,192~8,6,192
1,1,174~1,4,174
4,0,152~4,2,152
1,3,156~1,4,156
3,7,236~3,8,236
5,8,11~7,8,11
0,5,131~0,8,131
6,6,236~7,6,236
5,6,69~5,9,69
1,9,123~1,9,126
2,7,97~3,7,97
5,0,99~7,0,99
4,4,12~4,6,12
9,2,6~9,4,6
6,3,4~6,5,4
5,1,72~8,1,72
9,3,236~9,3,238
7,0,217~7,0,217
4,6,214~4,6,216
8,3,119~8,3,120
1,1,113~2,1,113
1,0,136~3,0,136
0,1,8~0,2,8
0,9,151~0,9,152
1,6,8~4,6,8
3,0,238~6,0,238
2,0,207~3,0,207
6,9,103~8,9,103
4,8,261~4,8,264
6,1,264~8,1,264
9,7,130~9,9,130
4,3,133~4,6,133
5,2,34~5,3,34
5,1,245~5,1,245
7,6,158~7,7,158
3,4,135~5,4,135
1,5,111~1,7,111
6,8,100~6,8,102
5,8,160~8,8,160
3,7,60~3,9,60
2,2,209~2,3,209
2,7,14~2,7,14
3,1,200~3,2,200
4,2,99~4,2,99
9,1,14~9,4,14
4,4,26~4,6,26
1,1,6~3,1,6
3,5,79~3,8,79
3,1,214~3,3,214
0,6,170~0,8,170
0,3,41~1,3,41
5,3,296~7,3,296
9,1,81~9,4,81
3,7,238~3,9,238
6,3,234~9,3,234
5,3,22~5,3,24
7,6,268~9,6,268
3,5,171~4,5,171
2,2,136~2,2,139
8,5,194~8,6,194
7,3,124~7,3,125
2,6,151~2,7,151
5,6,157~5,9,157
4,5,174~4,8,174
6,3,73~9,3,73
4,9,244~6,9,244
4,3,120~6,3,120
5,1,97~7,1,97
8,1,89~8,3,89
6,5,93~6,6,93
0,8,173~1,8,173
5,3,187~5,5,187
2,1,146~2,1,146
7,0,199~7,2,199
8,3,169~8,4,169
0,7,134~0,8,134
1,3,75~1,5,75
4,9,201~6,9,201
5,1,100~5,3,100
9,4,241~9,6,241
8,5,16~8,7,16
6,5,123~7,5,123
0,6,96~0,9,96
6,6,186~6,9,186
4,6,124~4,9,124
6,7,152~8,7,152
1,5,6~3,5,6
2,7,87~2,9,87
8,5,169~8,6,169
0,3,27~0,4,27
4,0,217~6,0,217
3,4,137~5,4,137
2,5,241~2,7,241
1,1,217~3,1,217
5,5,232~5,5,233
0,6,63~0,9,63
6,0,110~8,0,110
3,4,114~4,4,114
3,1,166~3,2,166
6,3,219~6,6,219
0,1,229~2,1,229
9,1,173~9,1,174
1,7,166~3,7,166
8,4,98~8,6,98
6,9,247~8,9,247
0,6,168~0,8,168
7,0,38~8,0,38
7,7,194~9,7,194
2,1,143~2,2,143
1,2,156~2,2,156
7,9,61~9,9,61
1,6,90~3,6,90
8,3,167~8,6,167
8,0,305~8,3,305
6,9,71~9,9,71
0,2,30~0,2,32
7,2,215~7,4,215
0,7,54~0,9,54
6,6,26~9,6,26
3,7,212~5,7,212
2,7,141~2,9,141
1,0,132~5,0,132
1,4,136~1,6,136
2,4,140~4,4,140
8,4,213~8,5,213
1,5,109~3,5,109
2,0,117~4,0,117
0,7,266~3,7,266
3,0,241~5,0,241
6,3,68~8,3,68
7,9,13~9,9,13
5,1,93~5,1,95
4,8,46~5,8,46
4,5,216~5,5,216
5,3,87~5,5,87
8,8,104~8,8,106
1,5,104~3,5,104
0,8,245~0,9,245
7,5,289~7,9,289
5,5,8~5,7,8
1,1,83~1,3,83
3,8,150~3,9,150
7,3,284~7,4,284
6,1,261~6,4,261
6,1,181~6,3,181
3,0,86~3,2,86
0,9,203~3,9,203
4,5,203~5,5,203
7,8,197~7,9,197
0,2,250~0,2,253
4,1,227~6,1,227
2,3,210~2,6,210
0,9,142~2,9,142
6,8,195~7,8,195
2,5,215~2,5,218
0,2,168~1,2,168
4,1,221~8,1,221
1,3,248~3,3,248
1,5,94~1,5,94
0,2,117~2,2,117
1,4,215~1,5,215
5,2,4~6,2,4
7,3,244~8,3,244
5,4,236~7,4,236
2,3,258~4,3,258
1,1,166~1,2,166
0,4,219~0,6,219
5,1,229~8,1,229
4,5,214~6,5,214
7,3,80~8,3,80
0,2,239~2,2,239
4,8,49~7,8,49
1,9,14~2,9,14
6,4,262~6,5,262
8,5,108~8,7,108
8,1,40~8,3,40
4,7,156~4,8,156
1,5,38~3,5,38
0,2,182~0,2,184
0,6,60~0,9,60
4,3,31~7,3,31
4,6,22~7,6,22
6,1,137~6,4,137
5,3,140~7,3,140
9,6,165~9,7,165
2,2,27~2,5,27
8,2,72~8,3,72
3,6,53~3,7,53
3,9,100~5,9,100
1,7,175~2,7,175
8,4,40~8,6,40
6,3,281~8,3,281
6,9,4~8,9,4
0,3,223~0,4,223
9,1,41~9,2,41
0,6,247~0,8,247
5,5,69~5,5,69
0,9,241~4,9,241
5,9,114~7,9,114
2,0,242~4,0,242
7,4,304~8,4,304
6,9,5~8,9,5
6,8,235~7,8,235
2,5,47~4,5,47
8,5,171~9,5,171
5,9,74~5,9,77
1,2,144~1,4,144
4,4,60~6,4,60
7,1,108~7,4,108
7,6,265~8,6,265
0,3,138~3,3,138
3,8,265~5,8,265
4,4,149~4,7,149
4,0,149~6,0,149
5,0,135~6,0,135
7,2,117~7,4,117
6,4,208~6,7,208
9,2,80~9,3,80
7,6,1~7,7,1
6,4,126~6,5,126
1,5,272~4,5,272
9,6,103~9,9,103
6,5,127~6,7,127
0,1,28~0,3,28
4,7,145~6,7,145
5,5,15~5,7,15
9,2,196~9,4,196
4,5,277~4,8,277
2,1,34~2,1,37
6,3,66~6,5,66
6,0,3~6,0,5
1,2,15~1,5,15
3,2,117~3,3,117
3,8,109~6,8,109
6,1,99~7,1,99
4,6,23~7,6,23
6,1,151~9,1,151
7,4,91~7,7,91
1,7,182~3,7,182
2,8,148~5,8,148
0,3,20~0,5,20
7,2,247~7,4,247
0,7,118~0,8,118
2,4,237~2,6,237
8,4,299~8,6,299
0,5,134~1,5,134
6,8,179~8,8,179
2,9,74~4,9,74
1,0,11~4,0,11
6,1,291~6,4,291
2,4,142~2,6,142
2,4,199~2,6,199
2,3,30~2,3,31
5,9,120~6,9,120
7,8,55~9,8,55
2,3,132~2,5,132
7,1,250~7,3,250
8,6,6~9,6,6
3,6,276~5,6,276
0,8,17~1,8,17
4,1,23~4,4,23
4,9,73~5,9,73
2,4,90~4,4,90
4,5,89~7,5,89
3,4,156~3,4,158
7,4,156~8,4,156
5,7,110~5,7,113
8,7,255~8,9,255
1,0,99~1,3,99
8,1,10~9,1,10
1,7,99~2,7,99
1,2,241~3,2,241
5,2,27~7,2,27
7,0,205~7,2,205
3,8,81~5,8,81
7,4,171~9,4,171
4,1,242~4,2,242
6,2,85~7,2,85
3,5,51~5,5,51
2,5,73~5,5,73
8,0,195~8,3,195
4,0,266~4,3,266
0,4,106~0,6,106
7,0,12~7,1,12
2,3,158~4,3,158
1,9,121~4,9,121
6,1,4~7,1,4
2,6,112~2,8,112
0,1,114~3,1,114
6,5,73~8,5,73
7,3,185~9,3,185
5,3,144~7,3,144
8,8,188~9,8,188
6,2,140~7,2,140
7,3,266~9,3,266
0,1,211~2,1,211
4,2,115~5,2,115
5,1,244~5,4,244
5,3,162~7,3,162
1,9,55~3,9,55
1,6,139~2,6,139
7,8,241~9,8,241
2,0,95~2,0,96
5,8,292~8,8,292
3,0,151~3,1,151
9,2,199~9,4,199
9,6,64~9,6,67
8,0,103~9,0,103
6,6,287~8,6,287
2,2,29~2,4,29
8,5,80~8,7,80
3,3,189~6,3,189
2,6,1~2,9,1
3,5,163~3,8,163
1,4,23~3,4,23
0,7,167~2,7,167
8,1,11~8,2,11
1,5,20~1,7,20
3,3,12~3,5,12
1,2,233~2,2,233
3,1,95~3,1,97
1,1,182~1,4,182
5,1,216~5,2,216
3,3,8~3,3,9
2,3,123~4,3,123
8,5,37~8,7,37
3,2,88~3,2,89
1,1,228~1,2,228
0,4,158~0,5,158
1,0,90~2,0,90
4,2,194~7,2,194
3,0,104~6,0,104
3,3,177~4,3,177
5,4,282~7,4,282
6,0,37~8,0,37
2,1,100~2,3,100
5,3,190~5,5,190
8,0,81~8,2,81
5,1,81~5,3,81
2,5,112~4,5,112
2,7,185~2,9,185
2,4,57~5,4,57
6,0,120~8,0,120
4,5,10~7,5,10
4,3,118~6,3,118
4,1,69~4,3,69
3,1,122~6,1,122
1,3,22~2,3,22
6,5,34~8,5,34
5,4,29~5,6,29
6,5,37~6,7,37
2,1,90~4,1,90
9,5,121~9,7,121
4,6,68~6,6,68
1,4,111~4,4,111
0,1,156~0,4,156
7,0,10~7,3,10
1,5,49~1,7,49
0,2,120~2,2,120
2,7,251~3,7,251
8,5,294~8,8,294
3,5,84~6,5,84
6,5,150~8,5,150
2,7,80~4,7,80
2,6,6~2,6,7
6,3,253~8,3,253
0,0,238~1,0,238
3,6,112~5,6,112
1,6,10~1,7,10
3,2,3~3,4,3
0,7,142~1,7,142
3,6,158~3,8,158
0,1,167~2,1,167
9,5,166~9,7,166
2,2,276~4,2,276
2,1,232~2,2,232
8,0,121~8,2,121
0,0,23~0,3,23
2,5,130~2,7,130
3,3,164~3,6,164
8,2,79~8,5,79
1,5,43~4,5,43
1,1,104~1,2,104
1,6,282~3,6,282
1,9,99~4,9,99
4,2,192~4,3,192
5,2,262~7,2,262
0,5,159~0,5,161
1,8,145~1,8,145
2,3,91~2,6,91
4,6,85~7,6,85
0,6,110~0,6,113
5,1,294~8,1,294
5,8,246~5,9,246
5,7,109~6,7,109
6,9,111~9,9,111
2,2,87~5,2,87
1,2,103~3,2,103
2,0,168~2,1,168
1,7,251~1,9,251
8,1,144~8,4,144
0,1,241~0,1,244
0,1,5~0,3,5
3,1,143~3,3,143
9,4,158~9,6,158
2,7,169~2,7,171
9,6,195~9,8,195
4,3,166~4,5,166
2,9,63~3,9,63
8,6,38~8,8,38
5,4,101~7,4,101
9,6,69~9,8,69
9,1,13~9,5,13
2,3,269~5,3,269
9,2,124~9,5,124
8,1,7~8,5,7
3,9,17~6,9,17
1,3,211~2,3,211
4,4,88~5,4,88
4,9,14~6,9,14
2,4,244~2,5,244
4,7,188~6,7,188
5,1,163~7,1,163
6,8,105~7,8,105
3,0,235~3,1,235
0,1,238~2,1,238
0,2,105~1,2,105
3,7,170~5,7,170
4,0,72~4,1,72
3,3,273~3,5,273
6,5,184~6,7,184
4,5,217~6,5,217
7,5,112~7,7,112
6,0,34~7,0,34
9,3,118~9,5,118
8,4,173~9,4,173
5,4,218~5,5,218
6,3,191~8,3,191
0,7,232~2,7,232
5,3,76~5,5,76
2,1,117~4,1,117
4,6,71~6,6,71
2,2,24~5,2,24
7,1,84~7,3,84
7,2,44~9,2,44
0,4,260~0,6,260
4,4,81~6,4,81
0,8,143~1,8,143
2,3,212~4,3,212
5,5,167~5,7,167
7,1,35~7,4,35
6,5,44~8,5,44
2,3,197~5,3,197
6,7,214~8,7,214
3,4,223~3,7,223
3,4,35~3,7,35
1,3,199~4,3,199
7,2,169~7,4,169
4,2,8~7,2,8
1,0,79~2,0,79
3,3,46~3,7,46
8,4,12~8,6,12
5,5,235~5,7,235
4,4,5~7,4,5
3,3,88~3,4,88
4,7,11~6,7,11
0,1,123~3,1,123
7,3,255~8,3,255
3,8,101~5,8,101
1,8,166~3,8,166
8,1,259~8,3,259
3,5,227~6,5,227
1,6,88~4,6,88
8,1,42~9,1,42
6,7,142~6,7,142
4,9,10~5,9,10
2,5,10~2,5,11
7,3,165~9,3,165
0,2,2~0,4,2
3,8,92~4,8,92
6,5,42~6,7,42
1,1,245~1,3,245
4,1,35~6,1,35
2,5,196~4,5,196
8,0,236~9,0,236
6,5,110~7,5,110
6,1,102~6,1,105
6,6,216~6,7,216
3,3,34~4,3,34
2,8,266~5,8,266
0,8,126~2,8,126
4,1,150~4,2,150
1,0,21~1,2,21
8,8,195~8,9,195
3,4,152~6,4,152
3,9,153~4,9,153
3,4,285~3,4,287
9,9,15~9,9,15
6,7,231~7,7,231
7,3,81~7,3,83
8,5,182~8,8,182
5,7,5~5,9,5
4,3,174~6,3,174
6,5,182~6,5,182
1,9,265~3,9,265
4,0,15~5,0,15
3,7,95~3,9,95
2,4,17~5,4,17
2,3,254~4,3,254
7,3,219~7,3,221
5,3,208~8,3,208
0,6,277~3,6,277
5,2,163~6,2,163
3,5,98~3,7,98
0,2,247~2,2,247
9,7,280~9,9,280
7,9,137~9,9,137
0,6,135~3,6,135
7,3,261~7,5,261
5,5,4~5,7,4
8,3,105~8,6,105
4,2,35~4,3,35
6,0,72~8,0,72
9,2,277~9,5,277
5,5,238~5,8,238
7,1,100~7,2,100
2,6,173~2,7,173
1,9,117~1,9,119
0,5,17~0,5,18
6,9,138~7,9,138
2,0,116~3,0,116
7,7,155~7,8,155
6,3,11~6,5,11
0,6,134~1,6,134
2,7,113~2,9,113
9,7,220~9,9,220
0,1,226~0,3,226
1,8,258~4,8,258
9,6,297~9,8,297
1,0,102~3,0,102
6,1,107~7,1,107
4,0,130~4,4,130
5,6,155~7,6,155
7,5,232~9,5,232
5,8,299~7,8,299
2,5,240~4,5,240
7,0,213~8,0,213
4,3,173~4,4,173
5,2,79~5,5,79
2,6,253~2,6,253
8,7,102~8,9,102
2,0,76~3,0,76
5,9,191~6,9,191
1,4,58~3,4,58
9,7,300~9,9,300
3,7,153~5,7,153
1,9,20~3,9,20
0,4,5~0,4,7
2,1,253~4,1,253
6,9,7~8,9,7
5,2,200~6,2,200
1,5,110~3,5,110
9,2,200~9,3,200
6,6,73~6,8,73
3,1,24~5,1,24
5,2,280~7,2,280
6,5,238~6,8,238
1,4,91~1,6,91
7,6,111~7,8,111
9,5,129~9,8,129
8,4,104~8,6,104
6,3,123~8,3,123
0,4,11~0,5,11
9,7,124~9,8,124
0,6,157~0,8,157
7,0,150~7,1,150
3,3,71~5,3,71
0,2,258~1,2,258
6,9,189~8,9,189
4,2,126~4,5,126
1,8,14~2,8,14
4,0,97~4,2,97
8,5,240~8,8,240
4,5,165~4,7,165
2,2,230~2,6,230
0,1,174~0,3,174
8,5,126~8,7,126
0,0,93~2,0,93
9,6,275~9,9,275
5,2,161~5,4,161
9,2,167~9,4,167
8,7,196~8,9,196
3,0,107~6,0,107
8,1,5~9,1,5
4,3,128~5,3,128
0,4,279~0,6,279
9,2,207~9,6,207
1,2,203~4,2,203
4,3,213~7,3,213
9,8,57~9,9,57
2,5,30~4,5,30
9,2,269~9,4,269
5,3,123~5,6,123
5,6,106~5,8,106
2,4,82~4,4,82
0,6,257~2,6,257
3,0,49~3,3,49
2,8,21~2,9,21
2,0,13~5,0,13
1,0,81~1,2,81
0,2,107~2,2,107
6,0,1~8,0,1
0,7,11~3,7,11
4,9,249~5,9,249
2,4,248~4,4,248
5,7,9~5,9,9
3,1,88~6,1,88
3,6,111~3,9,111
0,2,141~0,4,141
6,3,113~6,4,113
1,0,95~1,2,95
5,6,164~5,9,164
0,6,261~3,6,261
0,0,241~1,0,241
1,4,244~1,6,244
6,4,230~8,4,230
4,1,224~6,1,224
1,9,279~4,9,279
1,6,106~1,9,106
6,9,9~6,9,11
0,4,9~0,7,9
3,5,183~6,5,183
7,8,46~7,8,46
5,7,234~7,7,234
9,3,168~9,4,168
8,6,81~8,6,82
8,1,8~8,3,8
1,8,140~4,8,140
2,8,88~4,8,88
9,7,60~9,9,60
4,8,177~6,8,177
5,7,50~5,9,50
2,6,124~2,9,124
8,7,1~9,7,1
5,2,20~5,4,20
5,8,98~5,9,98
0,3,216~0,5,216
5,5,81~5,7,81
4,6,15~4,8,15
4,7,94~4,9,94
4,2,109~4,5,109
4,3,134~6,3,134
3,6,84~6,6,84
3,2,148~5,2,148
2,7,253~2,8,253
0,5,213~4,5,213
4,6,167~4,6,168
3,8,28~6,8,28
2,4,278~2,6,278
9,5,106~9,7,106
7,9,95~8,9,95
4,0,88~6,0,88
3,7,241~4,7,241
0,6,222~0,9,222
7,6,165~8,6,165
6,7,100~9,7,100
5,7,240~5,8,240
1,5,60~1,8,60
5,1,7~5,4,7
7,3,286~7,6,286
4,7,214~4,9,214
0,8,172~2,8,172
3,8,120~5,8,120
1,7,148~4,7,148
1,7,13~1,9,13
5,3,25~5,5,25
2,2,197~5,2,197
5,8,236~8,8,236
6,1,157~7,1,157
7,2,231~7,5,231
9,7,11~9,9,11
5,2,131~5,4,131
5,0,155~6,0,155
8,2,113~8,4,113
8,0,69~8,3,69
5,9,194~8,9,194
4,1,271~5,1,271
9,3,203~9,5,203
1,7,176~3,7,176
5,6,118~5,8,118
0,7,128~1,7,128
3,3,90~3,3,91
0,4,153~2,4,153
4,4,243~4,4,245
1,1,24~1,1,26
1,8,59~1,9,59
5,0,147~5,3,147
1,0,208~3,0,208
1,1,237~1,3,237
1,3,252~1,4,252
0,6,155~1,6,155
4,5,4~4,5,6
9,6,160~9,6,162
7,9,16~8,9,16
0,6,154~1,6,154
6,7,255~6,9,255
6,2,158~6,5,158
4,2,241~4,5,241
5,7,116~8,7,116
0,8,136~0,8,136
4,1,66~4,4,66
1,0,157~3,0,157
0,2,177~1,2,177
4,8,146~6,8,146
1,5,140~3,5,140
0,1,22~0,3,22
5,8,43~7,8,43
7,3,32~7,6,32
4,5,136~4,6,136
2,5,9~2,8,9
9,4,127~9,6,127
8,1,84~8,3,84
8,1,87~8,4,87
0,0,237~1,0,237
1,3,239~1,5,239
3,3,116~5,3,116
0,7,146~0,9,146
7,8,163~7,8,165
0,7,263~3,7,263
8,4,242~8,6,242
2,7,152~2,8,152
6,3,111~8,3,111
1,0,248~1,2,248
0,4,71~1,4,71
8,1,302~8,4,302
5,5,132~7,5,132
7,5,209~7,6,209
2,0,74~5,0,74
1,0,51~3,0,51
3,0,236~4,0,236
0,1,46~0,3,46
2,8,256~4,8,256
3,3,56~4,3,56
3,4,64~5,4,64
0,5,140~0,7,140
5,4,291~5,4,293
4,0,100~8,0,100
2,1,207~4,1,207
9,6,210~9,6,212
0,9,98~2,9,98
1,3,260~4,3,260
6,3,178~6,6,178
2,3,20~2,4,20
7,8,52~7,9,52
6,7,110~6,9,110
3,2,226~3,5,226
0,6,11~0,6,13
0,6,287~2,6,287
6,1,101~7,1,101
3,5,200~5,5,200
2,4,75~2,6,75
2,5,281~2,8,281
6,8,300~6,8,301
6,4,115~9,4,115
9,4,63~9,7,63
3,1,165~3,2,165
0,1,245~0,4,245
0,1,47~0,3,47
5,5,31~5,7,31
6,3,147~6,5,147
1,6,150~1,7,150
4,0,216~7,0,216
5,0,277~7,0,277
0,5,103~2,5,103
2,7,272~3,7,272
0,3,176~0,4,176
2,0,32~2,2,32
3,6,52~3,9,52
3,5,116~3,7,116
4,7,140~6,7,140
5,0,40~6,0,40
3,3,52~3,3,54
2,9,143~2,9,145
0,7,90~2,7,90
0,6,166~3,6,166
9,3,292~9,5,292
7,4,292~7,7,292
3,5,173~3,5,174
2,0,150~2,2,150
0,0,78~3,0,78
4,8,267~5,8,267
6,3,258~8,3,258
4,6,27~4,8,27
9,2,212~9,2,214
3,6,246~3,6,246
0,5,109~0,6,109
0,7,227~2,7,227
5,5,95~5,5,96
4,7,179~4,9,179
6,2,5~6,3,5
2,1,141~4,1,141
0,0,154~2,0,154
9,5,29~9,8,29
3,6,156~5,6,156
7,5,156~8,5,156
3,3,5~5,3,5
5,1,162~5,2,162
1,5,17~1,6,17
1,7,249~3,7,249
3,5,70~6,5,70
0,3,137~0,5,137
1,4,155~1,4,155
1,0,234~1,4,234
7,3,76~8,3,76
3,3,191~3,4,191
5,1,90~5,3,90
6,4,280~6,5,280
7,7,232~7,9,232
4,4,142~4,6,142
3,8,261~3,8,263
0,2,139~0,3,139
0,2,254~2,2,254
3,7,201~3,9,201
8,6,129~8,6,129
5,8,188~6,8,188
4,2,191~4,3,191
3,4,283~5,4,283
5,5,152~5,7,152
6,5,146~6,7,146
6,4,127~6,4,128
6,2,43~9,2,43
1,3,249~3,3,249
9,2,82~9,4,82
8,4,157~9,4,157
3,1,51~4,1,51
8,1,153~9,1,153
1,7,199~1,9,199
7,6,182~7,8,182
6,9,147~8,9,147
6,4,95~6,7,95
5,5,40~7,5,40
6,3,105~7,3,105
6,8,191~6,8,194
9,1,3~9,3,3
2,4,176~2,4,176
1,0,7~2,0,7
6,0,218~6,1,218
0,8,223~0,9,223
6,9,148~7,9,148
4,8,296~6,8,296
4,4,289~7,4,289
0,3,42~2,3,42
1,2,23~1,2,25
9,3,116~9,5,116
2,7,230~6,7,230
6,2,203~6,2,205
1,7,287~3,7,287
9,1,74~9,3,74
9,7,136~9,9,136
5,9,70~7,9,70
1,1,125~1,4,125
6,4,97~8,4,97
0,6,99~0,8,99
8,7,277~9,7,277
1,0,87~4,0,87
2,7,235~4,7,235
3,4,39~3,6,39
8,1,158~8,2,158
1,2,97~3,2,97
2,4,48~4,4,48
0,6,52~0,8,52
1,0,8~3,0,8
3,3,68~3,4,68
3,2,33~3,5,33
8,3,19~8,5,19
6,4,193~9,4,193
7,4,250~7,7,250
8,3,147~8,6,147
0,6,57~0,9,57
6,6,19~7,6,19
1,2,3~1,4,3
0,5,15~0,6,15
7,6,2~7,9,2
4,3,95~6,3,95
1,7,81~3,7,81
8,4,211~9,4,211
7,0,232~7,2,232
1,9,196~4,9,196
4,4,154~6,4,154
6,6,97~6,8,97
0,7,244~0,9,244
0,3,248~0,5,248
1,6,19~3,6,19
7,5,265~7,5,266
0,7,50~2,7,50
3,8,293~5,8,293
6,1,184~6,1,187
0,8,3~0,8,5
7,9,141~7,9,144
3,1,269~5,1,269
7,2,264~9,2,264
8,4,131~8,6,131
0,4,72~0,4,74
3,3,154~3,6,154
5,1,235~9,1,235
7,9,292~9,9,292
1,9,58~4,9,58
1,2,154~1,4,154
6,2,278~6,4,278
0,5,22~0,7,22
8,4,232~8,4,233
2,0,14~4,0,14
2,0,138~3,0,138
2,6,117~2,8,117
3,5,107~6,5,107
5,6,107~5,8,107
2,5,247~4,5,247
6,1,146~9,1,146
0,2,179~2,2,179
3,2,128~5,2,128
3,6,211~6,6,211
5,6,17~7,6,17
4,3,103~7,3,103
9,1,210~9,3,210
3,7,90~3,9,90
1,0,141~3,0,141
0,0,7~0,3,7
7,7,168~9,7,168
4,1,93~4,3,93
5,4,115~5,6,115
3,2,263~6,2,263
1,3,40~1,6,40
1,2,141~1,4,141
6,3,143~8,3,143
5,2,214~8,2,214
4,5,92~6,5,92
3,8,143~7,8,143
6,1,176~6,4,176
1,0,152~3,0,152
1,4,70~5,4,70
0,6,149~0,9,149
7,1,120~7,1,122
0,3,6~2,3,6
6,7,253~8,7,253
5,2,107~5,4,107
4,9,102~4,9,104
5,1,231~5,3,231
2,0,206~2,2,206
3,7,147~5,7,147
7,6,156~7,8,156
6,2,207~6,4,207
3,4,160~3,7,160
5,4,100~7,4,100
4,7,231~4,7,234
8,5,298~8,7,298
7,4,206~9,4,206
9,7,132~9,7,133
7,0,288~7,3,288
0,1,257~0,2,257
7,3,217~9,3,217
0,6,114~0,8,114
4,4,197~7,4,197
0,1,142~0,3,142
7,4,45~7,4,47
2,4,174~2,6,174
4,0,95~4,2,95
1,6,250~2,6,250
7,5,15~8,5,15
7,3,264~7,6,264
3,5,76~3,6,76
4,3,156~8,3,156
0,6,254~2,6,254
3,5,276~3,5,279
7,6,57~7,8,57
7,2,156~9,2,156
5,7,33~5,7,34
7,5,18~9,5,18
4,7,150~6,7,150
3,0,92~3,2,92
5,5,9~5,6,9
1,6,172~3,6,172
1,7,224~3,7,224
0,2,164~3,2,164
0,1,44~0,3,44
7,0,119~7,3,119
6,6,40~6,9,40
7,7,94~7,9,94
2,9,16~2,9,19
8,4,103~8,6,103
0,2,243~0,5,243
5,5,273~5,7,273
4,4,159~4,6,159
1,1,15~1,1,18
6,9,119~7,9,119
6,3,8~6,3,9
5,9,167~8,9,167
3,9,193~5,9,193
0,9,153~2,9,153
4,6,269~4,9,269
1,0,109~1,3,109
6,2,39~6,4,39
6,4,98~7,4,98
4,3,163~4,6,163
5,8,47~7,8,47
3,7,276~5,7,276
1,6,231~3,6,231"
        .lines()
        .into_iter()
        .enumerate()
        .map(|(i, s)| Brick::of_string(s, i))
        .collect::<Vec<Brick>>();

    input.sort_by(|a, b| a.z_low.cmp(&b.z_low));
    let mut bricks = input.iter().fold(HashMap::new(), |mut acc, b| {
        acc.insert(b.id, b.clone());
        acc
    });
    let mut occupied: HashMap<(i32, i32, i32), usize> = HashMap::new();
    let mut supported_by: HashMap<usize, HashSet<usize>> = HashMap::new();

    for brick in input.iter_mut() {
        while brick.z_low > 1 {
            let mut supported = false;
            (brick.x_low..(brick.x_high + 1)).for_each(|x| {
                (brick.y_low..(brick.y_high + 1)).for_each(|y| {
                    if let Some(id) = occupied.get(&(x, y, brick.z_low - 1)) {
                        supported_by
                            .entry(brick.id)
                            .and_modify(|h| {
                                h.insert(*id);
                            })
                            .or_insert_with(|| {
                                let mut h = HashSet::new();
                                h.insert(*id);
                                h
                            });
                        bricks.entry(*id).and_modify(|b| {
                            b.supports.insert(brick.id);
                        });
                        supported = true;
                    }
                })
            });
            if supported {
                break;
            }
            brick.z_low -= 1;
            brick.z_high -= 1;
        }

        for x in brick.x_low..(brick.x_high + 1) {
            for y in brick.y_low..(brick.y_high + 1) {
                for z in brick.z_low..(brick.z_high + 1) {
                    occupied.insert((x, y, z), brick.id);
                }
            }
        }
    }
    let part_one = bricks.iter().fold(0, |acc, (_, b)| {
        if b.supports.iter().fold(true, |acc, b_id| {
            supported_by.get(b_id).unwrap().len() > 1 && acc
        }) {
            acc + 1
        } else {
            acc
        }
    });

    println!("part one {part_one}");

    let mut part_two = 0;

    for id in 0..input.len() {
        let mut dropping = HashSet::new();
        dropping.insert(id);
        let mut bricks_to_fall = bricks.get(&id).unwrap().supports.clone();
        while !bricks_to_fall.is_empty() {
            let mut new_bricks_to_fall = HashSet::new();
            for brick_id in bricks_to_fall {
                if supported_by.get(&brick_id).unwrap().is_subset(&dropping) {
                    dropping.insert(brick_id);
                    new_bricks_to_fall.extend(bricks.get(&brick_id).unwrap().supports.iter());
                }
            }
            bricks_to_fall = new_bricks_to_fall;
        }
        part_two += dropping.len() - 1;
    }
    println!("part two {part_two}")
}
