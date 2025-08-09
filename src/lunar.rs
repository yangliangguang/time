//! 简单的农历计算模块
//! 由于没有找到现成的Rust农历库，这里实现一个简化的农历计算功能

/// 农历信息结构
#[derive(Debug, Clone)]
pub struct LunarDate {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub is_leap_month: bool,
}

/// 将公历日期转换为农历日期（简化实现）
/// 注意：这是一个非常简化的实现，仅用于演示目的
pub fn solar_to_lunar(year: i32, month: i32, day: i32) -> LunarDate {
    // 这里使用一个简化的算法，实际的农历计算非常复杂
    // 简单起见，我们只实现一个近似的转换
    
    // 农历新年（春节）通常在公历1月或2月
    // 这里使用一个固定的偏移量来估算
    let lunar_new_year_offset = match year {
        2020 => 25,  // 2020年春节是1月25日
        2021 => 12,  // 2021年春节是2月12日
        2022 => 1,   // 2022年春节是2月1日
        2023 => 22,  // 2023年春节是1月22日
        2024 => 10,  // 2024年春节是2月10日
        2025 => 29,  // 2025年春节是1月29日
        2026 => 17,  // 2026年春节是2月17日
        2027 => 6,   // 2027年春节是2月6日
        2028 => 26,  // 2028年春节是1月26日
        2029 => 13,  // 2029年春节是2月13日
        _ => 10,     // 默认值
    };
    
    // 计算距离春节的天数
    let days_since_new_year = if month == 1 && day < lunar_new_year_offset {
        // 如果是1月且在春节前，说明还在去年的农历年
        let prev_year_offset = match year - 1 {
            2020 => 25,
            2021 => 12,
            2022 => 1,
            2023 => 22,
            2024 => 10,
            2025 => 29,
            2026 => 17,
            2027 => 6,
            2028 => 26,
            2029 => 13,
            _ => 10,
        };
        let days_in_prev_year = if is_leap_year(year - 1) { 366 } else { 365 };
        days_in_prev_year - prev_year_offset + day
    } else if month == 1 {
        day - lunar_new_year_offset
    } else if month == 2 && day < lunar_new_year_offset {
        31 - lunar_new_year_offset + day
    } else {
        let mut days = 0;
        // 计算从春节到当前月份的天数
        for m in 2..month {
            days += days_in_month(year, m);
        }
        if month > 2 || (month == 2 && day >= lunar_new_year_offset) {
            days += day - lunar_new_year_offset + 31; // 加上1月的天数
        } else {
            days += day;
        }
        days
    };
    
    // 简化的农历月份计算（假设每个月都是30天）
    let lunar_month = days_since_new_year / 30 + 1;
    let lunar_day = days_since_new_year % 30 + 1;
    
    // 简化的闰月处理（实际农历有复杂的闰月规则）
    let is_leap_month = false; // 简化处理，不考虑闰月
    
    // 确定农历年份
    let lunar_year = if month == 1 && day < lunar_new_year_offset {
        year - 1
    } else {
        year
    };
    
    LunarDate {
        year: lunar_year,
        month: lunar_month,
        day: lunar_day,
        is_leap_month,
    }
}

/// 判断是否为闰年
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// 获取指定月份的天数
fn days_in_month(year: i32, month: i32) -> i32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap_year(year) { 29 } else { 28 },
        _ => 0,
    }
}

/// 将农历日期格式化为字符串
pub fn format_lunar_date(lunar_date: &LunarDate) -> String {
    format!("农历{}年{}月{}日", lunar_date.year, lunar_date.month, lunar_date.day)
}