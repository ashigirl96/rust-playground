use crate::SortOrder;
use std::cmp::Ordering;

pub fn is_power_of_two(x: usize) -> bool {
    let y = (x as f64).log2();
    y == y as u64 as f64
}

pub fn sort<T: Ord>(x: &mut [T], order: &SortOrder) -> Result<(), String> {
    match *order {
        SortOrder::Ascending => sort_by(x, &|a, b| a.cmp(b)),
        SortOrder::Descending => sort_by(x, &|a, b| b.cmp(a)),
    }
}

fn sort_by<T, F>(x: &mut [T], comparator: &F) -> Result<(), String>
where
    F: Fn(&T, &T) -> Ordering,
{
    if is_power_of_two(x.len()) {
        do_sort(x, true, comparator);
        Ok(())
    } else {
        Err(format!(
            "The length of x is not a power of two. (x.len(): {})",
            x.len()
        ))
    }
}

fn do_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    if x.len() > 1 {
        let mid_point = x.len() / 2;
        do_sort(&mut x[..mid_point], true, comparator);
        do_sort(&mut x[mid_point..], false, comparator);
        sub_sort(x, forward, comparator);
    }
}

fn sub_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    if x.len() > 1 {
        compare_and_swap(x, forward, comparator);
        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], forward, comparator);
        sub_sort(&mut x[mid_point..], forward, comparator);
    }
}

fn compare_and_swap<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    let swap_condition = if forward {
        Ordering::Greater
    } else {
        Ordering::Less
    };
    let mid_point = x.len() / 2;
    for i in 0..mid_point {
        if comparator(&x[i], &x[mid_point + i]) == swap_condition {
            x.swap(i, mid_point + i);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::third::{sort, sort_by};
    use crate::SortOrder::*;
    use std::fmt::Display;

    #[derive(Debug)]
    struct Student {
        first_name: String,
        last_name: String,
        age: u8,
    }

    impl Display for Student {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                f,
                "Name: {0} {1}, age: {2}",
                self.first_name, self.last_name, self.age
            )
        }
    }

    impl Student {
        fn new(first_name: &str, last_name: &str, age: u8) -> Self {
            Self {
                first_name: first_name.to_string(),
                last_name: last_name.to_string(),
                age,
            }
        }
    }

    impl PartialEq for Student {
        fn eq(&self, other: &Self) -> bool {
            self.first_name == other.first_name
                && self.last_name == other.last_name
                && self.age == other.age
        }
    }

    #[test]
    fn sort_u32_ascending() {
        let mut x = vec![10, 30, 11, 20, 4, 330, 21, 110];

        assert_eq!(sort(&mut x, &Ascending), Ok(()));

        assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn sort_student_ascending() {
        let paul = Student::new("Paul", "Smith", 34);
        let taro = Student::new("Yamada", "Taro", 25);
        let hanako = Student::new("Yamada", "Hanako", 20);
        let yui = Student::new("Irie", "Yui", 19);

        let mut x = vec![&taro, &yui, &paul, &hanako];

        let expected = vec![&yui, &hanako, &taro, &paul];

        assert_eq!(sort_by(&mut x, &|a, b| a.age.cmp(&b.age)), Ok(()));
        assert_eq!(x, expected);
    }

    #[test]
    fn sort_student_by_name_ascending() {
        let paul = Student::new("Paul", "Smith", 34);
        let taro = Student::new("Yamada", "Taro", 25);
        let hanako = Student::new("Yamada", "Aanako", 20);
        let yui = Student::new("Arie", "Yui", 19);

        let mut x = vec![&taro, &yui, &paul, &hanako];

        let expected = vec![&yui, &paul, &hanako, &taro];

        assert_eq!(
            sort_by(&mut x, &|a: &&Student,
                              b: &&Student|
             -> std::cmp::Ordering {
                a.first_name
                    .cmp(&b.first_name)
                    .then_with(|| a.last_name.cmp(&b.last_name))
            }),
            Ok(())
        );
        assert_eq!(x, expected);
    }

    #[test]
    fn compare() {
        use std::cmp::Ordering;
        assert_eq!(14u8.cmp(&16u8), Ordering::Less); // 14 < 16
        assert_eq!(15u8.cmp(&15u8), Ordering::Equal); // 15 == 15
        assert_eq!(16u8.cmp(&14u8), Ordering::Greater); // 16 > 14

        println!("{0} {1:p} {2:p}", 1u8, &1u8, &&1u8);
    }

    #[test]
    fn print_stduent() {
        let takeda = Student::new("Takeda", "Shingen", 23);
        println!("{:?}", takeda);
        println!("{:?}", &takeda);
        println!("{:p}", &takeda);
        println!("{}", takeda);
    }

    #[test]
    fn is_power_of_two() {
        use crate::third::is_power_of_two;

        assert_eq!(is_power_of_two(2), true);
        assert_eq!(is_power_of_two(3), false);
        assert_eq!(is_power_of_two(4), true);
        assert_eq!(is_power_of_two(6), false);
        assert_eq!(is_power_of_two(8), true);
    }
}
