use std::collections::HashMap;
mod network;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Methods
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Associated Functions
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        // self.width > other.width && self.height > other.height
        self.area() > other.area()
    }
}

fn main() {
    network::test();
    let default_task = Task {
        id: String::from("0"),
        title: String::from("Task 1"),
        content: String::from("Task 1 content"),
        status: TaskStatus::Backlog,
    };
    let task1 = Task {
        id: String::from("1"),
        title: String::from("Task 1"),
        content: String::from("Task 1 content"),
        status: TaskStatus::Backlog,
    };
    let task2 = Task {
        id: String::from("2"),
        title: String::from("Task 2"),
        content: String::from("Task 2 content"),
        status: TaskStatus::InProgress,
    };
    let mut tasks : HashMap<String, Task>= HashMap::new();
    tasks.insert(task1.id.clone(), task1);
    tasks.insert(task2.id.clone(), task2);

    let project = Project {
        id: String::from("1"),
        name: String::from("Project 1"),
        owner_id: String::from("1"),
        description: String::from("Project 1 description"),
        members: vec![String::from("1"), String::from("2")],
        project_tasks: ProjectTaskData {
            sorted_ids: vec![String::from("1"), String::from("2")],
            tasks,
        },
    };

    let get_task = match project.project_tasks.get_task("3") {
        Some(task) => task,
        None => &default_task,
    };

    println!("Project: {:#?}", project);
    println!("Project: {:#?}", get_task);


    test_screen();
    loader_test();
}

trait Draw {
    fn draw(&self);
    fn test(&self) {
        println!("Test");
    }
}

struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        self.test();
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        self.test();
    }
}

struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

fn test_screen() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 75,
                height: 10,
                label: String::from("OK"),
            }),
            Box::new(SelectBox {
                width: 50,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
        ],
    };
    screen.run();
}

#[derive(Debug)]
struct Project {
    id: String,
    name: String,
    owner_id: String,
    description: String,
    members: Vec<String>,
    project_tasks: ProjectTaskData,
}

#[derive(Debug)]
struct ProjectTaskData {
    sorted_ids: Vec<String>,
    tasks: HashMap<String, Task>,
}

impl ProjectTaskData {
    fn get_task(&self, id: &str) -> Option<&Task> {
        self.tasks.get(id)
    }

    fn update_task(&mut self, id: &str, task: Task) {
        self.tasks.insert(id.to_string(), task);
    }

    fn delete_task(&mut self, id: &str) {
        self.tasks.remove(id);
        self.sorted_ids.retain(|x| x != id);
    }
}

#[derive(Debug)]
struct Task {
    id: String,
    title: String,
    content: String,
    status: TaskStatus,
}

#[derive(Debug)]
enum TaskStatus {
    Backlog,
    InProgress,
    Done,
}

trait Loader {
    fn load(&mut self, id: &str) -> &Box<Project>;
    fn request(&self, id: &str) -> Box<Project> {
        Box::new(Project {
            id: id.to_string(),
            name: String::from("Project 2"),
            owner_id: String::from("1"),
            description: String::from("Project 2 description"),
            members: vec![String::from("1"), String::from("2")],
            project_tasks: ProjectTaskData {
                sorted_ids: vec![String::from("1"), String::from("2")],
                tasks: HashMap::new(),
            },
        })
    }
    fn join(&self, id: &str) -> Box<Project> {
        Box::new(Project {
            id: id.to_string(),
            name: String::from("Project 1"),
            owner_id: String::from("1"),
            description: String::from("Project 1 description"),
            members: vec![String::from("1"), String::from("2")],
            project_tasks: ProjectTaskData {
                sorted_ids: vec![String::from("1"), String::from("2")],
                tasks: HashMap::new(),
            },
        })
    }
}

struct ProjectLoaderImpl {
    user_id: String,
    map: HashMap<String, Box<Project>>,
}

impl Loader for ProjectLoaderImpl {
    fn load(&mut self, id: &str) -> &Box<Project> {
        let task1 = Task {
            id: String::from("1"),
            title: String::from("Task 1"),
            content: String::from("Task 1 content"),
            status: TaskStatus::Backlog,
        };
        let task2 = Task {
            id: String::from("2"),
            title: String::from("Task 2"),
            content: String::from("Task 2 content"),
            status: TaskStatus::InProgress,
        };
        let mut tasks : HashMap<String, Task>= HashMap::new();
        tasks.insert(task1.id.clone(), task1);
        tasks.insert(task2.id.clone(), task2);

        let project = Project {
            id: String::from("1"),
            name: String::from("Project 1"),
            owner_id: String::from("1"),
            description: String::from("Project 1 description"),
            members: vec![String::from("1"), String::from("2")],
            project_tasks: ProjectTaskData {
                sorted_ids: vec![String::from("1"), String::from("2")],
                tasks,
            },
        };

        self.map.insert(project.id.clone(), Box::new(project));


        let project = self.map.get(id);
        let stable_project = match  project {
            Some(p) => p,
            None => {
                let project = self.request(id);
                self.map.insert(id.to_string(), project);
                self.map.get(id).unwrap()
            },
        };
        self.map.get(id).unwrap()

        // let find_me = stable_project.members.iter().find(|&x| x == &self.user_id);
        // match find_me {
        //     Some(_) => self.map.get(id).unwrap().to_owned(),
        //     None => {
        //         let project = self.join(id);
        //         self.map.insert(id.to_string(), project);
        //         self.map.get(id).unwrap()
        //     },
        // }
    }
}

fn loader_test() {
    let mut loader = ProjectLoaderImpl {
        user_id: String::from("2"),
        map: HashMap::new(),
    };

    loader.load("2");
    // project.as_mut().project_tasks.update_task("1", Task {
    //     id: String::from("1"),
    //     title: String::from("Task 1"),
    //     content: String::from("Task 1 content"),
    //     status: TaskStatus::Backlog,
    // });

    // map.get_mut("1").unwrap().project_tasks.update_task("1", Task {
    //     id: String::from("1"),
    //     title: String::from("Task 1"),
    //     content: String::from("Task 1 content"),
    //     status: TaskStatus::Backlog,
    // });

    let project = match loader.map.get_mut("2") {
        Some(p) => {
            println!("P: {:#?}", p);
            p.project_tasks.update_task("2", Task {
                id: String::from("change 1"),
                title: String::from("Task 1"),
                content: String::from("Task 1 content"),
                status: TaskStatus::Backlog,
            });

            p
        },
        None => {
            let project = loader.request("2");
            loader.map.insert("2".to_string(), project);
            loader.map.get("2").unwrap()
        },
    };

    println!("Project: {:#?}", project);
}