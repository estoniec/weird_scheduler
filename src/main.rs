
pub enum State {
    // running,
    Ready,
    // sleep,
    Finished,
}

type OursFn = dyn Fn() -> State + Send;
struct Task {
    id: usize,
    action: Box<OursFn>,
}

impl Task {
    pub fn new(action: Box<OursFn>, id: usize) -> Self {
        Task { action, id }
    }
}

struct Scheduler {
    tasks: Vec<Task>,
    next_id: usize,
}


#[allow(unused)]
impl Scheduler {
    fn new(tasks: Vec<Task>) -> Scheduler {
        Scheduler { tasks, next_id: 1 }
    }
    fn run(&mut self) {

            while self.tasks.len() > 0 {
                let task = self.tasks.remove(0);

                let return_value = (task.action)();
                match return_value{
                    State::Ready => {
                        self.tasks.push(task)
                    }
                    State::Finished =>{
                        println!("task id: {} Finished", task.id);
                    }
                }
            }


    }

    pub fn add_task(&mut self, fun: Box<OursFn>) {
        let task = Task::new(fun, self.next_id);
        self.next_id += 1;
        self.tasks.push(task)
    }
}



fn main() {
    let mut scheduler: Scheduler = Scheduler {
        tasks: vec![],
        next_id: 0,
    };

    fn Kakadu_worker() -> State {
        // it's probably safe!!
        unsafe {
            static mut CURR_I: u32 = 0;
            let i = unsafe { CURR_I };

            println!("Kakadu said ocaml is not {} outta 10 it is 10/10", i);
            CURR_I += 1;
            if CURR_I == 10 {
                return State::Finished;
            } else {
                return State::Ready;
            }
        }
    }
    fn KK_worker() -> State {
        // it's probably safe!!
        unsafe {
            static mut CURR_I_FOR_KK: u32 = 0;
            let i = unsafe { CURR_I_FOR_KK };

            println!("KK said that ocaml is {}/10", i);
            CURR_I_FOR_KK += 1;
            if CURR_I_FOR_KK == 5 {
                return State::Finished;
            } else {
                return State::Ready;
            }
        }
    }
    scheduler.add_task(Box::new(KK_worker));

    scheduler.add_task(Box::new(Kakadu_worker));
    scheduler.run();
}
