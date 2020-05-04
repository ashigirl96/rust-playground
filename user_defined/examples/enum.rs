use crate::Task::{AssignTo, Done, Working};

#[derive(Debug)]
struct UserName(String);

#[derive(Debug)]
enum Task {
    Open,
    AssignTo(UserName),
    Working {
        assignee: UserName,
        remaining_hours: u16,
    },
    Done,
}

fn main() {
    let tasks = vec![
        Task::Open,
        Task::AssignTo(UserName(format!("Yui"))),
        Task::Working {
            assignee: UserName(format!("reon")),
            remaining_hours: 18,
        },
        Task::Done,
    ];

    for (i, task) in tasks.iter().enumerate() {
        match task {
            AssignTo(assignee) => {
                println!("Task {} is assigned by {:?}", i, assignee.0);
            }
            Working {
                assignee,
                remaining_hours,
            } => {
                println!(
                    "Task {} is working by {:?}. remaining time is {} hours",
                    i, assignee.0, remaining_hours
                );
            }
            _ => {
                println!("Status of task{} is {:?}", i, task);
            }
        }
    }
}
