//这个测验测试：
//-泛型
//-特征
//
//虚构的魔法学校编写了新的成绩单生成系统
//在 Rust 中！目前，系统仅支持创建成绩单，其中
//学生的成绩以数字表示（例如 1.0 -> 5.5）。然而，
//学校还发布按字母顺序排列的成绩（A+ -> F-）并且需要能够
//打印两种类型的报告卡！
//
//在结构 `ReportCard` 和 impl 中进行必要的代码更改
//除了数字报告卡之外，还支持字母报告卡。

//TODO：如上所述调整结构。
struct ReportCard<T> {
    grade: T,
    student_name: String,
    student_age: u8,
}

// TODO: Adjust the impl block as described above.
impl<T: std::fmt::Display> ReportCard<T> {
    fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade,
        )
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1",
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+",
        );
    }
}
