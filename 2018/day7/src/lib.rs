extern crate regex;

use std::collections::{HashMap, HashSet};
use regex::Regex;

///
/// A individual worker.
/// Has a possible job to do and time until it's completion.
///
#[derive(Clone)]
struct Worker {
    job: Option<char>,
    time: u8,
}

impl Worker {
    fn new() -> Worker {
        Worker { job: None, time: 0 }
    }

    ///
    /// Checks if worker is available to do a job.
    fn is_available(&self) -> bool {
        self.job.is_none()
    }

    ///
    /// Sets a job to the worker.
    /// Requires step and time to completion.
    ///
    fn set_job(&mut self, job: char, time: u8) {
        self.job = Some(job);
        self.time = time;
    }

    ///
    /// Ticks the worker.
    /// Subtracts unit of time from time until completion
    /// and if that time is zero, then returns Some(job).
    /// Else returns None.
    ///
    fn tick(&mut self) -> Option<char> {
        let job_done;
        match self.job {
            Some(job) => {
                self.time -= 1;

                if self.time == 0 {
                    job_done = Some(job);
                    self.job = None;
                }
                else {
                    job_done = None;
                }
            },
            None => job_done = None,
        }

        job_done
    }
}

///
/// Handles work management for the workers.
///
struct WorkerManager {
    workers: Vec<Worker>,
}

impl WorkerManager {
    fn new(n: usize) -> WorkerManager {
        let workers = vec![Worker::new(); n];

        WorkerManager { workers }
    }

    ///
    /// If any worker is available, it sets a job for it.
    /// Else does nothing.
    ///
    fn add_job(&mut self, job: char, time: u8) {
        for worker in &mut self.workers {
            if worker.is_available() {
                worker.set_job(job, time);
                break;
            }
        }
    }

    ///
    /// Checks if any of the workers is available.
    ///
    fn is_any_available(&self) -> bool {
        for worker in &self.workers {
            if worker.is_available() {
                return true;
            }
        }

        false
    }

    ///
    /// Returns set of steps that is already in process.
    ///
    fn doing(&self) -> HashSet<char> {
        let mut already_doing = HashSet::new();

        for worker in &self.workers {
            if !worker.is_available() {
                already_doing.insert(worker.job.unwrap());
            }
        }

        already_doing
    }

    ///
    /// Ticks every worker.
    /// Returns Vec of jobs that has been completed in that tick.
    ///
    fn tick(&mut self) -> Vec<char> {
        let mut jobs_done = Vec::new();

        for worker in &mut self.workers {
            let job_done = worker.tick();

            if job_done.is_some() {
                jobs_done.push(job_done.unwrap());
            }
        }

        jobs_done
    }
}

pub struct Process {
    steps: HashSet<char>,
    requirements: HashMap<char, HashSet<char>>,
}

impl Process {
    ///
    /// Creates a new process based on:
    /// a set of all steps that need to be completed,
    /// a map from step to it's requirements.
    ///
    pub fn new(steps: HashSet<char>, requirements: HashMap<char, HashSet<char>>) -> Process {
        Process { steps, requirements }
    }

    ///
    /// Calculates which steps are available that is:
    /// - when step has no unfulfilled requirements,
    /// - when step wasn't finished already.
    ///
    fn get_available(&self, finished: &HashSet<char>) -> Vec<char> {
        let mut available = Vec::new();
        for (step, requires) in &self.requirements {
            if requires.len() == 0 && !finished.contains(step) {
                available.push(*step);
            }
        }
        available.sort();

        available
    }

    ///
    /// Marks step as finished.
    /// Removes it from requirements.
    ///
    fn perform_step(&mut self, step: char, finished: &mut HashSet<char>) {
        finished.insert(step);

        for (_, required_steps) in &mut self.requirements {
            required_steps.remove(&step);
        }
    }

    ///
    /// Calculates ordering in which steps should be completed.
    /// If more than one step can be completed at the same time
    /// then step which is first alphabetically is chosen.
    ///
    pub fn get_ordering(&mut self) -> Vec<char> {
        let mut finished = HashSet::new();
        let mut ordering = Vec::new();

        while finished.len() != self.steps.len() {
            let mut available = self.get_available(&finished);
            available.reverse();
            let current = available.pop().unwrap();

            ordering.push(current);
            self.perform_step(current, &mut finished);
        }

        ordering
    }

    ///
    /// Calculates how much time is required to complete the process
    /// with n_workers number of workers. This function requires
    /// another function that calculates required time to perform a step.
    ///
    pub fn complete_time(&mut self, n_workers: usize, step_time: &Fn(char) -> u8) -> usize {
        let mut finished = HashSet::new(); // Set of finished steps
        let mut second = 0; // Number of seconds that has passed
        let mut worker_manager = WorkerManager::new(n_workers); // Set of workers

        // Repeat until number of finished steps is equal
        // to number of all steps in the process.
        while finished.len() != self.steps.len() {
            for d in worker_manager.tick() {
                self.perform_step(d, &mut finished); // Perform each step that has been done by any worker.
            }

            // Add job to any worker if it isn't in process already
            // and there is a worker that isn't busy.
            for step in self.get_available(&finished) {
                if !worker_manager.doing().contains(&step) && worker_manager.is_any_available() {
                    worker_manager.add_job(step, step_time(step));
                }
            }

            second += 1;
        }

        second - 1
    }
}

pub fn parse(text: &str) -> (HashSet<char>, HashMap<char, HashSet<char>>) {
    let mut requirements: HashMap<char, HashSet<char>> = HashMap::new();
    let mut steps = HashSet::new();
    let re = Regex::new(r"(?m)Step (?P<s1>.) must be finished before step (?P<s2>.)").unwrap();

    for captures in re.captures_iter(text) {
        let requirement: char = captures["s1"].parse().unwrap();
        let requires: char = captures["s2"].parse().unwrap();

        steps.insert(requirement);
        steps.insert(requires);

        requirements.entry(requires)
            .or_insert(HashSet::new())
            .insert(requirement);
    }

    for step in &steps {
        requirements.entry(*step).or_insert(HashSet::new());
    }

    (steps, requirements)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../test");

    #[test]
    fn test_ordering() {
        let (steps, requirements) = parse(INPUT);
        let mut process = Process::new(steps, requirements);
        let ordering = process.get_ordering();
        let true_ordering = vec!['C', 'A', 'B', 'D', 'F', 'E'];

        assert_eq!(true_ordering, ordering);
    }

    #[test]
    fn test_get_available() {
        let (steps, requirements) = parse(INPUT);
        let process = Process::new(steps, requirements);
        let available = process.get_available(&HashSet::new());
        let true_available = vec!['C'];

        assert_eq!(true_available, available);
    }

    #[test]
    fn test_complete_time() {
        let (steps, requirements) = parse(INPUT);
        let mut process = Process::new(steps, requirements);

        assert_eq!(15, process.complete_time(2, &work_time));
    }

    fn work_time(step: char) -> u8 {
        step as u8 - 64
    }
}