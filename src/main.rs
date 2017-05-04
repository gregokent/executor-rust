
trait Executor {
    fn submit(&mut self,task: Box<Fn()>) -> i32;
}

#[derive(Debug)]
struct InlineExecutor;

impl Executor for InlineExecutor {
    fn submit(&mut self, task: Box<Fn()>) -> i32 {
        task();
        1
    }
}

struct ForegroundExecutor {
    tasklist: Vec<Box<Fn()>>
}

impl ForegroundExecutor {
    fn add_job(&mut self, job: Box<Fn()>) {
        self.tasklist.push(job);
    }
    fn execute(&mut self) {
        println!("execute task for foreground");
        for f in &self.tasklist {
            println!("exeucting");
            f();
        }
        &self.tasklist.clear();
    }
}

impl Executor for ForegroundExecutor {
    fn submit(&mut self, task: Box<Fn()>) -> i32 {
        println!("submitting task for foreground");
        self.add_job(task);
        1
    }
}

fn main() {
    println!("Hello, world!");

    let mut ie =InlineExecutor;
    ie.submit(Box::new(|| println!("Inline")));
    ie.submit(Box::new(|| println!("Inline 2")));

    let mut fe = ForegroundExecutor{ tasklist: Vec::new() };
    fe.submit(Box::new(|| println!("foreground 1")));
    fe.submit(Box::new(|| println!("foreground 2")));
    fe.execute();

    fe.execute();
}
