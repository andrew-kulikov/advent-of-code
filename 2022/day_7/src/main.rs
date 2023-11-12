struct CommandLine {
    name: String,
    args: Vec<String>,
}

enum OutputLine {
    Command(CommandLine),
    Output(String),
}

enum Command {
    Cd(String),
    Ls,
}

struct FileData {
    name: String,
    size: u32,
}

enum DirChild {
    File(FileData),
    Dir(u32),
}

struct DirData {
    id: u32,
    name: String,
    parent_id: Option<u32>,
    children: Vec<DirChild>,
}

struct Fs {
    last_dir_id: u32,
    dirs: Vec<DirData>,
}

struct ExecutionContext {
    current_dir_id: u32,
    fs: Fs,
    current_command: Option<Command>,
    current_output: Option<Vec<String>>,
}

fn parse_command(command: CommandLine) -> Command {
    if command.name == "cd" {
        return Command::Cd(command.args[0].clone());
    }

    Command::Ls
}

impl Fs {
    fn new() -> Fs {
        let root_dir = DirData {
            id: 0,
            name: String::from("/"),
            parent_id: None,
            children: vec![],
        };
        Fs {
            last_dir_id: 0,
            dirs: vec![root_dir],
        }
    }

    fn print(&self) {
        self.print_dir(0, 0);
    }

    fn print_dir(&self, dir_id: u32, offset: usize) -> () {
        let dir = self.find_dir(dir_id);

        println!("{}{}", str::repeat(" ", offset * 4), dir.name);

        for child in dir.children.iter() {
            if let DirChild::File(f) = child {
                println!("{}{}: {}", str::repeat(" ", offset * 4), f.name, f.size)
            }
        }

        for child in dir.children.iter() {
            if let DirChild::Dir(d) = child {
                self.print_dir(*d, offset + 1)
            }
        }
    }

    fn calculate_total_size(&self) -> u32 {
        return match self.dirs.first() {
            Some(dir) => self.calculate_dir_size(dir),
            None => 0,
        };
    }

    fn calculate_dir_size(&self, dir: &DirData) -> u32 {
        let mut size = 0;

        for child in dir.children.iter() {
            size += match child {
                DirChild::File(file) => file.size,
                DirChild::Dir(dir_id) => self.calculate_dir_size(self.find_dir(*dir_id)),
            };
        }

        return size;
    }

    fn try_create_dir(&mut self, name: String, parent_id: u32) {
        let existing_child = self.find_dir_by_name(name.clone(), parent_id);
        if existing_child.is_some() {
            return;
        }

        // Create child dir
        let next_id = self.last_dir_id + 1;

        self.dirs.push(DirData {
            id: next_id,
            name: name.clone(),
            parent_id: Some(parent_id),
            children: vec![],
        });

        let parent_dir = self.find_dir_mut(parent_id);
        parent_dir.children.push(DirChild::Dir(next_id));

        self.last_dir_id = next_id;
    }

    fn try_add_file(&mut self, dir_id: u32, name: String, size: u32) {
        let dir = self.find_dir_mut(dir_id);
        dir.children.push(DirChild::File(FileData { name, size }));
    }

    fn find_dir(&self, dir_id: u32) -> &DirData {
        self.dirs.iter().find(|x| x.id == dir_id).unwrap()
    }

    fn find_dir_mut(&mut self, dir_id: u32) -> &mut DirData {
        self.dirs.iter_mut().find(|x| x.id == dir_id).unwrap()
    }

    fn find_dir_by_name(&self, name: String, parent_id: u32) -> Option<&DirData> {
        self.dirs
            .iter()
            .find(|x| x.parent_id.is_some_and(|p| p == parent_id) && x.name == name)
    }
}

impl ExecutionContext {
    fn apply_output_line(&mut self, line: &str) -> () {
        let line: OutputLine = parse_output_line(line);

        match line {
            OutputLine::Command(command) => {
                self.execute_last_command();

                self.current_command = Some(parse_command(command));
                self.current_output = Some(vec![])
            }
            OutputLine::Output(output) => {
                let mut new_output = self.current_output.to_owned().unwrap();
                new_output.push(output);
                self.current_output = Some(new_output);
            }
        }
    }

    fn execute_last_command(&mut self) -> () {
        if self.current_command.is_none() {
            return;
        }

        let current = self.current_command.as_ref().unwrap();

        match current {
            Command::Cd(dir) => {
                self.execute_cd_command(dir.clone());
            }
            Command::Ls => {
                self.execute_ls_command();
            }
        }
    }

    fn execute_ls_command(&mut self) -> () {
        if self.current_output.is_none() {
            return;
        }

        self.current_output
            .as_ref()
            .unwrap()
            .iter()
            .for_each(|line| {
                let parts = line.split(' ').collect::<Vec<&str>>();

                match parts[..] {
                    ["dir", dir] => self
                        .fs
                        .try_create_dir(String::from(dir), self.current_dir_id),
                    [size, filename] => self.fs.try_add_file(
                        self.current_dir_id,
                        String::from(filename),
                        size.parse::<u32>().unwrap(),
                    ),
                    _ => {
                        panic!("Invalid format")
                    }
                }
            });
    }

    fn execute_cd_command(&mut self, dir: String) -> () {
        if dir == "/" {
            self.current_dir_id = 0;
            return;
        }

        dir.split('/').for_each(|segment| {
            if segment.is_empty() || segment == "." {
                return;
            }

            if segment == ".." {
                let parent_id = self.fs.find_dir(self.current_dir_id).parent_id;
                if let Some(parent_id) = parent_id {
                    self.current_dir_id = parent_id;
                }
                return;
            }

            let child = self
                .fs
                .find_dir_by_name(String::from(segment), self.current_dir_id);

            if let Some(c) = child {
                self.current_dir_id = c.id;
            }
        })
    }
}
// -----

fn parse_command_line(line: &str) -> CommandLine {
    // Skip command sign $
    let mut iter = line.split(" ").skip(1);
    let comand_name = iter.next().expect("Command name expected");
    let args: Vec<String> = iter.map(|s| String::from(s)).collect();

    CommandLine {
        name: String::from(comand_name),
        args: args,
    }
}

fn parse_output_line(line: &str) -> OutputLine {
    if line.starts_with("$") {
        return OutputLine::Command(parse_command_line(line));
    }

    OutputLine::Output(String::from(line))
}

fn solve(context: &ExecutionContext) -> u32 {
    let mut total_size = 0;

    for dir in context.fs.dirs.iter() {
        let size = context.fs.calculate_dir_size(dir);
        if size <= 100000 {
            total_size += size;
        }
    }

    return total_size;
}

fn solve_part2(context: &ExecutionContext) -> u32 {
    let total_size = context.fs.calculate_total_size();
    if total_size <= 40000000 {
        return 0;
    }

    let remaining_size = total_size - 40000000;
    let mut best_dir_size = total_size;

    for dir in context.fs.dirs.iter() {
        let size = context.fs.calculate_dir_size(dir);
        if size > remaining_size && size < best_dir_size {
            best_dir_size = size;
        }
    }

    return best_dir_size;
}

fn main() {
    let mut context: ExecutionContext = ExecutionContext {
        current_dir_id: 0,
        fs: Fs::new(),
        current_output: None,
        current_command: None,
    };

    include_str!("../data/input.txt")
        .lines()
        .for_each(|line| context.apply_output_line(line));

    context.execute_last_command();

    context.fs.print();

    let result = solve(&context);

    println!("Result: {}", result);

    let result2 = solve_part2(&context);

    println!("Result2: {}", result2);
}
