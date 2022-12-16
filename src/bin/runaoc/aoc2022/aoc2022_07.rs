use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Default)]
pub struct Aoc2022_07 {
    root: Rc<Dir>,
}

#[derive(Default, Debug)]
struct Dir {
    size: RefCell<usize>,
    parent: Option<Rc<Dir>>,
    subdir: RefCell<HashMap<String, Rc<Dir>>>,
}

impl Dir {
    fn size(&self) -> usize {
        *self.size.borrow() + self.subdir.borrow().values().fold(0, |a, b| a + b.size())
    }
}

impl Aoc2022_07 {
    pub fn new() -> Self {
        Self {
            root: Rc::new(Dir {
                ..Default::default()
            }),
        }
    }
}

impl crate::Runner for Aoc2022_07 {
    fn name(&self) -> (usize, usize) {
        (2022, 07)
    }

    fn parse(&mut self) {
        let mut cwd = Rc::clone(&self.root);

        for line in aoc::read_linedata_from_file::<String>("input/2022-07.txt") {
            let words = line.split(' ').collect::<Vec<&str>>();
            match (words[0], words[1]) {
                ("$", "ls") => {}

                ("$", "cd") => match words[2] {
                    "/" => cwd = Rc::clone(&self.root),
                    ".." => cwd = Rc::clone(cwd.parent.as_ref().unwrap()),
                    dirname => {
                        let newdir = cwd.subdir.borrow().get(dirname).unwrap().clone();
                        cwd = newdir;
                    }
                },

                ("dir", dirname) => {
                    cwd.subdir.borrow_mut().insert(
                        dirname.to_string(),
                        Rc::new(Dir {
                            size: RefCell::new(0),
                            parent: Some(Rc::clone(&cwd)),
                            subdir: RefCell::new(HashMap::new()),
                        }),
                    );
                }

                (size, _name) => {
                    *cwd.size.borrow_mut() += size.parse::<usize>().unwrap();
                }
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut res = 0;
        let mut to_visit = vec![Rc::clone(&self.root)];

        while let Some(dir) = to_visit.pop() {
            to_visit.extend(dir.subdir.borrow().values().map(|d| Rc::clone(d)));

            let size = dir.size();

            if size <= 100_000 {
                res += size;
            }
        }
        crate::output(res)
    }

    fn part2(&mut self) -> Vec<String> {
        let max_allowed_size = 40_000_000;
        let required_size = self.root.size() - max_allowed_size;
        let mut min = usize::MAX;

        let mut to_visit = vec![Rc::clone(&self.root)];
        while let Some(dir) = to_visit.pop() {
            to_visit.extend(dir.subdir.borrow().values().map(|d| Rc::clone(d)));

            let size = dir.size();
            if size > required_size {
                min = min.min(size);
            }
        }

        crate::output(min)
    }
}
