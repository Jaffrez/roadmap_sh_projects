use chrono::{DateTime, Utc};
use clap::ValueEnum;
use rusqlite::Connection;
use std::fmt;
use std::io::{Error, ErrorKind};

#[derive(Debug, Clone)]
pub struct Task {
    id: u64,
    name: String,
    description: String,
    status: TaskStatus,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
}

#[allow(unused)]
impl Task {
    pub fn new(id: u64, name: String, description: String) -> Self {
        Task {
            id,
            name,
            description,
            status: TaskStatus::Todo,
            created: Utc::now(),
            updated: Utc::now(),
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn status(&self) -> &TaskStatus {
        &self.status
    }

    pub fn created(&self) -> &DateTime<Utc> {
        &self.created
    }

    pub fn updated(&self) -> &DateTime<Utc> {
        &self.updated
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
        self.updated = Utc::now();
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
        self.updated = Utc::now();
    }

    pub fn set_status(&mut self, status: TaskStatus) {
        self.status = status;
        self.updated = Utc::now();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskStatus {
    Todo,
    Skip,
    InProgress,
    Done,
}

#[allow(unused)]
impl TaskStatus {
    pub fn from_str(s: &str) -> Option<&Self> {
        match s {
            "todo" => Some(&TaskStatus::Todo),
            "skip" => Some(&TaskStatus::Skip),
            "in_progress" => Some(&TaskStatus::InProgress),
            "done" => Some(&TaskStatus::Done),
            _ => None,
        }
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskStatus::Todo => write!(f, "todo"),
            TaskStatus::Skip => write!(f, "skip"),
            TaskStatus::InProgress => write!(f, "in_progress"),
            TaskStatus::Done => write!(f, "done"),
        }
    }
}

impl ValueEnum for TaskStatus {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            TaskStatus::Todo,
            TaskStatus::Skip,
            TaskStatus::InProgress,
            TaskStatus::Done,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            TaskStatus::Todo => Some(clap::builder::PossibleValue::new("todo")),
            TaskStatus::Skip => Some(clap::builder::PossibleValue::new("skip")),
            TaskStatus::InProgress => Some(clap::builder::PossibleValue::new("in_progress")),
            TaskStatus::Done => Some(clap::builder::PossibleValue::new("done")),
        }
    }
}

#[allow(unused)]
pub struct TaskVec {
    tasks: Vec<Task>,
    next_id: u64,
}

#[allow(unused)]
impl TaskVec {
    pub fn new() -> Self {
        TaskVec {
            tasks: vec![],
            next_id: 0,
        }
    }

    pub fn from(path: &str) -> Self {
        let mut result = TaskVec::new();
        let conn = match Connection::open(path) {
            Ok(conn) => conn,
            Err(e) => panic!("Error: {}", e),
        };
        // * 如果不存在表则创建
        conn.execute(
            "create table if not exists tasks (
                                id int primary key,
                                name text,
                                desc text,
                                status text,
                                created Date,
                                updated Date)",
            [],
        );
        // * 从数据库中获取数据并保存
        let mut stmt = conn
            .prepare("SELECT id, name, desc, status, created, updated FROM tasks")
            .unwrap();
        let task_iter = stmt
            .query_map([], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    status: match row.get::<_, String>(3)?.as_str() {
                        "todo" => TaskStatus::Todo,
                        "skip" => TaskStatus::Skip,
                        "in_progress" => TaskStatus::InProgress,
                        "done" => TaskStatus::Done,
                        _ => {
                            return Err(rusqlite::Error::InvalidColumnType(
                                3,
                                "status".to_string(),
                                rusqlite::types::Type::Text,
                            ))
                        }
                    },
                    created: row.get(4)?,
                    updated: row.get(5)?,
                })
            })
            .unwrap();

        for task in task_iter {
            if let Ok(task) = task {
                result.tasks.push(task);
            }
        }

        result.next_id = result.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        result
    }

    pub fn to(&self, path: &str) {
        let conn = match Connection::open(path) {
            Ok(conn) => conn,
            Err(e) => panic!("Error: {}", e),
        };
        // * 如果不存在表则创建
        conn.execute(
            "create table if not exists tasks (
                                id int primary key,
                                name text,
                                desc text,
                                status text,
                                created Date
                                updated Date)",
            [],
        );
        // * 保存数据
        conn.execute("DELETE FROM tasks", [])
            .expect("Failed to clear tasks table");

        for task in &self.tasks {
            conn.execute(
                "INSERT INTO tasks (id, name, desc, status, created, updated) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                (
                    task.id(),
                    task.name(),
                    task.description(),
                    task.status().to_possible_value().unwrap().get_name(),
                    task.created().to_rfc3339(),
                    task.updated().to_rfc3339(),
                ),
            )
            .expect("Failed to insert task");
        }
    }

    pub fn add(&mut self, name: &str, desc: &str) -> Option<&mut Task> {
        let task = Task::new(self.next_id, name.to_string(), desc.to_string());
        self.tasks.push(task);
        self.next_id += 1;
        println!("Successfully added task, id: {}", self.next_id - 1);
        self.tasks.last_mut()
    }

    pub fn del(&mut self, id: u64) -> Task {
        let index = self.tasks.iter().position(|task| task.id() == id).unwrap();
        println!("Successfully deleted task, id: {}", id);
        self.tasks.remove(index)
    }

    pub fn update(
        &mut self,
        id: u64,
        name: Option<&String>,
        desc: Option<&String>,
    ) -> Result<&Task, Error> {
        let index = match self.tasks.iter().position(|task| task.id() == id) {
            Some(idx) => idx,
            None => return Err(Error::new(ErrorKind::NotFound, "task not found")),
        };
        let task = self.tasks.get_mut(index).unwrap();
        if let Some(name) = name {
            task.set_name(name.to_string());
        }
        if let Some(desc) = desc {
            task.set_description(desc.to_string());
        }
        println!("Successfully updated task, id: {}", id);
        Ok(task)
    }

    pub fn mark(&mut self, id: u64, status: &TaskStatus) -> Result<&Task, Error> {
        let index = match self.tasks.iter().position(|task| task.id() == id) {
            Some(idx) => idx,
            None => return Err(Error::new(ErrorKind::NotFound, "task not found")),
        };
        let task = self.tasks.get_mut(index).unwrap();
        task.set_status(status.clone());
        println!("Successfully marked task, id: {}", id);
        Ok(task)
    }

    pub fn list_by_status(&self, status: Option<&TaskStatus>) -> Vec<&Task> {
        match status {
            Some(s) => {
                return self
                    .tasks
                    .iter()
                    .filter(|task| task.status() == s)
                    .collect();
            }
            None => {
                return self.tasks.iter().collect();
            }
        }
    }
}
