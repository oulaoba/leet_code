use std::collections::VecDeque;

fn main() {
    println!("sum_subarray_mins");
}
pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = arr.len();
    let mut ans: i64 = 0;
    let mut left = vec![-1; n];
    let mut count = 0;

    for index in 0..n {
        let current = arr[index];
        for inner_index in (0..index).rev() {
            if arr[inner_index] <= current {
                left[index] = inner_index as i32;
                count += 1;
                break;
            }
        }
    }

    let mut right = vec![n as i32; n];
    for index in (0..n).rev() {
        let current = arr[index];
        for inner_index in (index + 1)..n {
            if arr[inner_index] < current {
                right[index] = inner_index as i32;
                count += 1;
                break;
            }
        }
    }
    for i in 0..n {
        ans +=
            ((((right[i] - (i as i32)) * ((i as i32) - left[i])) as i64) * (arr[i] as i64)) % MOD;
        ans %= MOD;
        count += 1;
    }
    println!("count:{count}");
    ans as i32
}

pub fn sum_subarray_mins1(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut left = vec![-1; n];
    let mut right = vec![n as i32; n];
    let mut stk: VecDeque<usize> = VecDeque::new();
    let mut count = 0;
    for i in 0..n {
        while !stk.is_empty() && arr[*stk.back().unwrap()] >= arr[i] {
            stk.pop_back();
            count += 1;
        }
        if let Some(&top) = stk.back() {
            left[i] = top as i32;
        }
        stk.push_back(i);
    }

    stk.clear();
    for i in (0..n).rev() {
        while !stk.is_empty() && arr[*stk.back().unwrap()] > arr[i] {
            stk.pop_back();
            count += 1;
        }
        if let Some(&top) = stk.back() {
            right[i] = top as i32;
        }
        stk.push_back(i);
    }

    const MOD: i64 = 1_000_000_007;
    let mut ans: i64 = 0;
    for i in 0..n {
        ans +=
            ((((right[i] - (i as i32)) * ((i as i32) - left[i])) as i64) * (arr[i] as i64)) % MOD;
        ans %= MOD;
        count += 1;
    }
    println!("count:{count}");
    ans as i32
}

#[test]
fn test() {
    let arr = vec![3, 1, 2, 4];
    let ans = sum_subarray_mins(arr);
    assert_eq!(17, ans);
}

#[test]
fn test1() {
    let arr = vec![11, 81, 94, 43, 3];
    let ans = sum_subarray_mins(arr);
    assert_eq!(444, ans);
}

#[test]
fn test2() {
    let arr = vec![71, 55, 82, 55];
    let ans = sum_subarray_mins(arr);
    assert_eq!(593, ans);
}

#[test]
fn test4() {
    let arr = vec![
        29959, 29867, 29822, 29704, 29676, 29650, 29577, 29488, 29286, 29255, 29232, 29207, 29071,
        29034, 28925, 28849, 28731, 28693, 28624, 28606, 28591, 28397, 28357, 28308, 28295, 28210,
        28119, 28090, 28004, 27903, 27845, 27830, 27777, 27736, 27640, 27540, 27506, 27428, 27341,
        27308, 27182, 27152, 27122, 27029, 26928, 26872, 26796, 26765, 26663, 26597, 26580, 26530,
        26498, 26475, 26436, 26406, 26382, 26312, 26213, 26134, 26088, 26025, 25943, 25912, 25875,
        25845, 25810, 25702, 25638, 25614, 25531, 25524, 25488, 25470, 25444, 25402, 25283, 25262,
        25121, 24988, 24958, 24886, 24769, 24697, 24635, 24595, 24490, 24456, 24453, 24346, 24313,
        24248, 24200, 24148, 24107, 24052, 24044, 24021, 23970, 23908, 23897, 23835, 23752, 23741,
        23714, 23661, 23596, 23545, 23509, 23470, 23439, 23409, 23350, 23215, 23166, 23155, 23100,
        23024, 22923, 22825, 22793, 22627, 22613, 22536, 22450, 22383, 22312, 22268, 22205, 22175,
        22136, 22028, 21971, 21900, 21824, 21769, 21726, 21583, 21546, 21513, 21494, 21428, 21327,
        21264, 21254, 21174, 21140, 21112, 21000, 20921, 20902, 20830, 20817, 20783, 20735, 20657,
        20616, 20573, 20485, 20378, 20363, 20305, 20259, 20210, 20114, 20002, 19846, 19785, 19747,
        19667, 19645, 19622, 19610, 19580, 19542, 19516, 19454, 19392, 19310, 19277, 19194, 19131,
        19090, 19004, 18883, 18845, 18791, 18781, 18668, 18591, 18518, 18475, 18368, 18331, 18310,
        18287, 18217, 18114, 18092, 18048, 17990, 17964, 17912, 17836, 17740, 17704, 17630, 17613,
        17573, 17428, 17356, 17341, 17300, 17260, 17180, 17174, 17126, 17071, 17041, 16866, 16850,
        16828, 16672, 16618, 16577, 16499, 16407, 16357, 16318, 16293, 16202, 16150, 16075, 16041,
        15948, 15921, 15844, 15843, 15785, 15764, 15668, 15626, 15579, 15473, 15387, 15255, 15190,
        15139, 15062, 14996, 14954, 14918, 14907, 14902, 14867, 14851, 14817, 14799, 14751, 14720,
        14536, 14506, 14474, 14353, 14303, 14280, 14185, 14107, 14012, 13932, 13858, 13781, 13585,
        13563, 13533, 13451, 13412, 13362, 13249, 13208, 13181, 13064, 13037, 12961, 12926, 12892,
        12786, 12731, 12611, 12573, 12506, 12502, 12496, 12470, 12443, 12370, 12262, 12182, 12153,
        12069, 12000, 11847, 11806, 11781, 11708, 11687, 11593, 11550, 11445, 11372, 11329, 11308,
        11291, 11268, 11241, 11191, 11027, 10982, 10879, 10862, 10776, 10695, 10603, 10502, 10464,
        10350, 10338, 10305, 10273, 10176, 10124, 10094, 10038, 9953, 9935, 9812, 9786, 9743, 9728,
        9508, 9472, 9383, 9349, 9236, 9215, 9130, 9124, 9042, 9008, 8988, 8901, 8833, 8809, 8780,
        8716, 8580, 8462, 8334, 8321, 8305, 8280, 8257, 8246, 8137, 8077, 8043, 8016, 7984, 7955,
        7927, 7906, 7746, 7663, 7653, 7572, 7542, 7530, 7489, 7420, 7390, 7361, 7337, 7245, 7210,
        7188, 7175, 7096, 6898, 6846, 6745, 6675, 6569, 6478, 6427, 6363, 6284, 6260, 6243, 6206,
        6154, 6135, 6078, 6061, 6017, 5995, 5917, 5863, 5836, 5793, 5763, 5743, 5678, 5572, 5532,
        5459, 5384, 5341, 5299, 5251, 5231, 4995, 4933, 4861, 4740, 4672, 4625, 4496, 4445, 4361,
        4282, 4215, 4135, 4097, 4028, 3917, 3862, 3711, 3553, 3498, 3410, 3388, 3384, 3288, 3279,
        3244, 3221, 3181, 3171, 3150, 3060, 3035, 2975, 2965, 2834, 2760, 2637, 2584, 2533, 2440,
        2383, 2311, 2285, 2255, 2211, 2192, 2121, 2054, 2010, 1964, 1850, 1724, 1642, 1577, 1411,
        1409, 1332, 1296, 1265, 1256, 1220, 1195, 937, 903, 880, 811, 739, 720, 650, 609, 547, 533,
        459, 434, 384, 279, 231, 163, 102, 78, 30, 5, 52, 100, 155, 217, 277, 328, 389, 446, 473,
        546, 583, 649, 702, 734, 768, 857, 882, 912, 1043, 1219, 1243, 1258, 1290, 1325, 1359,
        1409, 1567, 1642, 1679, 1726, 1873, 1965, 2017, 2088, 2172, 2204, 2226, 2273, 2288, 2316,
        2434, 2522, 2558, 2622, 2678, 2790, 2933, 2965, 3025, 3037, 3071, 3167, 3180, 3194, 3233,
        3269, 3282, 3383, 3387, 3401, 3465, 3528, 3595, 3801, 3910, 4020, 4078, 4128, 4213, 4271,
        4295, 4420, 4472, 4612, 4663, 4739, 4845, 4891, 4980, 5109, 5241, 5284, 5335, 5379, 5388,
        5478, 5546, 5639, 5705, 5751, 5766, 5803, 5855, 5879, 5975, 6000, 6024, 6070, 6093, 6137,
        6156, 6212, 6256, 6276, 6304, 6421, 6441, 6537, 6614, 6743, 6844, 6893, 7087, 7169, 7183,
        7200, 7237, 7262, 7352, 7376, 7398, 7441, 7491, 7541, 7564, 7602, 7656, 7707, 7814, 7924,
        7940, 7958, 8014, 8036, 8048, 8132, 8141, 8250, 8279, 8288, 8321, 8331, 8374, 8515, 8655,
        8723, 8807, 8825, 8878, 8953, 8990, 9011, 9077, 9128, 9172, 9219, 9276, 9383, 9420, 9499,
        9535, 9736, 9744, 9801, 9900, 9951, 10038, 10093, 10119, 10147, 10265, 10301, 10314, 10340,
        10456, 10499, 10564, 10622, 10767, 10802, 10876, 10882, 10997, 11063, 11217, 11243, 11276,
        11299, 11314, 11365, 11407, 11456, 11587, 11627, 11705, 11751, 11792, 11831, 11901, 12012,
        12118, 12180, 12240, 12296, 12385, 12469, 12473, 12497, 12503, 12537, 12578, 12723, 12778,
        12858, 12901, 12936, 13020, 13048, 13136, 13195, 13232, 13325, 13377, 13424, 13493, 13547,
        13564, 13724, 13856, 13911, 13938, 14075, 14151, 14234, 14300, 14353, 14395, 14499, 14507,
        14705, 14724, 14796, 14802, 14823, 14858, 14882, 14905, 14914, 14936, 14962, 15049, 15114,
        15161, 15237, 15272, 15399, 15565, 15587, 15666, 15749, 15778, 15830, 15843, 15864, 15928,
        16039, 16075, 16141, 16163, 16246, 16315, 16333, 16389, 16415, 16526, 16601, 16650, 16798,
        16845, 16861, 16991, 17046, 17090, 17140, 17178, 17186, 17292, 17305, 17343, 17419, 17456,
        17610, 17617, 17693, 17728, 17783, 17909, 17918, 17970, 18032, 18083, 18104, 18114, 18223,
        18296, 18330, 18363, 18428, 18496, 18578, 18660, 18733, 18782, 18792, 18861, 18929, 19069,
        19127, 19184, 19269, 19279, 19355, 19394, 19494, 19539, 19559, 19599, 19612, 19643, 19666,
        19745, 19760, 19815, 19864, 20012, 20141, 20231, 20270, 20330, 20370, 20380, 20500, 20595,
        20617, 20690, 20751, 20811, 20824, 20843, 20910, 20925, 21044, 21126, 21165, 21198, 21260,
        21280, 21343, 21467, 21505, 21531, 21564, 21640, 21755, 21817, 21885, 21929, 22010, 22103,
        22159, 22196, 22229, 22270, 22368, 22414, 22515, 22570, 22615, 22630, 22806, 22864, 22951,
        23030, 23107, 23155, 23191, 23226, 23399, 23438, 23464, 23487, 23524, 23559, 23634, 23667,
        23719, 23747, 23764, 23869, 23901, 23936, 24012, 24022, 24045, 24074, 24141, 24185, 24204,
        24272, 24327, 24452, 24455, 24490, 24560, 24615, 24641, 24734, 24815, 24890, 24963, 25025,
        25242, 25282, 25283, 25414, 25446, 25475, 25489, 25527, 25586, 25636, 25640, 25771, 25844,
        25848, 25883, 25923, 26005, 26048, 26106, 26157, 26312, 26359, 26395, 26429, 26465, 26491,
        26513, 26558, 26584, 26601, 26667, 26770, 26864, 26900, 26996, 27118, 27129, 27176, 27272,
        27313, 27389, 27478, 27517, 27580, 27700, 27761, 27811, 27844, 27848, 27967, 28051, 28108,
        28176, 28264, 28302, 28332, 28380, 28525, 28591, 28617, 28681, 28727, 28744, 28874, 28994,
        29047, 29123, 29221, 29239, 29274, 29347, 29493, 29596, 29668, 29694, 29717, 29847, 29871,
    ];
    let ans = sum_subarray_mins1(arr);
    assert_eq!(508796209, ans);
}

#[test]
fn test3() {
    let arr = vec![
        10322, 29855, 18404, 12530, 23720, 2030, 17116, 15622, 25837, 17587, 14559, 15607, 23457,
        10420, 16734, 21222, 29867, 20811, 28857, 11971, 18264, 22359, 10453, 8076, 29185, 10457,
        29998, 25569, 6483, 24847, 29509, 8305, 19934, 23088, 2499, 24580, 9251, 21192, 28505,
        1054, 18352, 8773, 3440, 21356, 29372, 15382, 10391, 13473, 12439, 15958, 12276, 2412,
        29383, 29902, 5225, 7232, 13109, 20962, 941, 25065, 19154, 8079, 26127, 19837, 9508, 16436,
        29378, 1453, 21254, 25483, 20770, 4201, 17915, 23579, 12558, 17473, 7605, 9383, 24258,
        14685, 13465, 3687, 11235, 15622, 6931, 24239, 11512, 7155, 9249, 24734, 27125, 28809, 456,
        22617, 15765, 3737, 7397, 8451, 12127, 14122, 28104, 1751, 21276, 25624, 10719, 7477, 6640,
        9026, 4359, 16551, 7512, 818, 6980, 24619, 12522, 26507, 28309, 7301, 16800, 26433, 17443,
        5041, 7439, 29629, 8982, 26032, 23851, 22260, 21659, 23700, 25423, 1871, 5035, 15166, 6375,
        15997, 14796, 3804, 2579, 350, 24752, 2453, 28847, 29515, 22371, 13512, 8274, 12374, 10359,
        11889, 21786, 26776, 4732, 7281, 6297, 1361, 25627, 15339, 2012, 13388, 13519, 23339,
        22842, 4035, 18807, 15291, 405, 4433, 20006, 11011, 28911, 15053, 20649, 4009, 14384,
        22024, 25005, 5989, 11908, 14206, 13212, 14264, 8878, 24262, 27393, 10471, 19136, 11422,
        17363, 20867, 15046, 20238, 22715, 25299, 5665, 6492, 15432, 15290, 24232, 15522, 27002,
        23329, 15190, 24790, 14276, 10252, 13003, 13687, 19516, 1565, 21886, 29047, 13765, 2064,
        6038, 3097, 7691, 23818, 32, 26208, 28187, 5551, 5208, 28973, 10812, 24354, 271, 29724,
        495, 18228, 27854, 24938, 9313, 24663, 11792, 14995, 10899, 10642, 17463, 23463, 20985,
        23043, 429, 15951, 10584, 6259, 27626, 5905, 5535, 5365, 19743, 18353, 15170, 17605, 28218,
        25837, 27179, 1625, 26923, 941, 19430, 27925, 15074, 12580, 9953, 27484, 27780, 18574,
        21433, 10174, 4146, 29371, 19711, 8232, 29313, 18270, 9918, 26875, 2338, 24125, 4788, 3231,
        7769, 21445, 18241, 12639, 4772, 27713, 22989, 20588, 24388, 23228, 3373, 23201, 25162,
        22403, 14469, 20594, 3001, 22931, 7867, 23554, 19059, 15273, 16081, 2242, 1227, 445, 23267,
        4155, 3686, 11552, 29332, 26578, 9399, 24594, 2553, 11762, 17377, 29588, 1462, 12373,
        27589, 829, 17973, 11982, 16809, 21101, 12201, 26762, 28672, 29552, 1644, 25105, 26352,
        4363, 28243, 3814, 6220, 28667, 24924, 17297, 8635, 7582, 12095, 270, 19091, 1547, 23216,
        2105, 308, 8540, 23181, 24980, 15220, 28149, 3436, 9967, 22211, 9700, 4361, 17351, 1878,
        11191, 7617, 9955, 14556, 16072, 2531, 9466, 11511, 12100, 14402, 12777, 2221, 6580, 19329,
        24863, 15344, 4446, 2115, 4677, 11522, 16957, 19928, 15731, 16959, 10568, 19968, 21806,
        12064, 24448, 22318, 27143, 14003, 10357, 23317, 1128, 11517, 29316, 10268, 15370, 7494,
        2309, 3329, 13057, 18571, 16373, 15561, 3350, 1208, 10761, 28275, 5395, 8078, 14495, 27427,
        28410, 40, 19437, 7085, 19635, 22286, 26615, 20503, 25417, 23535, 4822, 1311, 22164, 15778,
        29936, 4947, 11559, 8634, 26361, 14511, 27410, 21216, 29965, 17994, 29285, 5203, 13251,
        17352, 29349, 17037, 25958, 18856, 15373, 11744, 11060, 8857, 6342, 28878, 16094, 25977,
        26549, 28269, 7146, 10132, 18995, 9662, 4318, 22796, 16512, 16096, 21315, 3443, 27658,
        14395, 20755, 18010, 18157, 21298, 2664, 14902, 6309, 11691, 15353, 10619, 12202, 21927,
        1300, 28698, 18118, 4907, 16717, 2429, 15045, 20659, 12594, 9426, 20212, 28059, 26210,
        11562, 25903, 3452, 1762, 29496, 22888, 18450, 16764, 9267, 26973, 25583, 14452, 6612,
        20292, 21786, 29866, 23712, 2738, 29699, 25636, 17476, 9928, 21988, 17899, 2873, 16906,
        606, 23661, 24708, 21904, 2745, 17643, 11742, 3852, 26763, 28269, 27669, 15937, 25933,
        5936, 8865, 27262, 17712, 6781, 3903, 6850, 14881, 9105, 22033, 8140, 2354, 26947, 19916,
        29222, 5460, 26112, 516, 16135, 14393, 3786, 24454, 8126, 26954, 20983, 25571, 25995,
        23754, 22095, 123, 4303, 6214, 17325, 102, 3815, 14550, 1232, 9563, 6600, 5233, 9499,
        26437, 6944, 19475, 22020, 20259, 5663, 10495, 8165, 16160, 29072, 6537, 16572, 1965,
        11268, 28626, 27255, 2857, 25576, 21472, 27538, 13825, 12677, 26885, 16085, 22842, 5330,
        2054, 13215, 9812, 15357, 10330, 25878, 22069, 11519, 3776, 21296, 16233, 4559, 1044,
        18502, 15488, 17264, 12476, 14062, 11743, 2632, 16060, 3649, 20205, 22483, 5038, 6046,
        7222, 17956, 12195, 26626, 16440, 15770, 6555, 2726, 27742, 2990, 3907, 5860, 17305, 19172,
        12950, 11847, 19231, 7863, 24014, 19202, 18086, 867, 10362, 23066, 6178, 19335, 29242,
        13217, 15539, 6362, 211, 7099, 14156, 18129, 25078, 8701, 9872, 21620, 19929, 21436, 26319,
        2661, 3735, 14891, 18379, 19623, 6903, 4083, 27644, 7203, 535, 8488, 15516, 19001, 22177,
        4368, 15237, 9606, 21741, 10785, 24335, 5749, 29842, 15644, 12716, 21770, 11366, 1065,
        13227, 8634, 19837, 15802, 24903, 14679, 4661, 4459, 17179, 7947, 6348, 28254, 4179, 27483,
        16087, 28108, 25171, 8967, 14106, 2559, 11001, 27802, 3030, 7370, 1282, 25825, 20508,
        17681, 13970, 7802, 17576, 21082, 23790, 4875, 18760, 20365, 17436, 21833, 15534, 28640,
        19506, 18842, 395, 4307, 18033, 23008, 4328, 19105, 4631, 22897, 24637, 18619, 2038, 8384,
        8086, 14036, 23402, 17350, 29759, 21248, 4241, 29374, 1759, 9986, 19118, 11715, 666, 8895,
        7257, 23285, 17777, 4320, 26179, 6390, 9591, 26242, 22957, 12625, 15579, 29376, 21325,
        21472, 19674, 29648, 27721, 8948, 13355, 19071, 7108, 19419, 27099, 9189, 11038, 13499,
        11590, 19300, 1560, 23303, 8561, 18661, 12717, 18339, 17069, 15527, 4810, 226, 3210, 11547,
        7713, 14544, 14125, 15462, 3982, 14923, 28515, 5160, 467, 10167, 21255, 25823, 3279, 937,
        9305, 18662, 27613, 9913, 23381, 3826, 19230, 7499, 22836, 27364, 13243, 20617, 10622,
        25473, 12305, 17523, 29594, 16730, 13585, 2882, 28489, 15747, 21024, 19638, 7124, 19113,
        2899, 1719, 25228, 18024, 9042, 15806, 16738, 20334, 5, 4334, 2796, 22593, 13571, 5952,
        11235, 20084, 13837, 16376, 28126, 12010, 3098, 3215, 21742, 9375, 3452, 23346, 26665,
        3071, 18496, 22215, 7376, 25101, 20521, 13338, 4972, 7735, 24885, 5035, 21140, 17129,
        16822, 490, 3682, 9295, 18910, 11444, 7695, 11516, 13894, 17838, 16793, 11041, 12764,
        24188, 17671, 29115, 18075, 8499, 15394, 14188, 25588, 22125, 26418, 21535, 7599, 8185,
        4915, 26372, 27003, 9707, 697, 14229, 23652, 21089, 20269, 631, 3058, 21877, 24557, 5467,
        24492, 28539, 18050, 16552, 988, 12419, 14010, 28604, 13226, 358, 17014, 2044, 29761,
        19791, 21573, 20805, 28230, 12149, 8369, 2362, 685, 14316, 22312, 22178, 87, 18500, 24954,
        26009, 23089, 12567, 21119, 21047, 1927, 1167, 10384, 19301, 11429, 19350, 22846, 23142,
        8496, 29153, 16614, 18838, 19057, 20201, 4621, 20238, 12947, 29061, 21425, 29220, 29732,
        25020, 17586, 18021, 14091, 11091, 12563, 27405, 19390, 5098, 1118, 13175, 13226, 1975,
        6803, 9943, 11302, 18748, 4680, 24654, 14914, 2074, 2988, 28296,
    ];
    let ans = sum_subarray_mins(arr);
    assert_eq!(134082962, ans);
}
