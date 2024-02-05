/*
100214. 边界上的蚂蚁
边界上有一只蚂蚁，它有时向 左 走，有时向 右 走。

给你一个 非零 整数数组 nums 。蚂蚁会按顺序读取 nums 中的元素，从第一个元素开始直到结束。每一步，蚂蚁会根据当前元素的值移动：

如果 nums[i] < 0 ，向 左 移动 -nums[i]单位。
如果 nums[i] > 0 ，向 右 移动 nums[i]单位。
返回蚂蚁 返回 到边界上的次数。

注意：

边界两侧有无限的空间。
只有在蚂蚁移动了 |nums[i]| 单位后才检查它是否位于边界上。换句话说，如果蚂蚁只是在移动过程中穿过了边界，则不会计算在内。

这道题好难理解，只说是边界上，又没说左边还是右边。WA 了两次。
*/
pub fn return_to_boundary_count(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let n = nums.len() as i32;
    let mut index = n;
    for num in nums {
        index += num;
        if index == n {
            ans += 1;
        }
    }
    ans
}

/*
100204. 将单词恢复初始状态所需的最短时间 I
100203. 将单词恢复初始状态所需的最短时间 II
100203 的数据范围 1e5 但是暴力也能过。如果需要优化，可以考虑使用 z 函数，将时间复杂度做到 n^2

给你一个下标从 0 开始的字符串 word 和一个整数 k 。

在每一秒，你必须执行以下操作：

移除 word 的前 k 个字符。
在 word 的末尾添加 k 个任意字符。
注意 添加的字符不必和移除的字符相同。但是，必须在每一秒钟都执行 两种 操作。

返回将 word 恢复到其 初始 状态所需的 最短 时间（该时间必须大于零）。
*/
pub fn minimum_time_to_initial_state(word: String, k: i32) -> i32 {
    let mut ans = 1;
    let k = k as usize;
    let cnt = if word.len() % k == 0 {
        word.len() / k
    } else {
        word.len() / k + 1
    };
    for i in 1..cnt {
        if word.starts_with(word.get(i * k..word.len()).unwrap()) {
            break;
        }
        ans += 1;
    }
    ans
}
/*
100189. 找出网格的区域平均强度

给你一个下标从 0 开始、大小为 m x n 的网格 image ，表示一个灰度图像，其中 image[i][j] 表示在范围 [0..255] 内的某个像素强度。另给你一个 非负 整数 threshold 。

如果 image[a][b] 和 image[c][d] 满足 |a - c| + |b - d| == 1 ，则称这两个像素是 相邻像素 。

区域 是一个 3 x 3 的子网格，且满足区域中任意两个 相邻 像素之间，像素强度的 绝对差 小于或等于 threshold 。

区域 内的所有像素都认为属于该区域，而一个像素 可以 属于 多个 区域。

你需要计算一个下标从 0 开始、大小为 m x n 的网格 result ，其中 result[i][j] 是 image[i][j] 所属区域的 平均 强度，向下取整 到最接近的整数。如果 image[i][j] 属于多个区域，result[i][j] 是这些区域的 “取整后的平均强度” 的 平均值，也 向下取整 到最接近的整数。如果 image[i][j] 不属于任何区域，则 result[i][j] 等于 image[i][j] 。

返回网格 result 。

写起来非常非常的费劲。意义不大。需要注意的是，所选区域内的相邻元素的最大差值，而不是整个区域内。还有就是同一像素点，可能会计算多次平均值，答案要求返回多次平均值的平均值。
*/
   
pub fn result_grid(image: Vec<Vec<i32>>, threshold: i32) -> Vec<Vec<i32>> {
    let m = image.len();
    let n = image[0].len();
    // 再遍历 abs 判断这些点位是否满足矩形区域的定义
    // 每次结果累加到 list 上 ，
    let mut ans = image.clone();
    let mut cnt = vec![vec![-1; n]; m];
    for i in 1..m - 1 {
        for j in 1..n - 1 {
            let check = 
                // 中心点
                (image[i][j] - image[i][j+1]).abs() <= threshold
                && (image[i][j] - image[i][j-1]).abs() <= threshold
                && (image[i][j] - image[i-1][j]).abs() <= threshold
                && (image[i][j] - image[i+1][j]).abs() <= threshold
                // 左上角
                && (image[i-1][j-1] - image[i-1][j]).abs() <= threshold
                && (image[i-1][j-1] - image[i][j-1]).abs() <= threshold
                // 右上角
                && (image[i-1][j+1] - image[i-1][j]).abs() <= threshold
                && (image[i-1][j+1] - image[i][j+1]).abs() <= threshold
                // 左下角
                && (image[i+1][j-1] - image[i][j-1]).abs() <= threshold
                && (image[i+1][j-1] - image[i+1][j]).abs() <= threshold
                // 右下角
                && (image[i+1][j+1] - image[i+1][j]).abs() <= threshold
                && (image[i+1][j+1] - image[i][j+1]).abs() <= threshold
            ;
            if check {
                let sum = image[i][j]
                    + image[i][j - 1]
                    + image[i][j + 1]
                    + image[i + 1][j]
                    + image[i + 1][j - 1]
                    + image[i + 1][j + 1]
                    + image[i - 1][j]
                    + image[i - 1][j - 1]
                    + image[i - 1][j + 1];
                let avg = sum / 9;


                for ii in i-1..i+2 {
                    for jj in j-1..j+2 {
                        if cnt[ii][jj] != -1 {
                            ans[ii][jj] += avg;
                            cnt[ii][jj] +=1;
                        }else {
                            cnt[ii][jj] =1;
                            ans[ii][jj] = avg;
                        }
                    }
                }
            }
        }
    }
    // 最后 遍历 list 求平均值
    for i in 0..m {
        for j in 0..n {
            if cnt[i][j]!=-1 {
                ans[i][j] /= cnt[i][j];
            }
        }
    }
    ans
}


/*
292. Nim 游戏
你和你的朋友，两个人一起玩 Nim 游戏：

桌子上有一堆石头。
你们轮流进行自己的回合， 你作为先手 。
每一回合，轮到的人拿掉 1 - 3 块石头。
拿掉最后一块石头的人就是获胜者。
假设你们每一步都是最优解。请编写一个函数，来判断你是否可以在给定石头数量为 n 的情况下赢得游戏。如果可以赢，返回 true；否则，返回 false 。
*/

pub fn can_win_nim(n: i32) -> bool {
    n%4!=0
}